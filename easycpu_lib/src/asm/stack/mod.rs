use self::{
    base::{StackBaseInstruction, StackBaseOperation},
    call::StackCallInstruction,
    jump::StackJumpInstruction,
    label::StackLabelInstruction,
};

use crate::{
    compile::{AtomBox, CompileError},
    stack::{
        instr::{
            alu::AluStackOp,
            cons::ConstStackOp,
            func::{FunctionOperation, FunctionStackOp},
            local::{LocalOperation, LocalStackOp},
            mem::MemStackOp,
        },
        stackop::StackOpInstruction,
    },
};

use super::{alu::AluOperation, jump::JumpOperation, mem::MemOperation};

use crate::parser::ParseParts;

pub mod base;
pub mod call;
pub mod jump;
pub mod label;

pub fn parse_instruction(
    command_pure: &str,
    command_flags: &str,
    parts: ParseParts,
) -> Result<AtomBox, CompileError> {
    if let Some(base_op) = StackBaseOperation::parse_operation(command_pure) {
        let ins = StackBaseInstruction::parse_asm(base_op, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(base_op) = LocalOperation::parse_operation(command_pure) {
        let ins = LocalStackOp::parse_asm(base_op, parts)?;
        return Ok(StackOpInstruction::wrap_atombox(ins));
    };

    if command_pure == "PLABEL" {
        let ins = StackLabelInstruction::parse_asm(parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(alu_op) = AluOperation::parse_operation(command_pure) {
        let op = AluStackOp::parse_asm(alu_op, command_flags);
        return Ok(StackOpInstruction::wrap_atombox(op));
    };

    if let Some(cons_op) = ConstStackOp::parse_operation(command_pure) {
        let op = ConstStackOp::parse_asm(cons_op, parts)?;
        return Ok(StackOpInstruction::wrap_atombox(op));
    };

    if let Some(mem_op) = MemOperation::parse_operation(command_pure) {
        let ins = MemStackOp::parse_asm(mem_op);
        return Ok(StackOpInstruction::wrap_atombox(ins));
    };

    if let Some(jmp_op) = JumpOperation::parse_operation(command_pure) {
        let ins = StackJumpInstruction::parse_asm(jmp_op, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(func_op) = FunctionOperation::parse_operation(command_pure) {
        let ins = FunctionStackOp::parse_asm(func_op, parts)?;
        return Ok(StackOpInstruction::wrap_atombox(ins));
    };

    if command_pure == "CALL" {
        let ins = StackCallInstruction::parse_asm(parts)?;
        return Ok(Box::new(ins));
    };

    Err(CompileError::UnknownCommand(format!("${}", command_pure)))
}
