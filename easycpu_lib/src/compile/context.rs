use std::rc::Rc;

use crate::cpu;

use super::{
    comp::{CompContext, MainCompContext},
    label::LabelResolver,
    status::ContextStatus,
    CompileError,
};

pub struct CompileContext {
    pub comp: Box<dyn CompContext>,
    pub named_resolver: Box<LabelResolver>,
    pub status: Rc<ContextStatus>,
}

impl CompileContext {
    pub fn new() -> Self {
        let status = Rc::new(ContextStatus::new());

        CompileContext {
            comp: Box::new(MainCompContext::new(status.clone())),
            named_resolver: Box::new(LabelResolver::new()),
            status,
        }
    }

    pub fn instruct(&mut self, instruction: cpu::Instruction) {
        self.comp.instruct(instruction)
    }

    pub fn emit_new_label(&mut self) -> usize {
        self.comp.emit_new_label()
    }

    pub fn emit_label(&mut self, id: usize) -> Result<(), CompileError> {
        self.comp.emit_label(id)
    }

    pub fn resolve_label(&mut self, label_id: usize) -> Result<u16, CompileError> {
        self.comp.resolve_label(label_id)
    }
}
