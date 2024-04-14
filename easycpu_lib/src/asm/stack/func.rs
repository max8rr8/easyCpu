use crate::compile::{compile_instructions, CompileContext, Atom};
use crate::asm::mem::{MemInstruction, MemOperation};
use crate::compile::CompileError;
use crate::parser::ParseParts;
use crate::cpu::{self};

use crate::asm::alu::{AluInstruction, AluOperation};
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};

use super::base::{StackBaseInstruction, StackBaseOperation};
use super::local::{StackLocalInstruction, StackLocalOperation};

#[derive(Copy, Clone, Debug)]
pub enum StackFunctionOperation {
    INIT,
    RETURN,
}

impl StackFunctionOperation {
    pub fn parse_operation(s: &str) -> Option<StackFunctionOperation> {
        match s {
            "FUNC" => Some(StackFunctionOperation::INIT),
            "RET" => Some(StackFunctionOperation::RETURN),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct StackFunctionInstruction {
    pub op: StackFunctionOperation,
    pub locals: u16,
    pub args: u16,
    pub returned: u16,
}

impl StackFunctionInstruction {
    pub fn new_init(locals: u16, args: u16, returned: u16) -> Result<StackFunctionInstruction, CompileError> {
        Ok(StackFunctionInstruction {
            op: StackFunctionOperation::INIT,
            locals,
            args,
            returned
        })
    }
    pub fn new_return() -> Result<StackFunctionInstruction, CompileError> {
        Ok(StackFunctionInstruction {
            op: StackFunctionOperation::RETURN,
            locals: 0,
            args: 0,
            returned:0
        })
    }

    pub fn parse_asm(
        op: StackFunctionOperation,
        mut parts: ParseParts,
    ) -> Result<StackFunctionInstruction, CompileError> {
        match op {
            StackFunctionOperation::INIT => {
                StackFunctionInstruction::new_init(parts.pop_const()?, parts.pop_const()?, parts.pop_const()?)
            }
            StackFunctionOperation::RETURN => StackFunctionInstruction::new_return(),
        }
        // StackFunctionInstruction::new(parts.pop_const(), parts.pop_const())
    }
}

impl Atom for StackFunctionInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let ins: Vec<Box<dyn Atom>> = match self.op {
            StackFunctionOperation::INIT => vec![
                Box::new(LoadConstInstruction::new(
                    LoadConstOperation::LOAD,
                    cpu::Register::R2,
                    0u16.wrapping_sub(self.args + 1).wrapping_add(self.returned),
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::ADD,
                    cpu::Register::R2,
                    cpu::Register::R2,
                    cpu::Register::SP,
                )),
                Box::new(StackBaseInstruction::new(
                    StackBaseOperation::PUSH,
                    cpu::Register::R2,
                )?),
                Box::new(StackLocalInstruction::new(
                    StackLocalOperation::LOCINIT,
                    self.locals
                )?),
            ],
            StackFunctionOperation::RETURN =>  vec![
                Box::new(StackLocalInstruction::new(
                    StackLocalOperation::LOCEND,
                    0
                )?),
                Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    cpu::Register::R2,
                    cpu::Register::SP,
                    -2
                )?),
                Box::new(MemInstruction::new(
                    MemOperation::LOAD,
                    cpu::Register::SP,
                    cpu::Register::SP,
                    -1
                )?),
                Box::new(AluInstruction::new(
                    AluOperation::MOV,
                    cpu::Register::PC,
                    cpu::Register::R2,
                    cpu::Register::ZX,
                )),
            ],
        };

        compile_instructions(ins, ctx)
    }
}
