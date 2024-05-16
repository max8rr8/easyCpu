use std::{mem, rc::Rc};

use crate::{
    compile::{
        comp::CompContext, label::LabelScope, status::ContextStatus, Atom, AtomBox, CompileError,
    },
    cpu, stack::opt,
};

use super::{StackOpSignature, StackOperation};

#[derive(Clone, Copy, Debug)]
pub struct LabelMarkStackOp {
    label_id: usize,
}

impl StackOperation for LabelMarkStackOp {
    fn signature(&self) -> StackOpSignature {
        StackOpSignature {
            flags: StackOpSignature::FLAG_SAVE_STACK | StackOpSignature::FLAG_RESET_STACK,
            ..Default::default()
        }
    }

    fn execute(
        &self,
        _: &mut super::StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        comp.emit_label(self.label_id)
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}

pub struct StackOptComp {
    status: Rc<ContextStatus>,
    ops: Vec<Box<dyn super::StackOperation>>,
}

impl StackOptComp {
    pub fn new(status: Rc<ContextStatus>) -> Self {
        StackOptComp { status, ops: Vec::new() }
    }
}

impl CompContext for StackOptComp {
    fn instruct(&mut self, _: cpu::Instruction) {
        self.status.report_err(CompileError::InstructionInStackopt);
    }

    fn emit_new_label(&mut self) -> usize {
        self.status.report_err(CompileError::InstructionInStackopt);
        0
    }

    fn emit_label(&mut self, label_id: usize) -> Result<(), CompileError> {
        self.ops.push(Box::new(LabelMarkStackOp { label_id }));
        Ok(())
    }

    fn resolve_label(&mut self, _: usize) -> Result<u16, CompileError> {
        Err(CompileError::InstructionInStackopt)
    }

    fn stack(&mut self, op: Box<dyn super::StackOperation>) {
        self.ops.push(op);
    }
}

#[derive(Debug)]
pub struct StackOptAtom {
    scope: LabelScope,
}

impl StackOptAtom {
    pub fn new(atoms: Vec<AtomBox>) -> Self {
        StackOptAtom {
            scope: LabelScope::new(atoms),
        }
    }
}

impl Atom for StackOptAtom {
    fn compile(&self, ctx: &mut crate::compile::CompileContext) -> Result<(), CompileError> {
        if !ctx.named_resolver.ready() {
            ctx.status.recompile();
            return self.scope.compile(ctx);
        }

        let mut comp: Box<dyn CompContext> = Box::new(StackOptComp::new(ctx.status.clone()));

        mem::swap(&mut ctx.comp, &mut comp);
        self.scope.compile(ctx)?;
        mem::swap(&mut ctx.comp, &mut comp);

        let comp_r: &mut StackOptComp = comp
            .as_any_mut()
            .downcast_mut()
            .expect("Wtf compile contex");

        let mut ops = Vec::new();
        mem::swap(&mut ops, &mut comp_r.ops);

        let ops = opt::optim::optimize(ops);
        opt::comp::compile(ops, ctx.comp.as_mut())?;

        Ok(())
    }
}
