use std::collections::VecDeque;
use std::str::Chars;

use super::inst::{CustomInstruction, CustomMultiInstruction};
use super::{err::CompileError, inst::Instruction};

use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::asm::load_label::LoadLabelInstruction;
use crate::asm::stack;

use crate::asm::alu::{AluInstruction, AluOperation};
use crate::asm::branch::BranchInstruction;
use crate::asm::inst::NopInstruction;
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};
use crate::asm::mem::{MemInstruction, MemOperation};
use crate::cpu;
use crate::parser::parse::{end_checker, letter_checker, nummeric_checker};
use crate::parser::position::PosCompileError;
use crate::parser::{parse_parts, ParseParts};
use crate::parser::{ParsePosition, ParseReader};

fn parse_instruction(s: String) -> Result<Atom, CompileError> {
    let s = s.to_uppercase();
    let mut parts: ParseParts = s.split_whitespace().collect::<VecDeque<&str>>().into();

    let command_raw = parts.pop_command()?;
    let command_raw = match command_raw.split_once('.') {
        Some(res) => res,
        None => (command_raw, ""),
    };
    let command_pure = command_raw.0;
    let command_flags = command_raw.1;

    if command_pure == "NOP" {
        let ins = NopInstruction::new();
        return Ok(Atom::Instruction(Box::new(ins)));
    }

    if let Some(alu_op) = AluOperation::parse_operation(command_pure) {
        let ins = AluInstruction::parse_asm(alu_op, command_flags, parts)?;
        return Ok(Atom::Instruction(Box::new(ins)));
    };

    if let Some(mem_op) = MemOperation::parse_operation(command_pure) {
        let ins = MemInstruction::parse_asm(mem_op, command_flags, parts)?;
        return Ok(Atom::Instruction(Box::new(ins)));
    };

    if command_pure == "BRANCH" {
        let ins = BranchInstruction::parse_asm(command_flags, parts)?;
        return Ok(Atom::Instruction(Box::new(ins)));
    }

    if let Some(op) = LoadConstOperation::parse_operation(command_pure) {
        let ins = LoadConstInstruction::parse_asm(op, parts)?;
        return Ok(Atom::Instruction(Box::new(ins)));
    }

    if command_pure == "HALT" {
        let ins = MemInstruction::new(
            MemOperation::STORE,
            cpu::Register::ZX,
            cpu::Register::ZX,
            -1,
        )?;
        return Ok(Atom::Instruction(Box::new(ins)));
    }

    if command_pure == "LLABEL" {
        let ins = LoadLabelInstruction::parse_asm(parts)?;
        return Ok(Atom::Instruction(Box::new(ins)));
    }

    if let Some(op) = JumpOperation::parse_operation(command_pure) {
        let ins = JumpInstruction::parse_asm(op, parts)?;
        return Ok(Atom::Instruction(Box::new(ins)));
    }

    if let Some(stripped_cmd) = command_pure.strip_prefix('$') {
        return stack::parse_instruction(stripped_cmd, command_flags, parts);
    }

    Err(CompileError::UnknownCommand(String::from(command_pure)))
}

#[derive(Debug)]
pub enum Atom {
    Instruction(Box<dyn Instruction>),
    Label(String),

    EnterLocalScope(usize),
    LeaveLocalScope,

    Nop,
}

#[derive(Debug)]
pub struct FullAtom {
    pub start_pos: ParsePosition,
    pub end_pos: ParsePosition,
    pub compiled: Result<Atom, CompileError>,
}

#[derive(Debug)]
pub struct Program {
    pub lines: Vec<FullAtom>,
}

struct AsmParse<'a> {
    reader: ParseReader<'a>,
    atoms: Vec<FullAtom>,
    current_scope: usize,
}

enum AsmStartToken {
    Letter,
    Comment,
    Number,
    String,
    CurlyBracket,
    Parentheses,
    Skip,
    Unknown,
}

impl<'a> AsmParse<'a> {
    fn peek_start_token(&mut self) -> Result<AsmStartToken, PosCompileError> {
        let cur = self.reader.peek()?;
        if letter_checker(cur) {
            Ok(AsmStartToken::Letter)
        } else if cur == '#' {
            Ok(AsmStartToken::Comment)
        } else if nummeric_checker(cur) {
            Ok(AsmStartToken::Number)
        } else if cur == '"' {
            Ok(AsmStartToken::String)
        } else if cur == '{' || cur == '}' {
            Ok(AsmStartToken::CurlyBracket)
        } else if cur == '(' {
            Ok(AsmStartToken::Parentheses)
        } else if end_checker(cur) | cur.is_whitespace() {
            Ok(AsmStartToken::Skip)
        } else {
            Ok(AsmStartToken::Unknown)
        }
    }

    fn take_parse_block(&mut self) -> Result<Vec<FullAtom>, PosCompileError> {
        let start_pos = self.reader.pos;
        let combined: Vec<char> = self.reader.take_block()?;
        let mut combined = combined.into_iter();
        let mut parser = ParseReader::from(&mut combined);
        parser.pos = start_pos;

        let mut subparser = AsmParse {
            reader: ParseReader::from(&mut combined),
            atoms: Vec::new(),
            current_scope: self.current_scope,
        };

        subparser.parse()?;
        self.current_scope = subparser.current_scope;

        Ok(subparser.atoms)
    }

    fn parse_atom(&mut self) -> Result<(), PosCompileError> {
        let start_pos = self.reader.pos;
        let atom = match self.peek_start_token()? {
            AsmStartToken::Letter => {
                let collected: Vec<char> = self
                    .reader
                    .read_until(|cur, prev| prev == ':' || end_checker(cur))?;
                let collected: String = collected.into_iter().collect();

                Some(if let Some(pure_label) = collected.strip_suffix(':') {
                    Ok(Atom::Label(String::from(pure_label)))
                } else {
                    parse_instruction(collected)
                })
            }
            AsmStartToken::Comment => {
                self.reader.read_until(|cur, _| cur == '\n')?;
                None
            }
            AsmStartToken::Number => {
                let collected: Vec<char> = self
                    .reader
                    .read_until(|cur, _| cur.is_whitespace() || end_checker(cur))?;

                let collected: String = collected.into_iter().collect();
                Some(
                    parse_parts::parse_u16_constant(collected.to_uppercase().as_str()).map(|val| {
                        let ins = Box::new(CustomInstruction::new(val));
                        Atom::Instruction(ins)
                    }),
                )
            }
            AsmStartToken::String => {
                let chars = self.reader.take_block()?;
                let ins = Box::new(CustomMultiInstruction::from(chars));

                Some(Ok(Atom::Instruction(ins)))
            }
            AsmStartToken::CurlyBracket => {
                let mut block = self.take_parse_block()?;
                
                self.current_scope += 1;
                self.atoms.push(FullAtom {
                    start_pos,
                    end_pos: start_pos,
                    compiled: Ok(Atom::EnterLocalScope(self.current_scope))
                });
                self.atoms.append(&mut block);
                self.atoms.push(FullAtom {
                    start_pos: self.reader.pos,
                    end_pos: self.reader.pos,
                    compiled: Ok(Atom::LeaveLocalScope)
                });

                None
            }
            AsmStartToken::Parentheses => {
                let last_el = self.atoms.pop();
                let mut block = self.take_parse_block()?;
                self.atoms.append(&mut block);

                if let Some(last_el) = last_el {
                    self.atoms.push(last_el);
                }

                None
            }
            AsmStartToken::Skip => {
                self.reader.take()?;
                None
            }
            AsmStartToken::Unknown => Some(Err(CompileError::UnknownToken(self.reader.take()?))),
        };

        if let Some(atom) = atom {
            self.atoms.push(FullAtom {
                start_pos,
                end_pos: self.reader.pos,
                compiled: atom
            })
        }


        Ok(())
    }


    pub fn parse(&mut self) -> Result<(), PosCompileError> {
        while !self.reader.is_empty() {
            self.parse_atom()?;
        }

        Ok(())
    }

    pub fn atoms(mut self) -> Result<Vec<FullAtom>, PosCompileError> {
        self.parse()?;
        Ok(self.atoms)
    }
}

impl<'a> From<&'a mut Chars<'a>> for AsmParse<'a> {
    fn from(c: &'a mut Chars<'a>) -> Self {
        let a: AsmParse = AsmParse {
            reader: ParseReader::from(c),
            atoms: Vec::new(),
            current_scope: 0,
        };
        a
    }
}

pub fn parse_listing(inp: &str) -> Result<Vec<FullAtom>, PosCompileError> {
    let mut c = inp.chars();
    let parser = AsmParse::from(&mut c);
    parser.atoms()
}
