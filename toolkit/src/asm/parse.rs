use std::collections::VecDeque;
use std::str::Chars;

use crate::cpu;

use super::err::CompileError;
use super::inst::{CustomInstruction, CustomMultiInstruction, Instruction};
use super::jump::{JumpInstruction, JumpOperation};
use super::load_label::LoadLabelInstruction;
use super::parse_parts::ParseParts;
use super::{parse_parts, stack};

use super::alu::{AluInstruction, AluOperation};
use super::branch::BranchInstruction;
use super::inst::NopInstruction;
use super::load_const::{LoadConstInstruction, LoadConstOperation};
use super::mem::{MemInstruction, MemOperation};

#[derive(Debug)]
pub enum Parsed {
    Instruction(Box<dyn Instruction>),
    Label(String),

    EnterLocalScope(usize),
    LeaveLocalScope,

    Nop,
}

#[derive(Debug)]
pub struct ProgramLine {
    pub line_number: usize,
    pub compiled: Result<Parsed, CompileError>,
}

#[derive(Debug)]
pub struct Program {
    pub lines: Vec<ProgramLine>,
}

fn parse_instruction(s: String) -> Result<Parsed, CompileError> {
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
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    if let Some(alu_op) = AluOperation::parse_operation(command_pure) {
        let ins = AluInstruction::parse_asm(alu_op, command_flags, parts)?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    };

    if let Some(mem_op) = MemOperation::parse_operation(command_pure) {
        let ins = MemInstruction::parse_asm(mem_op, command_flags, parts)?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    };

    if command_pure == "BRANCH" {
        let ins = BranchInstruction::parse_asm(command_flags, parts)?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    if let Some(op) = LoadConstOperation::parse_operation(command_pure) {
        let ins = LoadConstInstruction::parse_asm(op, parts)?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    if command_pure == "HALT" {
        let ins = MemInstruction::new(
            MemOperation::STORE,
            cpu::Register::ZX,
            cpu::Register::ZX,
            -1,
        )?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    if command_pure == "LLABEL" {
        let ins = LoadLabelInstruction::parse_asm(parts)?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    if let Some(op) = JumpOperation::parse_operation(command_pure) {
        let ins = JumpInstruction::parse_asm(op, parts)?;
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    if let Some(stripped_cmd) = command_pure.strip_prefix("$") {
        return stack::parse_instruction(stripped_cmd, command_flags, parts);
    }

    Err(CompileError::UnknownCommand(String::from(command_pure)))
}

fn nummeric_checker(s: char) -> bool {
    match s {
        '0'..='9' => true,
        '+' | '-' => true,
        _ => false,
    }
}

fn letter_checker(s: char) -> bool {
    match s {
        'a'..='z' => true,
        'A'..='Z' => true,
        '$' => true,
        _ => false,
    }
}

fn end_checker(s: char) -> bool {
    match s {
        ';' | '#' | '\n' | '(' => true,
        _ => false,
    }
}

struct ParseReader<'a> {
    inp: &'a mut dyn Iterator<Item = char>,
    more: bool,
    cur_char: char,
    cur_line: usize,
}

impl<'a> ParseReader<'a> {
    fn front(&self) -> Result<char, CompileError> {
        if !self.more {
            Err(CompileError::UnexpectedEndOfFile)
        } else {
            Ok(self.cur_char)
        }
    }

    fn pop(&mut self) -> Option<char> {
        let c = self.inp.next();
        match c {
            None => {
                self.more = false;
                None
            }
            Some(c) => {
                if c == '\n' {
                    self.cur_line += 1;
                }

                self.cur_char = c;
                Some(c)
            }
        }
    }

    fn read_until(&mut self, until: fn(char, char) -> bool) -> Result<Vec<char>, CompileError> {
        let mut collected: Vec<char> = Vec::new();
        let mut prev: char = '\0';

        while self.more {
            let cur = self.front()?;

            if until(cur, prev) {
                break;
            }

            prev = cur;

            collected.push(cur);
            self.pop();
        }

        return Ok(collected);
    }

    fn parse(mut self) -> Result<Vec<ProgramLine>, CompileError> {
        let mut parsed: Vec<ProgramLine> = Vec::new();

        let mut current_scope: usize = 0;
    
        while self.more {
            let mut cur = self.front()?;
    
            if letter_checker(cur) {
                // Handle instructions and labels
    
                let collected: Vec<char> =
                    self.read_until(|cur, prev| prev == ':' || end_checker(cur))?;
                let collected: String = collected.into_iter().collect();
    
                parsed.push(ProgramLine {
                    line_number: self.cur_line,
                    compiled: if let Some(pure_label) = collected.strip_suffix(':') {
                        Ok(Parsed::Label(String::from(pure_label)))
                    } else {
                        parse_instruction(collected)
                    },
                });
            } else if cur == '#' {
                // Handle Comments
                self.read_until(|cur, _| cur == '\n')?;
            } else if nummeric_checker(cur) {
                // Handle numbers
                let collected: Vec<char> =
                    self.read_until(|cur, _| cur.is_whitespace() || end_checker(cur))?;
    
                let collected: String = collected.into_iter().collect();
                let compiled =
                    parse_parts::parse_u16_constant(collected.to_uppercase().as_str()).map(|val| {
                        let ins = Box::new(CustomInstruction::new(val));
                        Parsed::Instruction(ins)
                    });
    
                parsed.push(ProgramLine {
                    line_number: self.cur_line,
                    compiled,
                });
            } else if cur == '"' {
                // Handle strings
    
                let mut collected: Vec<u16> = Vec::new();
                let mut is_special = false;
    
                self.pop();
                while self.more {
                    cur = self.front()?;
    
                    if is_special {
                        cur = match cur {
                            'n' => '\n',
                            't' => '\t',
                            '0' => '\0',
                            _ => cur,
                        };
    
                        is_special = false;
                    } else {
                        if cur == '\\' {
                            is_special = true;
                        } else if cur == '"' {
                            self.pop();
                            break;
                        }
                    }
    
                    if !is_special {
                        let mut bu: [u8; 1] = [0];
                        cur.encode_utf8(&mut bu);
                        collected.push(bu[0] as u16);
                    }
    
                    self.pop();
                }
    
                let ins = Box::new(CustomMultiInstruction::new(collected));
    
                parsed.push(ProgramLine {
                    line_number: self.cur_line,
                    compiled: Ok(Parsed::Instruction(ins)),
                });
            } else if cur == '{' || cur == '}' {
                parsed.push(ProgramLine {
                    line_number: self.cur_line,
                    compiled: if cur == '{' {
                        current_scope += 1;
                        Ok(Parsed::EnterLocalScope(current_scope))
                    } else {
                        Ok(Parsed::LeaveLocalScope)
                    },
                });
                self.pop();
            } else if cur == '(' {
                let start_line = self.cur_line;
                let last_el = parsed.pop();
                
                let mut combined: Vec<char> = Vec::new();
                let mut cur_level = 0;

                self.pop();
                while self.more {
                    cur = self.front()?;
                    if cur == '(' {
                        cur_level += 1;
                    } else if cur == ')' {
                        cur_level -= 1;
                    }
                    self.pop();

                    if cur_level < 0 {
                        break;
                    }
                    combined.push(cur);
                }

                let mut combined = combined.into_iter();
                let mut subparser = ParseReader {
                    inp: &mut combined,
                    more: true,
                    cur_char: '\0',
                    cur_line: start_line,
                };

                subparser.pop();
                let mut subparsed = subparser.parse()?;
                parsed.append(&mut subparsed);

                if let Some(last_el) = last_el {
                    parsed.push(last_el);
                }

            } else if end_checker(cur) | cur.is_whitespace() {
                // Handle empty lines and whitespaces
                self.pop();
            } else {
                // Handle numbers
    
                self.pop();
                parsed.push(ProgramLine {
                    line_number: self.cur_line,
                    compiled: Err(CompileError::UnknownToken(cur)),
                });
            }
        }
    
        Ok(parsed)
    }
}

impl<'a> From<&'a mut Chars<'a>> for ParseReader<'a> {
    fn from(c: &'a mut Chars<'a>) -> Self {
        let mut p: ParseReader = ParseReader {
            inp: c,
            more: true,
            cur_char: '\0',
            cur_line: 0,
        };
        p.pop(); 
        p
    }
}

pub fn parse_listing<'a>(inp: &'a str) -> Result<Vec<ProgramLine>, CompileError> {
    let mut c = inp.chars();
    let parser = ParseReader::from(&mut c);
    parser.parse()
}
