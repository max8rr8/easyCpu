use crate::compile::CompileError;
use crate::cpu::{self};
use crate::parser::{convert_to_u16, ParseParts};

use super::alu::AluOperation;
use super::mem::MemOperation;
use crate::compile::{Atom, CompileContext};

#[derive(Copy, Clone, Debug)]
pub enum LoadConstOperation {
    LOAD,
    ADD,
}

impl LoadConstOperation {
    pub fn parse_operation(s: &str) -> Option<LoadConstOperation> {
        match s {
            "LCONST" => Some(LoadConstOperation::LOAD),
            "ACONST" => Some(LoadConstOperation::ADD),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LoadConstInstruction {
    pub op: LoadConstOperation,
    pub dst: cpu::Register,
    pub val: u16,
}

impl LoadConstInstruction {
    pub fn new(
        op: LoadConstOperation,
        dst: cpu::Register,
        val: u16,
    ) -> Result<LoadConstInstruction, CompileError> {
        Ok(LoadConstInstruction { op, dst, val })
    }

    pub fn new_signed(
        op: LoadConstOperation,
        dst: cpu::Register,
        val: i32,
    ) -> Result<LoadConstInstruction, CompileError> {
        LoadConstInstruction::new(op, dst, convert_to_u16(val)?)
    }

    pub fn parse_asm(
        op: LoadConstOperation,
        mut parts: ParseParts,
    ) -> Result<LoadConstInstruction, CompileError> {
        let dst = parts.pop_register()?;
        let val = parts.pop_const()?;

        LoadConstInstruction::new(op, dst, val)
    }

    pub fn short_variant(
        val: u16,
        dst: cpu::Register,
        src: cpu::Register,
    ) -> Option<Vec<cpu::Instruction>> {
        match val {
            0 => Some(vec![AluOperation::ADD.instr(dst, src, cpu::Register::ZX)]),
            1 => Some(vec![AluOperation::INC.instr(dst, src, cpu::Register::ZX)]),
            0xffff => Some(vec![AluOperation::DEC.instr(dst, src, cpu::Register::ZX)]),

            2 => Some(vec![
                AluOperation::INC.instr(dst, src, cpu::Register::ZX),
                AluOperation::INC.instr(dst, dst, cpu::Register::ZX),
            ]),
            0xfffe => Some(vec![
                AluOperation::DEC.instr(dst, src, cpu::Register::ZX),
                AluOperation::DEC.instr(dst, dst, cpu::Register::ZX),
            ]),

            _ => None,
        }
    }

    pub fn instr_add(dst: cpu::Register, val: u16) -> Vec<cpu::Instruction> {
        let val_neg = u16::MAX.wrapping_sub(val).wrapping_add(1);

        if let Some(v) = Self::short_variant(val, dst, dst) {
            v
        } else if val < 4096 || dst == cpu::Register::PC {
            // We can fit up to 12 bits of data into two operations
            // Ooor if we are operating on PC basically everything
            vec![
                MemOperation::LADD
                    .instr(dst, cpu::Register::PC, 1)
                    .expect("CONST BAD LADD"),
                cpu::Instruction::CUSTOM(val),
            ]
        } else if val_neg < 4096 {
            vec![
                MemOperation::LSUB
                    .instr(dst, cpu::Register::PC, 1)
                    .expect("CONST BAD LSUB"),
                cpu::Instruction::CUSTOM(val_neg),
            ]
        } else {
            vec![
                MemOperation::LADD
                    .instr(dst, cpu::Register::PC, 2)
                    .expect("CONST BAD LADD"),
                cpu::Instruction::BRANCH(cpu::BranchInstruction {
                    eq: true,
                    gt: true,
                    lt: true,
                    cond: cpu::Register::ZX,
                    shift: 2,
                }), // Jump over value so it would not be executed
                cpu::Instruction::CUSTOM(val),
            ]
        }
    }

    pub fn instr_load(dst: cpu::Register, val: u16) -> Vec<cpu::Instruction> {
        let val_neg = u16::MAX.wrapping_sub(val).wrapping_add(1);

        if let Some(v) = Self::short_variant(val, dst, cpu::Register::ZX) {
            v
        } else if val < 4096 || dst == cpu::Register::PC {
            // We can fit up to 12 bits of data into two operations
            // Ooor if we are operating on PC basically everything
            vec![
                MemOperation::LOAD
                    .instr(dst, cpu::Register::PC, 1)
                    .expect("CONST BAD LOAD"),
                cpu::Instruction::CUSTOM(val),
            ]
        } else if val_neg < 4096 {
            vec![
                AluOperation::MOV.instr(dst, cpu::Register::ZX, cpu::Register::ZX),
                MemOperation::LSUB
                    .instr(dst, cpu::Register::PC, 1)
                    .expect("CONST BAD LSUB"),
                cpu::Instruction::CUSTOM(val_neg),
            ]
        } else {
            vec![
                MemOperation::LOAD
                    .instr(dst, cpu::Register::PC, 2)
                    .expect("CONST BAD LADD"),
                cpu::Instruction::BRANCH(cpu::BranchInstruction {
                    eq: true,
                    gt: true,
                    lt: true,
                    cond: cpu::Register::ZX,
                    shift: 2,
                }), // Jump over value so it would not be executed
                cpu::Instruction::CUSTOM(val),
            ]
        }
    }
}

impl Atom for LoadConstInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let v = match self.op {
            LoadConstOperation::LOAD => Self::instr_load(self.dst, self.val),
            LoadConstOperation::ADD => Self::instr_add(self.dst, self.val),
        };

        for inst in v {
            ctx.instruct(inst);
        }

        Ok(())
    }
}
