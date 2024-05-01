use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::cpu::{self};

use crate::asm::alu::AluOperation;

use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub struct AluStackOp {
    pub op: AluOperation,
}

impl AluStackOp {
    pub fn new(op: AluOperation) -> AluStackOp {
        AluStackOp { op }
    }

    pub fn parse_asm(op: AluOperation, _: &str) -> AluStackOp {
        AluStackOp::new(op)
    }
}

impl StackOperation for AluStackOp {
    fn signature(&self) -> StackOpSignature {
        let takes_two = self.op.get_second_reg(cpu::Register::R2).is_none();
        StackOpSignature {
            takes: if takes_two { 2 } else { 1 },
            pushes: 1,
            ..Default::default()
        }
    }

    fn execute(
        &self,
        stack: &mut crate::stack::StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        comp.instruct(self.op.instr(
            stack.outs[0],
            stack.inps[0],
            *stack.inps.get(1).unwrap_or(&cpu::Register::ZX),
        ));
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
