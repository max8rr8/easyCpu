use crate::compile::CompileError;
use crate::parser::ParseParts;
use crate::cpu;

use crate::compile::{CompileContext, Atom};

#[derive(Copy, Clone, Debug)]
pub enum MemOperation {
    LOAD,
    LADD,
    LSUB,
    STORE,
}

#[derive(Copy, Clone, Debug)]
pub struct MemInstruction {
    pub op: MemOperation,

    pub hi: bool,
    pub lo: bool,
    pub sw: bool,

    pub dst: cpu::Register,
    pub addr: cpu::Register,
    pub shift: i8,
}

impl MemOperation {
    fn match_instruction(&self, ins: cpu::MemInstruction) -> cpu::Instruction {
        match self {
            MemOperation::LOAD | MemOperation::LADD | MemOperation::LSUB => cpu::Instruction::LOAD(ins),
            MemOperation::STORE => cpu::Instruction::STORE(ins),
        }
    }

    pub fn parse_operation(s: &str) -> Option<MemOperation> {
        match s {
            "LOAD" => Some(MemOperation::LOAD),
            "LADD" => Some(MemOperation::LADD),
            "LSUB" => Some(MemOperation::LSUB),
            "STORE" => Some(MemOperation::STORE),
            _ => None,
        }
    }

    fn get_flags(&self) -> (bool, bool, bool) {
        match self {
            MemOperation::LOAD |  MemOperation::STORE => (true, true, false), // Set flags to load/store whole value
            MemOperation::LADD => (false, false, false), // Set flags for add load
            MemOperation::LSUB => (false, false, true), // Set flags for sub load
        }
    }

    pub fn instr(&self, dst: cpu::Register, addr: cpu::Register, shift: i8) -> Result<cpu::Instruction, CompileError> {
        let (op_hi, op_lo, op_sw) = self.get_flags();
        let ins = cpu::MemInstruction {
            dst, addr, shift,

            hi: op_hi,
            lo: op_lo,
            sw: op_sw,
        };

        let ins = self.match_instruction(ins);

        if let Err(e) = ins.validate() {
            return Err(CompileError::InvalidInstruction(e));
        }

        Ok(ins)
    }
}

impl MemInstruction {
    pub fn new(
        op: MemOperation,
        dst: cpu::Register,
        addr: cpu::Register,
        shift: i8,
    ) -> Result<MemInstruction, CompileError> {
        Ok(MemInstruction {
            op,
            hi: false,
            lo: false,
            sw: false,
            dst,
            addr,
            shift,
        })
    }

    pub fn set_flags(&mut self, hi: bool, lo: bool, sw: bool) -> &mut MemInstruction {
        self.hi = hi;
        self.lo = lo;
        self.sw = sw;
        self
    }

    pub fn set_flags_from_str(&mut self, s: &str) -> &mut MemInstruction {
        if s.contains('H') || s.contains('L') {
            // If either H or L are defined we shall set all flags
            self.set_flags(s.contains('H'), s.contains('L'), s.contains('S'));
        } else {
            self.set_flags(false, false, s.contains('S'));
        }
        self
    }

    pub fn parse_asm(
        op: MemOperation,
        flags: &str,
        mut parts: ParseParts,
    ) -> Result<MemInstruction, CompileError> {
        let dst = parts.pop_register()?;
        let addr = parts.pop_register()?;
        let shift = parts.pop_shift()?;

        let mut ins = MemInstruction::new(op, dst, addr, shift)?;
        ins.set_flags_from_str(flags);
        Ok(ins)
    }
}

impl Atom for MemInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let (op_hi, op_lo, op_sw) = self.op.get_flags();
        let ins = cpu::MemInstruction {
            dst: self.dst,
            addr: self.addr,
            shift: self.shift,

            hi: self.hi ^ op_hi,
            lo: self.lo ^ op_lo,
            sw: self.sw ^ op_sw,
        };

        let ins = self.op.match_instruction(ins);

        if let Err(e) = ins.validate() {
            return Err(CompileError::InvalidInstruction(e));
        }

        ctx.instruct(ins);
        Ok(())
    }
}
