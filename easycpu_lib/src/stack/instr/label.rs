use crate::asm::load_label::LoadLabelInstruction;
use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::compile::{Atom, CompileContext};
use crate::parser::{ParseParts, ParsedLabel};
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub struct LabelStackOp {
    pub label_id: usize,
}

impl LabelStackOp {
    pub fn new(label_id: usize) -> Self {
        Self { label_id }
    }
}

impl StackOperation for LabelStackOp {
    fn signature(&self) -> StackOpSignature {
        StackOpSignature {
            pushes: 1,
            ..Default::default()
        }
    }

    fn execute(
        &mut self,
        stack: &mut crate::stack::StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        LoadLabelInstruction::instr(comp, stack.outs[0], self.label_id)
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}

#[derive(Clone, Debug)]
pub struct StackLabelInstruction {
    pub label: ParsedLabel,
}

impl StackLabelInstruction {
    pub fn new_label(label: ParsedLabel) -> StackLabelInstruction {
        StackLabelInstruction { label }
    }

    pub fn parse_asm(mut parts: ParseParts) -> Result<StackLabelInstruction, CompileError> {
        Ok(StackLabelInstruction::new_label(parts.pop_label()?))
    }
}

impl Atom for StackLabelInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let label_id = self.label.resolve(ctx)?;
        ctx.comp.stack(Box::new(LabelStackOp::new(label_id)));
        Ok(())
    }
}
