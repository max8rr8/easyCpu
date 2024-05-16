use crate::compile::comp::CompContext;
use crate::cpu;
use crate::parser::{ParseParts, ParsedLabel};

use super::mem::MemOperation;
use crate::compile::CompileError;
use crate::compile::{CompileContext, Atom};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum JumpOperation {
    JMP,
    JEQ,
    JGT,
    JLT,
    JLE,
    JGE,
    JNE,
}

#[derive(Clone, Debug)]
pub struct JumpInstruction {
    pub op: JumpOperation,
    pub targ: ParsedLabel,
    pub cond: cpu::Register,
}

impl JumpOperation {
    fn get_flags(&self) -> (bool, bool, bool) {
        match self {
            JumpOperation::JMP => (true, true, true),

            JumpOperation::JEQ => (true, false, false),
            JumpOperation::JGT => (false, true, false),
            JumpOperation::JLT => (false, false, true),

            JumpOperation::JGE => (true, true, false),
            JumpOperation::JLE => (true, false, true),
            JumpOperation::JNE => (false, true, true),
        }
    }

    pub fn parse_operation(s: &str) -> Option<JumpOperation> {
        match s {
            "JMP" => Some(JumpOperation::JMP),
            "JEQ" => Some(JumpOperation::JEQ),
            "JGT" => Some(JumpOperation::JGT),
            "JLT" => Some(JumpOperation::JLT),
            "JLE" => Some(JumpOperation::JLE),
            "JGE" => Some(JumpOperation::JGE),
            "JNE" => Some(JumpOperation::JNE),
            _ => None,
        }
    }
}

impl JumpInstruction {
    pub fn new(op: JumpOperation, targ: ParsedLabel, cond: cpu::Register) -> JumpInstruction {
        JumpInstruction { op, targ, cond }
    }

    pub fn parse_asm(
        op: JumpOperation,
        mut parts: ParseParts,
    ) -> Result<JumpInstruction, CompileError> {
        let cond = match op {
            JumpOperation::JMP => cpu::Register::ZX,
            _ => parts.pop_register()?,
        };
        let targ = parts.pop_label()?;

        let ins = JumpInstruction::new(op, targ, cond);
        Ok(ins)
    }

    fn convert_u16_to_shift(inp: u16) -> Result<i8, CompileError> {
        if inp < 32 {
            Ok(inp as i8)
        } else if inp > 0xFFE0 {
            let conv = u16::MAX.wrapping_sub(inp) as i8;
            Ok(-conv - 1)
        } else {
            Err(CompileError::ShiftIsTooBig(0x7f))
        }
    }

    pub fn instr(
        comp: &mut dyn CompContext,
        op: JumpOperation,
        cond: cpu::Register,
        label_id: usize,
    ) -> Result<(), CompileError> {
        let mut targ = comp.resolve_label(label_id)?;
        let (eq, gt, lt) = op.get_flags();

        let converted = JumpInstruction::convert_u16_to_shift(targ).ok();

        if let Some(converted) = converted {
            comp.instruct(cpu::Instruction::BRANCH(cpu::BranchInstruction {
                eq,
                gt,
                lt,
                cond,
                shift: converted,
            }));
            return Ok(());
        }

        if !(eq && gt && lt) {
            comp.instruct(cpu::Instruction::BRANCH(cpu::BranchInstruction {
                eq: !eq,
                gt: !gt,
                lt: !lt,
                cond,
                shift: 3,
            }));
            targ = targ.wrapping_sub(1);
        }

        comp.instruct(MemOperation::LADD.instr(cpu::Register::PC, cpu::Register::PC, 1)?);
        comp.instruct(cpu::Instruction::CUSTOM(targ));

        Ok(())

    }
}

impl Atom for JumpInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let label_id = self.targ.resolve(ctx)?;
        Self::instr(ctx.comp.as_mut(), self.op, self.cond, label_id)
    }
}
