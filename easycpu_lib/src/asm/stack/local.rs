use crate::compile::{compile_instructions, CompileContext, Instruction};
use crate::asm::mem::{MemInstruction, MemOperation};
use crate::compile::CompileError;
use crate::parser::ParseParts;
use crate::cpu::{self};

use crate::asm::alu::{AluInstruction, AluOperation};
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};

use super::base::{StackBaseInstruction, StackBaseOperation};

#[derive(Copy, Clone, Debug)]
pub enum StackLocalMode {
    VAR,
    ARG,
}

#[derive(Copy, Clone, Debug)]
pub enum StackLocalOperation {
    LOCINIT,
    LOCEND,

    LOAD(StackLocalMode),
    STORE(StackLocalMode),
    ADDR(StackLocalMode),
}

impl StackLocalOperation {
    pub fn parse_operation(s: &str) -> Option<StackLocalOperation> {
        match s {
            "LOCINIT" => Some(StackLocalOperation::LOCINIT),
            "LOCEND" => Some(StackLocalOperation::LOCEND),

            "LVAR" => Some(StackLocalOperation::LOAD(StackLocalMode::VAR)),
            "SVAR" => Some(StackLocalOperation::STORE(StackLocalMode::VAR)),
            "AVAR" => Some(StackLocalOperation::ADDR(StackLocalMode::VAR)),

            "LARG" => Some(StackLocalOperation::LOAD(StackLocalMode::ARG)),
            "SARG" => Some(StackLocalOperation::STORE(StackLocalMode::ARG)),
            "AARG" => Some(StackLocalOperation::ADDR(StackLocalMode::ARG)),

            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StackLocalInstruction {
    pub op: StackLocalOperation,
    pub idx: u16,
}

impl StackLocalInstruction {
    pub fn new(op: StackLocalOperation, idx: u16) -> Result<StackLocalInstruction, CompileError> {
        Ok(StackLocalInstruction { op, idx })
    }

    pub fn parse_asm(
        op: StackLocalOperation,
        mut parts: ParseParts,
    ) -> Result<StackLocalInstruction, CompileError> {
        let idx = match op {
            StackLocalOperation::LOCEND => 0,
            _ => parts.pop_const()?,
        };
        StackLocalInstruction::new(op, idx)
    }

    pub fn load_address(
        mode: StackLocalMode,
        idx: u16,
    ) -> Result<Vec<Box<dyn Instruction>>, CompileError> {
        let shift = match mode {
            StackLocalMode::VAR => idx,
            StackLocalMode::ARG => 0u16.wrapping_sub(4).wrapping_sub(idx),
        };
        Ok(vec![
            Box::new(LoadConstInstruction::new(
                LoadConstOperation::LOAD,
                cpu::Register::R3,
                shift,
            )?),
            Box::new(AluInstruction::new(
                AluOperation::ADD,
                cpu::Register::R3,
                cpu::Register::R3,
                cpu::Register::LP,
            )),
        ])
    }
}

impl Instruction for StackLocalInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let ins: Vec<Box<dyn Instruction>> = match self.op {
            StackLocalOperation::LOCINIT => vec![
                Box::new(StackBaseInstruction::new(
                    StackBaseOperation::PUSH,
                    cpu::Register::LP,
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::MOV,
                    cpu::Register::LP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                )),
                Box::new(LoadConstInstruction::new(
                    LoadConstOperation::ADD,
                    cpu::Register::SP,
                    self.idx,
                )?),
            ],
            StackLocalOperation::LOCEND => vec![
                Box::new(AluInstruction::new(
                    AluOperation::MOV,
                    cpu::Register::SP,
                    cpu::Register::LP,
                    cpu::Register::ZX,
                )),
                Box::new(StackBaseInstruction::new(
                    StackBaseOperation::POP,
                    cpu::Register::LP,
                )?),
            ],
            StackLocalOperation::LOAD(mode) => {
                let mut ins = StackLocalInstruction::load_address(mode, self.idx)?;
                ins.push(Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    cpu::Register::R2,
                    cpu::Register::R3,
                    0,
                )?));
                ins.push(Box::new(StackBaseInstruction::new(
                    StackBaseOperation::PUSH,
                    cpu::Register::R2,
                )?));
                ins
            }
            StackLocalOperation::STORE(mode) => {
                let mut ins = StackLocalInstruction::load_address(mode, self.idx)?;
                ins.push(Box::new(StackBaseInstruction::new(
                    StackBaseOperation::POP,
                    cpu::Register::R2,
                )?));
                ins.push(Box::new(MemInstruction::new(
                    MemOperation::STORE,
                    cpu::Register::R2,
                    cpu::Register::R3,
                    0,
                )?));
                ins
            }
            StackLocalOperation::ADDR(mode) => {
                let mut ins = StackLocalInstruction::load_address(mode, self.idx)?;
                ins.push(Box::new(StackBaseInstruction::new(
                    StackBaseOperation::PUSH,
                    cpu::Register::R3,
                )?));
                ins
            },
        };
        compile_instructions(ins, ctx)
    }
}
