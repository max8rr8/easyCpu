use crate::compile::comp::CompContext;
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

    pub fn instr_shift(
        comp: &mut dyn CompContext,
        dst: cpu::Register,
        label_id: usize,
        shift: u16,
    ) -> Result<(), CompileError> {
        let targ_pos = comp.resolve_label(label_id)?.wrapping_add(shift);

        if dst != cpu::Register::PC {
            comp.instruct(AluOperation::MOV.instr(dst, cpu::Register::PC, cpu::Register::ZX));
        }

        let v = LoadConstInstruction::instr_add(dst, targ_pos);
        for inst in v {
            comp.instruct(inst);
        }

        Ok(())
    }

    pub fn instr(
        comp: &mut dyn CompContext,
        dst: cpu::Register,
        label_id: usize,
    ) -> Result<(), CompileError> {
        Self::instr_shift(comp, dst, label_id, 0)
    }
}

impl Atom for LoadLabelInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let label_id = self.label.resolve(ctx)?;
        Self::instr(ctx.comp.as_mut(), self.dst, label_id)?;

        Ok(())
    }
}
