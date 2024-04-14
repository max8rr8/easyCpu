use super::CompileContext;
use crate::compile::CompileError;
use std::fmt;

pub trait Atom: fmt::Debug {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError>;
}

pub type AtomBox = Box<dyn Atom>;

pub fn compile_instructions(
    instructions: Vec<Box<dyn Atom>>,
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

impl Atom for ErrorAtom {
    fn compile(&self, _: &mut CompileContext) -> Result<(), CompileError> {
        Err(self.err.clone())
    }
}