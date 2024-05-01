use crate::asm::alu::AluOperation;
use crate::asm::mem::MemOperation;
use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::compile::{Atom, CompileContext};
use crate::cpu;
use crate::parser::{ParseParts, ParsedLabel};
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub struct CallStackOp {
    pub label_id: usize,
}

impl CallStackOp {
    pub fn new(label_id: usize) -> Self {
        Self { label_id }
    }
}

impl StackOperation for CallStackOp {
    fn signature(&self) -> StackOpSignature {
        StackOpSignature {
            flags: StackOpSignature::FLAG_RESET_STACK
                | StackOpSignature::FLAG_SAVE_STACK
                | StackOpSignature::FLAG_IMPURE,
            ..Default::default()
        }
    }

    fn execute(
        &self,
        _: &mut crate::stack::StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        comp.instruct(AluOperation::INC.instr(
            cpu::Register::SP,
            cpu::Register::SP,
            cpu::Register::ZX,
        ));

        comp.instruct(AluOperation::MOV.instr(
            cpu::Register::R2,
            cpu::Register::PC,
            cpu::Register::ZX,
        ));
        comp.instruct(MemOperation::LADD.instr(cpu::Register::R2, cpu::Register::PC, 3)?);
        comp.instruct(MemOperation::STORE.instr(cpu::Register::R2, cpu::Register::SP, -1)?);
        let pos = comp.resolve_label(self.label_id)?;
        comp.instruct(MemOperation::LADD.instr(cpu::Register::PC, cpu::Register::PC, 2)?);
        comp.instruct(cpu::Instruction::CUSTOM(6));
        comp.instruct(cpu::Instruction::CUSTOM(pos));

        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}

#[derive(Clone, Debug)]
pub struct StackCallInstruction {
    pub targ: ParsedLabel,
}

impl StackCallInstruction {
    pub fn new(targ: ParsedLabel) -> Result<StackCallInstruction, CompileError> {
        Ok(StackCallInstruction { targ })
    }

    pub fn parse_asm(mut parts: ParseParts) -> Result<StackCallInstruction, CompileError> {
        StackCallInstruction::new(parts.pop_label()?)
    }
}

impl Atom for StackCallInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let label_id = self.targ.resolve(ctx)?;
        ctx.comp.stack(Box::new(CallStackOp::new(label_id)));
        Ok(())
    }
}
