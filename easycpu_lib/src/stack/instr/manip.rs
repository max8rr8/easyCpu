use crate::{
    compile::{comp::CompContext, CompileError}, cpu, stack::{StackOpSignature, StackOperation}
};

#[derive(Clone, Copy, Debug)]
enum ManipStackOperation {
    Swp,
    Dup,
    Drop,
    Puzx,
}

#[derive(Copy, Clone, Debug)]
pub struct ManipStackOp {
    op: ManipStackOperation,
}

impl ManipStackOp {
    pub fn swp() -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Swp,
        }
    }

    pub fn dup() -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Dup,
        }
    }

    pub fn drop() -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Drop,
        }
    }

    pub fn puzx() -> ManipStackOp {
      ManipStackOp {
          op: ManipStackOperation::Puzx,
      }
  }

    pub fn parse_asm(command_name: &str) -> Option<ManipStackOp> {
        match command_name {
            "SWP" => Some(ManipStackOp::swp()),
            "DUP" => Some(ManipStackOp::dup()),
            "DROP" => Some(ManipStackOp::drop()),
            "PUZX" => Some(ManipStackOp::puzx()),
            _ => None,
        }
    }
}

impl StackOperation for ManipStackOp {
    fn signature(&self) -> StackOpSignature {
        match self.op {
            ManipStackOperation::Swp => StackOpSignature {
                takes: 2,
                pushes: 2,
                ..Default::default()
            },
            ManipStackOperation::Dup => StackOpSignature {
                takes: 1,
                pushes: 2,
                ..Default::default()
            },
            ManipStackOperation::Drop => StackOpSignature {
                takes: 1,
                pushes: 0,
                ..Default::default()
            },
            ManipStackOperation::Puzx => StackOpSignature {
              pushes: 1,
              ..Default::default()
            },
        }
    }

    fn execute(
        &self,
        stack: &mut crate::stack::StackExecCtx,
        _: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        match self.op {
            ManipStackOperation::Swp => {
                stack.outs[0] = stack.inps[1];
                stack.outs[1] = stack.inps[0];
            }
            ManipStackOperation::Dup => {
                stack.outs[0] = stack.inps[0];
                stack.outs[1] = stack.inps[0];
            }
            ManipStackOperation::Drop => {}
            ManipStackOperation::Puzx => {
              stack.outs[0] = cpu::Register::ZX;
            },
        }
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
