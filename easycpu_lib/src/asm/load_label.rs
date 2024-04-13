use crate::asm::{err::CompileError, inst::Instruction};
use crate::parser::{ParsedLabel, ParseParts};
use crate::cpu::{self};

use super::alu::{AluInstruction, AluOperation};
use super::inst::{compile_instructions, CompileContext};
use super::load_const::{LoadConstInstruction, LoadConstOperation};

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

impl Instruction for LoadLabelInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let mut ins: Vec<Box<dyn Instruction>> = Vec::new();
        if self.dst != cpu::Register::PC {
            ins.push(Box::new(AluInstruction::new(
                AluOperation::MOV,
                self.dst,
                cpu::Register::PC,
                cpu::Register::ZX,
            )));
        }
        ins.push(Box::new(LoadConstInstruction::new(
            LoadConstOperation::ADD,
            self.dst,
            self.label.resolve(ctx)?,
        )?));

        compile_instructions(ins, ctx)
    }
}
