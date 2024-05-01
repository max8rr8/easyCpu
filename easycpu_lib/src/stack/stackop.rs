use std::{fmt::Debug, vec};

use crate::{
    asm::{alu::AluOperation, mem::MemOperation},
    compile::{comp::CompContext, context::CompileContext, Atom, CompileError},
    cpu,
};

#[derive(Debug, Default)]
pub struct StackOpSignature {
    pub flags: u16,

    pub takes: usize,
    pub pushes: usize,
    pub temps: usize,
}

impl StackOpSignature {
    pub const FLAG_SAVE_STACK: u16 = 1 << 0;
    pub const FLAG_RESET_STACK: u16 = 1 << 1;
    pub const FLAG_IMPURE: u16 = 1 << 2;

    pub fn check(&self, other: u16) -> bool {
        self.flags & other != 0
    }
}

pub struct StackExecCtx {
    pub inps: Vec<cpu::Register>,
    pub outs: Vec<cpu::Register>,
    pub temps: Vec<cpu::Register>,
}

pub trait StackOperation: Debug {
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

fn apply_stack_shift(comp: &mut dyn CompContext, stack_shift: &mut i8) {
    while *stack_shift < 0 {
        *stack_shift += 1;
        comp.instruct(AluOperation::DEC.instr(
            cpu::Register::SP,
            cpu::Register::SP,
            cpu::Register::ZX,
        ));
    }

    while *stack_shift > 0 {
        *stack_shift -= 1;
        comp.instruct(AluOperation::INC.instr(
            cpu::Register::SP,
            cpu::Register::SP,
            cpu::Register::ZX,
        ));
    }
}

pub fn compile_stackop(
    comp: &mut dyn CompContext,
    op: Box<dyn StackOperation>,
) -> Result<(), CompileError> {
    let signature = op.signature();

    if signature.pushes == 0 && !signature.check(StackOpSignature::FLAG_IMPURE) {
        let mut shift = -(signature.takes as i8);
        apply_stack_shift(comp, &mut shift);
        return Ok(());
    }

    let mut stack_exec = StackExecCtx {
        inps: vec![
            cpu::Register::R2,
            cpu::Register::R3,
            cpu::Register::R4,
            cpu::Register::R5,
        ],
        outs: vec![
            cpu::Register::R2,
            cpu::Register::R3,
            cpu::Register::R4,
            cpu::Register::R5,
        ],
        temps: vec![],
    };

    for _ in 0..signature.temps {
        stack_exec.temps.push(stack_exec.outs.pop().unwrap());
    }

    let mut stack_shift: i8 = 0;

    for i in (0..signature.takes).rev() {
        stack_shift -= 1;
        comp.instruct(MemOperation::LOAD.instr(
            stack_exec.inps[i],
            cpu::Register::SP,
            stack_shift,
        )?);
    }

    if signature.check(StackOpSignature::FLAG_SAVE_STACK) {
        apply_stack_shift(comp, &mut stack_shift);
    }

    op.execute(&mut stack_exec, comp)?;

    for i in 0..signature.pushes {
        comp.instruct(MemOperation::STORE.instr(
            stack_exec.outs[i],
            cpu::Register::SP,
            stack_shift,
        )?);
        stack_shift += 1;
    }

    apply_stack_shift(comp, &mut stack_shift);
    Ok(())
}
