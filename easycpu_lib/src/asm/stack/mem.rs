use crate::compile::{compile_instructions, CompileContext, Instruction};
use crate::compile::CompileError;
use crate::cpu::{self};

use crate::asm::mem::{MemInstruction, MemOperation};

use super::base::{StackBaseInstruction, StackBaseOperation};

#[derive(Copy, Clone, Debug)]
pub struct StackMemInstruction {
    pub op: MemOperation,

    pub hi: bool,
    pub lo: bool,
    pub sw: bool,
}

impl StackMemInstruction {
    pub fn new(op: MemOperation) -> StackMemInstruction {
        StackMemInstruction {
            op,
            hi: false,
            lo: false,
            sw: false,
        }
    }

    pub fn set_flags(&mut self, hi: bool, lo: bool, sw: bool) -> &mut StackMemInstruction {
        self.hi = hi;
        self.lo = lo;
        self.sw = sw;
        self
    }

    pub fn set_flags_from_str(&mut self, s: &str) -> &mut StackMemInstruction {
        if s.contains('H') || s.contains('L') {
            // If either H or L are defined we shall set all flags
            self.set_flags(s.contains('H'), s.contains('L'), s.contains('S'));
        } else {
            self.set_flags(false, false, s.contains('S'));
        }
        self
    }

    pub fn parse_asm(op: MemOperation, flags: &str) -> StackMemInstruction {
        *StackMemInstruction::new(op).set_flags_from_str(flags)
    }
}

impl Instruction for StackMemInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let mut ins: Vec<Box<dyn Instruction>> = Vec::new();

        ins.push(Box::new(StackBaseInstruction::new(
            StackBaseOperation::POP,
            cpu::Register::R3,
        )?));

        if !matches!(self.op, MemOperation::LOAD) {
            ins.push(Box::new(StackBaseInstruction::new(
                StackBaseOperation::POP,
                cpu::Register::R2,
            )?));
        }

        ins.push(Box::new(
            *MemInstruction::new(self.op, cpu::Register::R2, cpu::Register::R3, 0)?
                .set_flags(self.hi, self.lo, self.sw),
        ));

        if !matches!(self.op, MemOperation::STORE) {
            ins.push(Box::new(StackBaseInstruction::new(
                StackBaseOperation::PUSH,
                cpu::Register::R2,
            )?));
        }
        // dbg!(&ins);
        compile_instructions(ins, ctx)
    }
}
