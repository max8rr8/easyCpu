use crate::{
    compile::{comp::CompContext, CompileError},
    cpu,
    parser::ParseParts,
    stack::{StackOpSignature, StackOperation},
};

#[derive(Clone, Copy, Debug)]
pub enum ManipStackOperation {
    Swp,
    Peek(u8),
    Drop(u8),
    Puzx,
}

#[derive(Copy, Clone, Debug)]
pub struct ManipStackOp {
    pub op: ManipStackOperation,
}

impl ManipStackOp {
    pub fn swp() -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Swp,
        }
    }

    pub fn dup() -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Peek(1),
        }
    }

    pub fn peek(dist: u8) -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Peek(dist),
        }
    }

    pub fn drop(count: u8) -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Drop(count),
        }
    }

    pub fn puzx() -> ManipStackOp {
        ManipStackOp {
            op: ManipStackOperation::Puzx,
        }
    }

    pub fn parse_asm(
        command_name: &str,
        parts: &mut ParseParts,
    ) -> Result<Option<ManipStackOp>, CompileError> {
        Ok(match command_name {
            "SWP" => Some(ManipStackOp::swp()),
            "DUP" => Some(ManipStackOp::dup()),

            "PEEK" => {
                let count = parts.pop_const()?;
                let count: u8 = count
                    .try_into()
                    .map_err(|_| CompileError::ShiftIsTooBig(0))?;

                if count == 0 {
                    return Err(CompileError::ShiftIsTooBig(0));
                }

                Some(ManipStackOp::peek(count))
            }

            "DROP" => {
                let count = parts.pop_const().unwrap_or(1);
                let count: u8 = count
                    .try_into()
                    .map_err(|_| CompileError::ShiftIsTooBig(0))?;

                if count == 0 {
                    return Err(CompileError::ShiftIsTooBig(0));
                }

                Some(ManipStackOp::drop(count))
            }
            "PUZX" => Some(ManipStackOp::puzx()),
            _ => None,
        })
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
            ManipStackOperation::Peek(dist) => StackOpSignature {
                peeks: Some(dist),
                pushes: 1,
                ..Default::default()
            },
            ManipStackOperation::Drop(count) => StackOpSignature {
                takes: count as usize,
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
            ManipStackOperation::Peek(_) => {
                stack.outs[0] = stack.peek.unwrap();
            }
            ManipStackOperation::Drop(_) => {}
            ManipStackOperation::Puzx => {
                stack.outs[0] = cpu::Register::ZX;
            }
        }
        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
