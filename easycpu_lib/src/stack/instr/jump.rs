use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::compile::{Atom, CompileContext};
use crate::cpu::{self};
use crate::parser::{ParseParts, ParsedLabel};
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub struct JumpStackOp {
    pub op: JumpOperation,
    pub label_id: usize,
}

impl JumpStackOp {
    pub fn new(op: JumpOperation, label_id: usize) -> Self {
        Self { op, label_id }
    }
}

impl StackOperation for JumpStackOp {
    fn signature(&self) -> StackOpSignature {
        StackOpSignature {
            takes: match self.op {
                JumpOperation::JMP => 0,
                _ => 1,
            },
            flags: StackOpSignature::FLAG_SAVE_STACK | StackOpSignature::FLAG_IMPURE,
            ..Default::default()
        }
    }

    fn execute(
        &self,
        stack: &mut crate::stack::StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        let cond_reg = match self.op {
            JumpOperation::JMP => cpu::Register::ZX,
            _ => stack.inps[0]
        };

        JumpInstruction::instr(comp, self.op, cond_reg, self.label_id)?;

        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}

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

impl Atom for StackJumpInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let label_id = self.targ.resolve(ctx)?;
        ctx.comp.stack(Box::new(JumpStackOp::new(self.op, label_id)));
        Ok(())
    }
}
