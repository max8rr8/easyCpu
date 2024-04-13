use crate::cpu::{self};
use crate::parser::{ParseParts, ParsedLabel};

use super::branch::BranchInstruction;
use crate::compile::CompileError;
use crate::compile::{CompileContext, Instruction};
use super::load_const::LoadConstInstruction;
use super::load_const::LoadConstOperation;

#[derive(Copy, Clone, Debug)]
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
}

impl Instruction for JumpInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let (eq, gt, lt) = self.op.get_flags();
        let targ = self.targ.resolve(ctx)?;

        let converted = JumpInstruction::convert_u16_to_shift(targ).ok();

        if let Some(converted) = converted {
            let mut branch_ins = BranchInstruction::new(self.cond, converted)?;
            branch_ins.set_flags(eq, gt, lt);
            return branch_ins.compile(ctx);
        }

        let load_label = LoadConstInstruction::new(
            LoadConstOperation::ADD,
            cpu::Register::PC,
            targ.wrapping_sub(1),
        )?;
        let load_label = load_label.compile(ctx)?;

        let mut res: Vec<cpu::Instruction> = Vec::new();
        {
            let mut branch_ins = BranchInstruction::new(self.cond, load_label.len() as i8 + 1)?;
            branch_ins.set_flags(!eq, !gt, !lt);
            res.extend(branch_ins.compile(ctx)?)
        }
        res.extend(load_label);
        Ok(res)
        // Err(CompileError::ShiftIsTooBig(0x1f))
    }
}
