use crate::compile::{CompileContext, Instruction};
use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::parser::{ParseParts, ParsedLabel};
use crate::compile::CompileError;
use crate::cpu::{self};

use super::base::{StackBaseInstruction, StackBaseOperation};

#[derive(Clone, Debug)]
pub struct StackJumpInstruction {
    pub op: JumpOperation,
    pub targ: ParsedLabel,
}

impl StackJumpInstruction {
    pub fn new(op: JumpOperation, targ: ParsedLabel) -> StackJumpInstruction {
        StackJumpInstruction { op, targ }
    }

    pub fn parse_asm(
        op: JumpOperation,
        mut parts: ParseParts,
    ) -> Result<StackJumpInstruction, CompileError> {
        Ok(StackJumpInstruction::new(op, parts.pop_label()?))
    }
}

impl Instruction for StackJumpInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let cond_reg = match self.op {
            JumpOperation::JMP => cpu::Register::ZX,
            _ => {
                StackBaseInstruction::new(
                    StackBaseOperation::POP,
                    cpu::Register::R2,
                )?.compile(ctx)?;
                cpu::Register::R2
            } // (r) => r,
        };

        JumpInstruction::new(
            self.op,
            self.targ.clone(),
            cond_reg,
        ).compile(ctx)?;
       Ok(())
        
    }
}
