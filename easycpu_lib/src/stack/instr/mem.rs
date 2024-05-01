use crate::asm::alu::AluOperation;
use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::cpu;

use crate::asm::mem::MemOperation;
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub struct MemStackOp {
    pub op: MemOperation,
}

impl MemStackOp {
    pub fn new(op: MemOperation) -> Self {
        Self { op }
    }

    pub fn parse_asm(op: MemOperation) -> Self {
        Self::new(op)
    }

    fn match_takes_pushes(&self) -> (usize, usize) {
        match self.op {
            MemOperation::LOAD => (1, 1),
            MemOperation::LADD | MemOperation::LSUB => (2, 1),
            MemOperation::STORE => (2, 0),
        }
    }
}


impl StackOperation for MemStackOp {
  fn signature(&self) -> StackOpSignature {
      let (takes, pushes) = self.match_takes_pushes();
      StackOpSignature {
          takes, pushes,
          flags: if matches!(self.op, MemOperation::STORE) {
            StackOpSignature::FLAG_IMPURE
          } else {
            Default::default()
          },
          ..Default::default()
      }
  }

  fn execute(
      &self,
      stack: &mut crate::stack::StackExecCtx,
      comp: &mut dyn CompContext,
  ) -> Result<(), CompileError> {
      let (addr_reg, val_reg) = match self.op {
        MemOperation::LOAD => (stack.inps[0], stack.outs[0]),
        MemOperation::STORE => (stack.inps[1], stack.inps[0]),

        MemOperation::LADD | MemOperation::LSUB => {
          comp.instruct(AluOperation::MOV.instr(
            stack.outs[0],
            stack.inps[0],
            cpu::Register::ZX,
          ));

          (stack.inps[1], stack.outs[0])
        },
      };

      comp.instruct(self.op.instr(val_reg, addr_reg, 0)?);

      Ok(())
  }

  fn duplicate(&self) -> Box<dyn StackOperation> {
      Box::new(*self)
  }
}
