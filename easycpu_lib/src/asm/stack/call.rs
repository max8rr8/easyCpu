use crate::asm::alu::AluOperation;
use crate::asm::mem::MemOperation;
use crate::compile::CompileError;
use crate::compile::{CompileContext, Instruction};
use crate::cpu;
use crate::parser::{ParseParts, ParsedLabel};

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

impl Instruction for StackCallInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        ctx.instruct(AluOperation::INC.instr(
            cpu::Register::SP,
            cpu::Register::SP,
            cpu::Register::ZX,
        ));

        ctx.instruct(AluOperation::MOV.instr(
            cpu::Register::R2,
            cpu::Register::PC,
            cpu::Register::ZX,
        ));
        ctx.instruct(MemOperation::LADD.instr(cpu::Register::R2, cpu::Register::PC, 3)?);
        ctx.instruct(MemOperation::STORE.instr(cpu::Register::R2, cpu::Register::SP, -1)?);
        let pos = self.targ.resolve(ctx)?;
        ctx.instruct(MemOperation::LADD.instr(cpu::Register::PC, cpu::Register::PC, 2)?);
        ctx.instruct(cpu::Instruction::CUSTOM(6));
        ctx.instruct(cpu::Instruction::CUSTOM(pos));

        Ok(())
    }
}
