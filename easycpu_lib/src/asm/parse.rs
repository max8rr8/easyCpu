use std::collections::VecDeque;
use std::str::Chars;

use super::custom::{CustomInstruction, CustomMultiInstruction, NopInstruction};
use super::pushpop::{PushPopInstruction, PushPopOperation};
use crate::compile::label::LabelScope;
use crate::compile::{AtomBox, CompileError, ErrorAtom, Label};
use crate::stack;

use crate::asm::alu::{AluInstruction, AluOperation};
use crate::asm::branch::BranchInstruction;
use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};
use crate::asm::load_label::LoadLabelInstruction;
use crate::asm::mem::{MemInstruction, MemOperation};
use crate::cpu;
use crate::parser::parse::{end_checker, letter_checker, nummeric_checker};
use crate::parser::position::{PosCompileError, PositionAtom};
use crate::parser::{parse_parts, ParseParts, ParseReader};
use crate::stack::StackOptAtom;

fn parse_instruction(s: String) -> Result<AtomBox, CompileError> {
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
        return Ok(Box::new(ins));
    }

    if let Some(alu_op) = AluOperation::parse_operation(command_pure) {
        let ins = AluInstruction::parse_asm(alu_op, command_flags, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(mem_op) = MemOperation::parse_operation(command_pure) {
        let ins = MemInstruction::parse_asm(mem_op, command_flags, parts)?;
        return Ok(Box::new(ins));
    };

    if command_pure == "BRANCH" {
        let ins = BranchInstruction::parse_asm(command_flags, parts)?;
        return Ok(Box::new(ins));
    }

    if let Some(op) = LoadConstOperation::parse_operation(command_pure) {
        let ins = LoadConstInstruction::parse_asm(op, parts)?;
        return Ok(Box::new(ins));
    }

    if command_pure == "HALT" {
        let ins = MemInstruction::new(
            MemOperation::STORE,
            cpu::Register::ZX,
            cpu::Register::ZX,
            -1,
        )?;
        return Ok(Box::new(ins));
    }

    if command_pure == "LLABEL" {
        let ins = LoadLabelInstruction::parse_asm(parts)?;
        return Ok(Box::new(ins));
    }

    if let Some(op) = JumpOperation::parse_operation(command_pure) {
        let ins = JumpInstruction::parse_asm(op, parts)?;
        return Ok(Box::new(ins));
    }

    if let Some(stripped_cmd) = command_pure.strip_prefix('$') {
        if let Some(base_op) = PushPopOperation::parse_operation(stripped_cmd) {
            let ins = PushPopInstruction::parse_asm(base_op, parts)?;
            return Ok(Box::new(ins));
        };

        return stack::instr::parse_instruction(stripped_cmd, command_flags, parts);
    }

    Err(CompileError::UnknownCommand(String::from(command_pure)))
}

enum Modifier {
    Scope,
    StackOpt,
}

impl Modifier {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "stackopt" => Some(Modifier::StackOpt),
            _ => None,
        }
    }
}

struct AsmParse<'a> {
    reader: ParseReader<'a>,
    atoms: Vec<AtomBox>,
    modifier: Modifier,
}

enum AsmStartToken {
    Letter,
    Comment,
    Number,
    String,
    CurlyBracket,
    Parentheses,
    Skip,
    Modifier,
    Unknown,
}

impl<'a> AsmParse<'a> {
    fn peek_start_token(&mut self) -> Result<AsmStartToken, PosCompileError> {
        let cur = self.reader.peek()?;
        if letter_checker(cur) {
            Ok(AsmStartToken::Letter)
        } else if cur == '#' {
            Ok(AsmStartToken::Comment)
        } else if cur == '@' {
            Ok(AsmStartToken::Modifier)
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

    fn take_parse_block(&mut self) -> Result<Vec<AtomBox>, PosCompileError> {
        let start_pos = self.reader.pos;
        let combined: Vec<char> = self.reader.take_block()?;
        let mut combined = combined.into_iter();
        let mut parser = ParseReader::from(&mut combined);
        parser.pos = start_pos;

        let mut subparser = AsmParse {
            reader: parser,
            atoms: Vec::new(),
            modifier: Modifier::Scope,
        };

        subparser.parse()?;
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
                    Ok(Box::new(Label::new(pure_label.to_owned())) as AtomBox)
                } else {
                    parse_instruction(collected)
                })
            }

            AsmStartToken::Comment => {
                self.reader.read_until(|cur, _| cur == '\n')?;
                None
            }

            AsmStartToken::Modifier => {
                self.reader.take()?;
                let modi = self
                    .reader
                    .read_until(|cur, _| cur.is_whitespace() || end_checker(cur) || cur == '{')?;
                let modi: String = modi.into_iter().collect();
                let modi = modi.to_lowercase();

                if let Some(modifier) = Modifier::from_name(&modi) {
                    self.modifier = modifier;
                    None
                } else {
                    Some(Err(CompileError::UnknownModifier(modi)))
                }
            }

            AsmStartToken::Number => {
                let collected: Vec<char> = self
                    .reader
                    .read_until(|cur, _| cur.is_whitespace() || end_checker(cur))?;

                let collected: String = collected.into_iter().collect();
                Some(
                    parse_parts::parse_u16_constant(collected.to_uppercase().as_str())
                        .map(|val| Box::new(CustomInstruction::new(val)) as AtomBox),
                )
            }

            AsmStartToken::String => {
                let chars = self.reader.take_block()?;
                let ins = Box::new(CustomMultiInstruction::from(chars));

                Some(Ok(ins as AtomBox))
            }

            AsmStartToken::CurlyBracket => {
                let block = self.take_parse_block()?;
                let atom: AtomBox = match self.modifier {
                    Modifier::Scope => Box::new(LabelScope::new(block)),
                    Modifier::StackOpt => Box::new(StackOptAtom::new(block)),
                };
                self.modifier = Modifier::Scope;
                Some(Ok(atom))
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
            let atom = match atom {
                Ok(atom) => atom,
                Err(err) => Box::new(ErrorAtom::from(err)),
            };

            self.atoms.push(Box::new(PositionAtom::new(
                atom,
                start_pos,
                self.reader.pos,
            )))
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), PosCompileError> {
        while !self.reader.is_empty() {
            self.parse_atom()?;
        }

        Ok(())
    }

    pub fn atoms(mut self) -> Result<Vec<AtomBox>, PosCompileError> {
        self.parse()?;
        Ok(self.atoms)
    }
}

impl<'a> From<&'a mut Chars<'a>> for AsmParse<'a> {
    fn from(c: &'a mut Chars<'a>) -> Self {
        let a: AsmParse = AsmParse {
            reader: ParseReader::from(c),
            atoms: Vec::new(),
            modifier: Modifier::Scope,
        };
        a
    }
}

pub fn parse_listing(inp: &str) -> Result<Vec<AtomBox>, PosCompileError> {
    let mut c = inp.chars();
    let parser = AsmParse::from(&mut c);
    parser.atoms()
}
