use crate::asm::{err::CompileError, inst::Instruction};
use crate::parser::{ParseParts, convert_to_u16};
use crate::cpu::{self};

use super::alu::{AluInstruction, AluOperation};
use super::branch::BranchInstruction;
use super::inst::{compile_instructions, CompileContext, CustomInstruction};
use super::mem::{MemInstruction, MemOperation};

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
    ) -> Option<Vec<Box<dyn Instruction>>> {
        match val {
            0 => Some(vec![Box::new(AluInstruction::new(
                AluOperation::ADD,
                dst,
                src,
                cpu::Register::ZX,
            ))]),

            1 => Some(vec![Box::new(AluInstruction::new(
                AluOperation::INC,
                dst,
                src,
                cpu::Register::ZX,
            ))]),

            2 => Some(vec![
                Box::new(AluInstruction::new(
                    AluOperation::INC,
                    dst,
                    src,
                    cpu::Register::ZX,
                )),
                Box::new(AluInstruction::new(
                    AluOperation::INC,
                    dst,
                    dst,
                    cpu::Register::ZX,
                )),
            ]),

            0xffff => Some(vec![Box::new(AluInstruction::new(
                AluOperation::DEC,
                dst,
                src,
                cpu::Register::ZX,
            ))]),

            // 0xfffe => Some(vec![Box::new(
            //     *AluInstruction::new(
            //         AluOperation::ADD,
            //         dst,
            //         cpu::Register::ZX,
            //         cpu::Register::ZX,
            //     )
            //     .set_flags(true, true, false),
            // )]),

            _ => None,
        }
    }
}

impl Instruction for LoadConstInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let val_neg = u16::MAX.wrapping_sub(self.val).wrapping_add(1);

        let (mem_op, src_reg) = match self.op {
            LoadConstOperation::LOAD => (MemOperation::LOAD, cpu::Register::ZX),
            LoadConstOperation::ADD => (MemOperation::LADD, self.dst),
        };

        
        let ins: Vec<Box<dyn Instruction>> =
            if let Some(v) = LoadConstInstruction::short_variant(self.val, self.dst, src_reg) {
                v
            } else if self.val < 4096 {
                // We can fit up to 12 bits of data into two operations
                vec![
                    Box::new(MemInstruction::new(mem_op, self.dst, cpu::Register::PC, 1)?),
                    Box::new(CustomInstruction::new(self.val)),
                ]
            } else if val_neg < 4096
                && matches!(self.op, LoadConstOperation::LOAD)
                && self.dst != cpu::Register::PC
            {
                vec![
                    Box::new(AluInstruction::new(
                        AluOperation::MOV,
                        self.dst,
                        cpu::Register::ZX,
                        cpu::Register::ZX,
                    )),
                    Box::new(MemInstruction::new(
                        MemOperation::LSUB,
                        self.dst,
                        cpu::Register::PC,
                        1,
                    )?),
                    Box::new(CustomInstruction::new(val_neg)),
                ]
            } else if val_neg < 4096 && matches!(self.op, LoadConstOperation::ADD) {

                vec![
                    Box::new(MemInstruction::new(
                        MemOperation::LSUB,
                        self.dst,
                        cpu::Register::PC,
                        1,
                    )?),
                    Box::new(CustomInstruction::new(val_neg)),
                ]
            } else {

                vec![
                    Box::new(MemInstruction::new(
                        mem_op,
                        self.dst,
                        cpu::Register::PC,
                        2,
                    )?),
                    Box::new(BranchInstruction::new(cpu::Register::ZX, 2)?), // Jump over value so it would not be executed
                    Box::new(CustomInstruction::new(self.val)),
                ]
            };

        compile_instructions(ins, ctx)
    }
}
