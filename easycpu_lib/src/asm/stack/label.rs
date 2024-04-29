use crate::asm::load_label::LoadLabelInstruction;
use crate::compile::CompileError;
use crate::compile::{Atom, CompileContext};
use crate::cpu::{self};
use crate::parser::{ParseParts, ParsedLabel};

use super::base::{StackBaseInstruction, StackBaseOperation};

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
        LoadLabelInstruction::new(cpu::Register::R2, self.label.clone())?.compile(ctx)?;
        StackBaseInstruction::new(StackBaseOperation::PUSH, cpu::Register::R2)?.compile(ctx)?;

        Ok(())
    }
}
