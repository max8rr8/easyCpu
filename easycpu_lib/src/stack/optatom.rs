use std::{mem, rc::Rc};

use crate::{compile::{
    comp::CompContext,
    label::LabelScope,
    status::ContextStatus,
    Atom, AtomBox, CompileError,
}, cpu};

pub struct StackOptComp {
    status: Rc<ContextStatus>,
}

impl StackOptComp {
    pub fn new(status: Rc<ContextStatus>) -> Self {
        StackOptComp { status }
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

    fn emit_label(&mut self, _: usize) -> Result<(), CompileError> {
        Err(CompileError::InstructionInStackopt)
    }

    fn resolve_label(&mut self, _: usize) -> Result<u16, CompileError> {
        Err(CompileError::InstructionInStackopt)
    }
    
    fn stack(&mut self, _: Box<dyn super::StackOperation>) {
        dbg!("Stack in stackopt");
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

        Ok(())
    }
}
