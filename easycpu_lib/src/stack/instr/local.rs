use crate::asm::mem::MemOperation;
use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::cpu::{self};
use crate::parser::ParseParts;

use crate::asm::alu::AluOperation;
use crate::asm::load_const::LoadConstInstruction;
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub enum StackLocalMode {
    VAR,
    ARG,
}

#[derive(Copy, Clone, Debug)]
pub enum LocalOperation {
    LOCINIT,
    LOCEND,

    LOAD(StackLocalMode),
    STORE(StackLocalMode),
    ADDR(StackLocalMode),
}

impl LocalOperation {
    pub fn parse_operation(s: &str) -> Option<LocalOperation> {
        match s {
            "LOCINIT" => Some(LocalOperation::LOCINIT),
            "LOCEND" => Some(LocalOperation::LOCEND),

            "LVAR" => Some(LocalOperation::LOAD(StackLocalMode::VAR)),
            "SVAR" => Some(LocalOperation::STORE(StackLocalMode::VAR)),
            "AVAR" => Some(LocalOperation::ADDR(StackLocalMode::VAR)),

            "LARG" => Some(LocalOperation::LOAD(StackLocalMode::ARG)),
            "SARG" => Some(LocalOperation::STORE(StackLocalMode::ARG)),
            "AARG" => Some(LocalOperation::ADDR(StackLocalMode::ARG)),

            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LocalStackOp {
    pub op: LocalOperation,
    pub idx: u16,
}

impl LocalStackOp {
    pub fn new(op: LocalOperation, idx: u16) -> Result<LocalStackOp, CompileError> {
        Ok(LocalStackOp { op, idx })
    }

    pub fn parse_asm(
        op: LocalOperation,
        mut parts: ParseParts,
    ) -> Result<LocalStackOp, CompileError> {
        let idx = match op {
            LocalOperation::LOCEND => 0,
            _ => parts.pop_const()?,
        };
        LocalStackOp::new(op, idx)
    }

    fn instruct_local_addr(
        comp: &mut dyn CompContext,
        out: cpu::Register,
        mode: StackLocalMode,
        idx: u16,
    ) -> Result<(), CompileError> {
        let shift = match mode {
            StackLocalMode::VAR => idx,
            StackLocalMode::ARG => 0u16.wrapping_sub(4).wrapping_sub(idx),
        };

        comp.instruct(AluOperation::MOV.instr(out, cpu::Register::LP, cpu::Register::SP));

        LoadConstInstruction::instr_add(out, shift)
            .into_iter()
            .for_each(|i| comp.instruct(i));

        Ok(())
    }
}

impl StackOperation for LocalStackOp {
    fn signature(&self) -> StackOpSignature {
        match self.op {
            LocalOperation::LOCINIT | LocalOperation::LOCEND => StackOpSignature {
                flags: StackOpSignature::FLAG_SAVE_STACK
                    | StackOpSignature::FLAG_RESET_STACK
                    | StackOpSignature::FLAG_IMPURE,
                ..Default::default()
            },

            LocalOperation::STORE(_) => StackOpSignature {
                takes: 1,
                temps: 1,
                flags: StackOpSignature::FLAG_IMPURE,
                ..Default::default()
            },

            LocalOperation::ADDR(_) | LocalOperation::LOAD(_) => StackOpSignature {
                pushes: 1,
                ..Default::default()
            },
        }
    }

    fn execute(
        &self,
        stack: &mut crate::stack::StackExecCtx,
        comp: &mut dyn crate::compile::comp::CompContext,
    ) -> Result<(), CompileError> {
        match self.op {
            LocalOperation::LOCINIT => {
                comp.instruct(MemOperation::STORE.instr(
                    cpu::Register::LP,
                    cpu::Register::SP,
                    0,
                )?);

                comp.instruct(AluOperation::INC.instr(
                    cpu::Register::LP,
                    cpu::Register::SP,
                    cpu::Register::ZX,
                ));

                LoadConstInstruction::instr_add(cpu::Register::SP, self.idx + 1)
                    .into_iter()
                    .for_each(|i| comp.instruct(i));
            }

            LocalOperation::LOCEND => {
                comp.instruct(AluOperation::DEC.instr(
                    cpu::Register::SP,
                    cpu::Register::LP,
                    cpu::Register::ZX,
                ));

                comp.instruct(MemOperation::LOAD.instr(cpu::Register::LP, cpu::Register::SP, 0)?);
            }

            LocalOperation::LOAD(mode) => {
                Self::instruct_local_addr(comp, stack.outs[0], mode, self.idx)?;

                comp.instruct(MemOperation::LOAD.instr(stack.outs[0], stack.outs[0], 0)?);
            }

            LocalOperation::STORE(mode) => {
                Self::instruct_local_addr(comp, stack.temps[0], mode, self.idx)?;
                comp.instruct(MemOperation::STORE.instr(stack.outs[0], stack.temps[0], 0)?);
            }

            LocalOperation::ADDR(mode) => {
                Self::instruct_local_addr(comp, stack.outs[0], mode, self.idx)?;
            }
        }

        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
