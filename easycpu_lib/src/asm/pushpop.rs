use crate::compile::{compile_instructions, CompileContext, Atom};
use crate::asm::mem::{MemInstruction, MemOperation};
use crate::compile::CompileError;
use crate::parser::ParseParts;
use crate::cpu::{self};

use crate::asm::alu::{AluInstruction, AluOperation};
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};

#[derive(Copy, Clone, Debug)]
pub enum PushPopOperation {
    INIT,
    PUSH,
    POP,
}

impl PushPopOperation {
    pub fn parse_operation(s: &str) -> Option<PushPopOperation> {
        match s {
            "INIT" => Some(PushPopOperation::INIT),
            "PUSH" => Some(PushPopOperation::PUSH),
            "POP" => Some(PushPopOperation::POP),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PushPopInstruction {
    pub op: PushPopOperation,
    pub reg: cpu::Register,
}

impl PushPopInstruction {
    pub fn new(
        op: PushPopOperation,
        reg: cpu::Register,
    ) -> Result<PushPopInstruction, CompileError> {
        Ok(PushPopInstruction { op, reg })
    }

    pub fn parse_asm(
        op: PushPopOperation,
        mut parts: ParseParts,
    ) -> Result<PushPopInstruction, CompileError> {
        let reg = match op {
            PushPopOperation::INIT => cpu::Register::ZX,
            _ => parts.pop_register()?,
        };

        PushPopInstruction::new(op, reg)
    }
}

impl Atom for PushPopInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let ins: Vec<Box<dyn Atom>> = match self.op {
            PushPopOperation::INIT => vec![
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
            PushPopOperation::PUSH => vec![
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
            PushPopOperation::POP => vec![
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
        };
        compile_instructions(ins, ctx)
    }
}
