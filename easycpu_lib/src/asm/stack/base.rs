use crate::compile::{compile_instructions, CompileContext, Instruction};
use crate::asm::mem::{MemInstruction, MemOperation};
use crate::compile::CompileError;
use crate::parser::ParseParts;
use crate::cpu::{self};

use crate::asm::alu::{AluInstruction, AluOperation};
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};

#[derive(Copy, Clone, Debug)]
pub enum StackBaseOperation {
    INIT,
    PUSH,
    POP,
    DUP,
    SWP,
}

impl StackBaseOperation {
    pub fn parse_operation(s: &str) -> Option<StackBaseOperation> {
        match s {
            "INIT" => Some(StackBaseOperation::INIT),
            "PUSH" => Some(StackBaseOperation::PUSH),
            "POP" => Some(StackBaseOperation::POP),
            "DUP" => Some(StackBaseOperation::DUP),
            "SWP" => Some(StackBaseOperation::SWP),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StackBaseInstruction {
    pub op: StackBaseOperation,
    pub reg: cpu::Register,
}

impl StackBaseInstruction {
    pub fn new(
        op: StackBaseOperation,
        reg: cpu::Register,
    ) -> Result<StackBaseInstruction, CompileError> {
        Ok(StackBaseInstruction { op, reg })
    }

    pub fn parse_asm(
        op: StackBaseOperation,
        mut parts: ParseParts,
    ) -> Result<StackBaseInstruction, CompileError> {
        let reg = match op {
            StackBaseOperation::INIT | StackBaseOperation::DUP | StackBaseOperation::SWP => {
                cpu::Register::ZX
            }
            _ => parts.pop_register()?,
        };

        StackBaseInstruction::new(op, reg)
    }
}

impl Instruction for StackBaseInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let ins: Vec<Box<dyn Instruction>> = match self.op {
            StackBaseOperation::INIT => vec![
                Box::new(LoadConstInstruction::new(
                    LoadConstOperation::LOAD,
                    cpu::Register::SP,
                    0x4000,
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::MOV,
                    cpu::Register::LP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                )),
            ],
            StackBaseOperation::PUSH => vec![
                Box::new(MemInstruction::new(
                    MemOperation::STORE,
                    self.reg,
                    cpu::Register::SP,
                    0,
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::INC,
                    cpu::Register::SP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                )),
            ],
            StackBaseOperation::POP => vec![
                Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    self.reg,
                    cpu::Register::SP,
                    -1,
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::DEC,
                    cpu::Register::SP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                )),
            ],

            StackBaseOperation::DUP => vec![
                Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    cpu::Register::R2,
                    cpu::Register::SP,
                    -1,
                )?),
                Box::new(MemInstruction::new(
                    MemOperation::STORE,
                    cpu::Register::R2,
                    cpu::Register::SP,
                    0,
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::INC,
                    cpu::Register::SP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                )),
            ],

            StackBaseOperation::SWP => vec![
                Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    cpu::Register::R2,
                    cpu::Register::SP,
                    -1,
                )?),
                Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    cpu::Register::R3,
                    cpu::Register::SP,
                    -2,
                )?),
                Box::new(MemInstruction::new(
                    MemOperation::STORE,
                    cpu::Register::R3,
                    cpu::Register::SP,
                    -1,
                )?),
                Box::new(MemInstruction::new(
                    MemOperation::STORE,
                    cpu::Register::R2,
                    cpu::Register::SP,
                    -2,
                )?),
            ],
        };
        compile_instructions(ins, ctx)
    }
}
