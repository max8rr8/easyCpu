use crate::cpu;
use crate::parser::ParseParts;
use crate::compile::CompileError;
use crate::compile::{CompileContext, Atom};

const SHIFT_RANGE: std::ops::RangeInclusive<i8> = -31..=31;

#[derive(Copy, Clone, Debug)]
pub struct BranchInstruction {
    pub eq: bool,
    pub gt: bool,
    pub lt: bool,

    pub cond: cpu::Register,
    pub shift: i8,
}

impl BranchInstruction {
    pub fn new(
        cond: cpu::Register,
        shift: i8,
    ) -> Result<BranchInstruction, CompileError> {
        if !SHIFT_RANGE.contains(&shift) {
          return Err(CompileError::ShiftIsTooBig(shift));
        }

        Ok(BranchInstruction {
            eq: true,
            gt: true,
            lt: true,
            cond,
            shift,
        })
    }

    pub fn set_flags(&mut self, eq: bool, gt: bool, lt: bool) -> &mut BranchInstruction {
        self.eq = eq;
        self.gt = gt;
        self.lt = lt;
        self
    }

    pub fn set_flags_from_str(&mut self, s: &str) -> &mut BranchInstruction {
        if s.contains('E') | s.contains('G') | s.contains('L') {
          self.set_flags(s.contains('E'), s.contains('G'), s.contains('L'));
        }
        self
    }

    pub fn parse_asm(
        flags: &str,
        mut parts: ParseParts,
    ) -> Result<BranchInstruction, CompileError> {
        let cond = parts.pop_register()?;
        let shift = parts.pop_shift()?;
        // parts.pop_front().unwrap();
        // let dst = cpu::Register::ZX;
        // let src_a = cpu::Register::ZX;
        // let src_b = cpu::Register::ZX;

        let mut ins = BranchInstruction::new(cond, shift)?;
        ins.set_flags_from_str(flags);
        Ok(ins)
    }
}

impl Atom for BranchInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let ins = cpu::BranchInstruction {
            cond: self.cond,
            shift: self.shift,

            eq: self.eq,
            gt: self.gt,
            lt: self.lt,
        };

        let ins = cpu::Instruction::BRANCH(ins);
        
        if let Err(e) = ins.validate() {
          return Err(CompileError::InvalidInstruction(e));
        }

        ctx.instruct(ins);

        Ok(())
    }
}
