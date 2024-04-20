use crate::compile::CompileError;
use crate::cpu::{self};
use crate::parser::{ParseParts, ParsedLabel};

use super::alu::AluOperation;
use super::load_const::LoadConstInstruction;
use crate::compile::{Atom, CompileContext};

#[derive(Clone, Debug)]
pub struct LoadLabelInstruction {
    pub dst: cpu::Register,
    pub label: ParsedLabel,
}

impl LoadLabelInstruction {
    pub fn new(
        dst: cpu::Register,
        label: ParsedLabel,
    ) -> Result<LoadLabelInstruction, CompileError> {
        Ok(LoadLabelInstruction { label, dst })
    }

    pub fn parse_asm(mut parts: ParseParts) -> Result<LoadLabelInstruction, CompileError> {
        let dst = parts.pop_register()?;
        let label = parts.pop_label()?;

        LoadLabelInstruction::new(dst, label)
    }
}

impl Atom for LoadLabelInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let targ_pos = self.label.resolve(ctx)?;

        if self.dst != cpu::Register::PC {
            ctx.instruct(AluOperation::MOV.instr(self.dst, cpu::Register::PC, cpu::Register::ZX));
        }

        let v = LoadConstInstruction::instr_add(self.dst, targ_pos);
        for inst in v {
            ctx.instruct(inst);
        }

        Ok(())
    }
}
