use crate::asm::mem::MemOperation;
use crate::compile::CompileError;
use crate::cpu::{self};
use crate::parser::ParseParts;

use crate::asm::alu::AluOperation;
use crate::asm::load_const::LoadConstInstruction;
use crate::stack::instr::local::{LocalOperation, LocalStackOp};
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FunctionOperation {
    INIT,
    RETURN,
}

impl FunctionOperation {
    pub fn parse_operation(s: &str) -> Option<FunctionOperation> {
        match s {
            "FUNC" => Some(FunctionOperation::INIT),
            "RET" => Some(FunctionOperation::RETURN),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FunctionStackOp {
    pub op: FunctionOperation,
    pub scope: LocalStackOp,
    pub args: u16,
    pub returned: u16,
}

impl FunctionStackOp {
    pub fn new_init(
        locals: u16,
        args: u16,
        returned: u16,
    ) -> Result<FunctionStackOp, CompileError> {
        Ok(FunctionStackOp {
            op: FunctionOperation::INIT,
            scope: LocalStackOp::new(LocalOperation::LOCINIT, locals)?,
            args,
            returned,
        })
    }
    pub fn new_return() -> Result<FunctionStackOp, CompileError> {
        Ok(FunctionStackOp {
            op: FunctionOperation::RETURN,
            scope: LocalStackOp::new(LocalOperation::LOCEND, 0)?,
            args: 0,
            returned: 0,
        })
    }

    pub fn parse_asm(
        op: FunctionOperation,
        mut parts: ParseParts,
    ) -> Result<FunctionStackOp, CompileError> {
        match op {
            FunctionOperation::INIT => FunctionStackOp::new_init(
                parts.pop_const()?,
                parts.pop_const()?,
                parts.pop_const()?,
            ),
            FunctionOperation::RETURN => FunctionStackOp::new_return(),
        }
    }
}

impl StackOperation for FunctionStackOp {
    fn signature(&self) -> StackOpSignature {
        let mut flags = StackOpSignature::FLAG_SAVE_STACK | StackOpSignature::FLAG_IMPURE;
        
        flags |= match self.op {
            FunctionOperation::INIT => StackOpSignature::FLAG_RESET_STACK,
            FunctionOperation::RETURN => StackOpSignature::FLAG_NOT_CARE_SP,
        };

        StackOpSignature {
            flags,
            ..Default::default()
        }
    }

    fn execute(
        &self,
        stack: &mut crate::stack::StackExecCtx,
        comp: &mut dyn crate::compile::comp::CompContext,
    ) -> Result<(), CompileError> {
        match self.op {
            FunctionOperation::INIT => {
                LoadConstInstruction::instr_load(
                    cpu::Register::R2,
                    0u16.wrapping_sub(self.args + 1).wrapping_add(self.returned),
                )
                .into_iter()
                .for_each(|i| comp.instruct(i));

                comp.instruct(AluOperation::ADD.instr(
                    cpu::Register::R2,
                    cpu::Register::R2,
                    cpu::Register::SP,
                ));

                comp.instruct(MemOperation::STORE.instr(
                    cpu::Register::R2,
                    cpu::Register::SP,
                    0,
                )?);

                comp.instruct(AluOperation::INC.instr(
                    cpu::Register::SP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                ));

                self.scope.execute(stack, comp)?;
            }
            FunctionOperation::RETURN => {
                self.scope.execute(stack, comp)?;

                comp.instruct(MemOperation::LOAD.instr(
                    cpu::Register::R2,
                    cpu::Register::SP,
                    -2,
                )?);
                comp.instruct(MemOperation::LOAD.instr(
                    cpu::Register::SP,
                    cpu::Register::SP,
                    -1,
                )?);
                comp.instruct(AluOperation::MOV.instr(
                    cpu::Register::PC,
                    cpu::Register::R2,
                    cpu::Register::ZX,
                ));
            }
        };

        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
