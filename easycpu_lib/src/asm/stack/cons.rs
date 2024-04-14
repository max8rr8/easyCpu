use crate::compile::{compile_instructions, CompileContext, Atom};
use crate::asm::load_const::{LoadConstInstruction, LoadConstOperation};
use crate::asm::load_label::LoadLabelInstruction;
use crate::parser::{ParseParts, ParsedLabel};
use crate::compile::CompileError;
use crate::cpu::{self};

use super::base::{StackBaseInstruction, StackBaseOperation};

#[derive(Copy, Clone, Debug)]
pub enum StackConstOperation {
    CONST,
    ACONST,
    LABEL,
}

#[derive(Clone, Debug)]
pub enum ConstInternalOp {
    Label(ParsedLabel),
    Const(u16),
    AConst(u16),
}

impl StackConstOperation {
    pub fn parse_operation(s: &str) -> Option<StackConstOperation> {
        match s {
            "PCONST" => Some(StackConstOperation::CONST),
            "PLABEL" => Some(StackConstOperation::LABEL),
            "ACONST" => Some(StackConstOperation::ACONST),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StackConstInstruction {
    pub val: ConstInternalOp,
}

impl StackConstInstruction {
    pub fn new_const(val: u16) -> StackConstInstruction {
        StackConstInstruction {
            val: ConstInternalOp::Const(val),
        }
    }

    pub fn new_aconst(val: u16) -> StackConstInstruction {
        StackConstInstruction {
            val: ConstInternalOp::AConst(val),
        }
    }

    pub fn new_label(label: ParsedLabel) -> StackConstInstruction {
        StackConstInstruction {
            val: ConstInternalOp::Label(label),
        }
    }

    pub fn parse_asm(
        op: StackConstOperation,
        mut parts: ParseParts,
    ) -> Result<StackConstInstruction, CompileError> {
        Ok(match op {
            StackConstOperation::CONST => StackConstInstruction::new_const(parts.pop_const()?),
            StackConstOperation::ACONST => StackConstInstruction::new_aconst(parts.pop_const()?),
            StackConstOperation::LABEL => StackConstInstruction::new_label(parts.pop_label()?),
        })
    }
}

impl Atom for StackConstInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let mut ins: Vec<Box<dyn Atom>> = Vec::new();
        if matches!(self.val, ConstInternalOp::AConst(_)) {
            ins.push(Box::new(StackBaseInstruction::new(
                StackBaseOperation::POP,
                cpu::Register::R2,
            )?));
        }

        ins.push(match &self.val {
            ConstInternalOp::Const(val) => Box::new(LoadConstInstruction::new(
                LoadConstOperation::LOAD,
                cpu::Register::R2,
                *val,
            )?),
            ConstInternalOp::AConst(val) => Box::new(LoadConstInstruction::new(
                LoadConstOperation::ADD,
                cpu::Register::R2,
                *val,
            )?),
            ConstInternalOp::Label(label) => {
                Box::new(LoadLabelInstruction::new(cpu::Register::R2, label.clone())?)
            }
        });
        ins.push(Box::new(StackBaseInstruction::new(
            StackBaseOperation::PUSH,
            cpu::Register::R2,
        )?));
        // dbg!(&ins);

        compile_instructions(ins, ctx)
    }
}
