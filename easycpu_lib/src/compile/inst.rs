use super::CompileContext;
use crate::compile::CompileError;
use std::fmt;

pub trait Instruction: fmt::Debug {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError>;
}

pub type AtomBox = Box<dyn Instruction>;

pub fn compile_instructions(
    instructions: Vec<Box<dyn Instruction>>,
    ctx: &mut CompileContext,
) -> Result<(), CompileError> {
    instructions
        .into_iter()
        .map(|x| x.compile(ctx))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

#[derive(Debug)]
pub struct ErrorAtom {
    err: CompileError,
}

impl From<CompileError> for ErrorAtom {
    fn from(err: CompileError) -> Self {
        ErrorAtom { err }
    }
}

impl Instruction for ErrorAtom {
    fn compile(&self, _: &mut CompileContext) -> Result<(), CompileError> {
        Err(self.err.clone())
    }
}