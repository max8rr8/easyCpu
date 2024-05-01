use crate::{
    compile::{comp::CompContext, CompileError},
    stack::{StackOpSignature, StackOperation},
};

#[derive(Clone, Copy, Debug)]
enum ManipStackOperation {
    Swp,
    Dup,
    Drop,
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

    pub fn parse_asm(command_name: &str) -> Option<ManipStackOp> {
        match command_name {
            "SWP" => Some(ManipStackOp::swp()),
            "DUP" => Some(ManipStackOp::dup()),
            "DROP" => Some(ManipStackOp::drop()),
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
        }
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
