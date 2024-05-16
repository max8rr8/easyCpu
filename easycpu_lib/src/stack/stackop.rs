use std::fmt::Debug;

use crate::{
    compile::{comp::CompContext, context::CompileContext, Atom, CompileError},
    cpu, AsAny,
};

#[derive(Debug, Default)]
pub struct StackOpSignature {
    pub flags: u16,

    pub takes: usize,
    pub pushes: usize,
    pub temps: usize,

    pub peeks: Option<u8>
}

impl StackOpSignature {
    pub const FLAG_SAVE_STACK: u16 = 1 << 0;
    pub const FLAG_RESET_STACK: u16 = 1 << 1;
    pub const FLAG_IMPURE: u16 = 1 << 2;
    pub const FLAG_NOT_CARE_SP: u16 = 1 << 3;

    pub fn check(&self, other: u16) -> bool {
        self.flags & other != 0
    }
}

pub struct StackExecCtx {
    pub inps: Vec<cpu::Register>,
    pub outs: Vec<cpu::Register>,
    pub temps: Vec<cpu::Register>,
    pub peek: Option<cpu::Register>
}

pub trait StackOperation: Debug + AsAny {
    fn signature(&self) -> StackOpSignature;
    fn execute(
        &self,
        stack: &mut StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError>;

    fn duplicate(&self) -> Box<dyn StackOperation>;
}

#[derive(Debug)]
pub struct StackOpInstruction {
    op: Box<dyn StackOperation>,
}

impl StackOpInstruction {
    pub fn new(op: Box<dyn StackOperation>) -> Self {
        StackOpInstruction { op }
    }

    pub fn wrap<T: StackOperation + 'static>(op: T) -> Self {
        StackOpInstruction { op: Box::new(op) }
    }

    pub fn wrap_atombox<T: StackOperation + 'static>(op: T) -> Box<dyn Atom> {
        Box::new(StackOpInstruction { op: Box::new(op) })
    }
}

impl Atom for StackOpInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        ctx.comp.stack(self.op.duplicate());
        Ok(())
    }
}