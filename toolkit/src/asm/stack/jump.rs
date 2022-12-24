use crate::asm::inst::{compile_instructions, CompileContext};
use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::asm::parse_parts::{ParseParts, ParsedLabel};
use crate::asm::{err::CompileError, inst::Instruction};
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
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let mut ins: Vec<cpu::Instruction> = Vec::new();

        let cond_reg = match self.op {
            JumpOperation::JMP => cpu::Register::ZX,
            _ => {
                ins.extend(StackBaseInstruction::new(
                    StackBaseOperation::POP,
                    cpu::Register::R2,
                )?.compile(ctx)?);
                cpu::Register::R2
            } // (r) => r,
        };

        ins.extend(JumpInstruction::new(
            self.op,
            self.targ.clone(),
            cond_reg,
        ).compile(&CompileContext {
          current_pc: ctx.current_pc + ins.len() as u16,
          label_map: ctx.label_map,
          scope_stack: ctx.scope_stack
        })?);
       Ok(ins)
        
    }
}
