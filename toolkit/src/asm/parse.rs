use std::collections::VecDeque;

use crate::cpu;

use super::inst::{CustomInstruction, Instruction, CustomMultiInstruction};
use super::jump::{JumpOperation, JumpInstruction};
use super::load_label::LoadLabelInstruction;
use super::{parse_parts, stack};
use super::{err::CompileError, parse_parts::ParseParts};

use super::alu::{AluInstruction, AluOperation};
use super::branch::BranchInstruction;
use super::inst::NopInstruction;
use super::load_const::{LoadConstInstruction, LoadConstOperation};
use super::mem::{MemInstruction, MemOperation};

#[derive(Debug)]
pub enum Parsed {
    Instruction(Box<dyn Instruction>),
    Label(String),
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

fn starts_with_numeric(s: char) -> bool {
    match s {
        '0'..='9' => true,
        '+' | '-' => true,
        _ => false,
    }
}

fn parse_instruction(s: &str) -> Result<Parsed, CompileError> {
    if s.starts_with('"') && s.ends_with('"') {
        let mut chars: Vec<char> = s.chars().skip(1).collect();
        chars.pop();
        let chars = chars.iter().map(|c| {
            let mut bu: [u8; 1] = [0];
            c.encode_utf8(&mut bu);
            // dbg!(&c, &bu);
            bu[0] as u16
        });
        let ins = CustomMultiInstruction::new(chars.collect());
        return Ok(Parsed::Instruction(Box::new(ins)));
    }

    let s = s.to_uppercase();

    if s.starts_with(starts_with_numeric) {
        let num = parse_parts::parse_u16_constant(s.as_str())?;
        let ins = CustomInstruction::new(num);
        return Ok(Parsed::Instruction(Box::new(ins)));
    }


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

pub fn parse_listing<'a>(inp: &'a str) -> Vec<ProgramLine> {
    inp.split('\n')
        .enumerate()
        .map(|(line_n, s)| {
            let s = s.trim();
            let s = match s.split_once('#') {
                Some(s) => s.0.trim(),
                None => s,
            };

            // let uncommented =
            let compiled = if s.len() == 0 {
                // Empty line
                Ok(Parsed::Nop)
            } else if s.ends_with(':') {
                // Label
                let mut label = String::from(s);
                label.pop();
                Ok(Parsed::Label(label))
            } else {
                parse_instruction(s)
            };

            ProgramLine {
                line_number: line_n,
                compiled,
            }
        })
        .collect()

    // Program { lines: Vec::new() }
}
