use self::{
    alu::StackAluInstruction,
    base::{StackBaseInstruction, StackBaseOperation},
    call::StackCallInstruction,
    cons::{StackConstInstruction, StackConstOperation},
    func::{StackFunctionInstruction, StackFunctionOperation},
    jump::StackJumpInstruction,
    local::{StackLocalInstruction, StackLocalOperation},
    mem::StackMemInstruction,
};

use crate::compile::{AtomBox, CompileError};

use super::{
    alu::AluOperation, jump::JumpOperation, mem::MemOperation
};

use crate::parser::ParseParts;

pub mod alu;
pub mod base;
pub mod call;
pub mod cons;
pub mod func;
pub mod jump;
pub mod local;
pub mod mem;

pub fn parse_instruction(
    command_pure: &str,
    command_flags: &str,
    parts: ParseParts,
) -> Result<AtomBox, CompileError> {
    if let Some(base_op) = StackBaseOperation::parse_operation(command_pure) {
        let ins = StackBaseInstruction::parse_asm(base_op, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(base_op) = StackLocalOperation::parse_operation(command_pure) {
        let ins = StackLocalInstruction::parse_asm(base_op, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(const_op) = StackConstOperation::parse_operation(command_pure) {
        let ins = StackConstInstruction::parse_asm(const_op, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(alu_op) = AluOperation::parse_operation(command_pure) {
        let ins = StackAluInstruction::parse_asm(alu_op, command_flags);
        return Ok(Box::new(ins));
    };

    if let Some(mem_op) = MemOperation::parse_operation(command_pure) {
        let ins = StackMemInstruction::parse_asm(mem_op, command_flags);
        return Ok(Box::new(ins));
    };

    if let Some(jmp_op) = JumpOperation::parse_operation(command_pure) {
        let ins = StackJumpInstruction::parse_asm(jmp_op, parts)?;
        return Ok(Box::new(ins));
    };

    if let Some(func_op) = StackFunctionOperation::parse_operation(command_pure) {
        let ins = StackFunctionInstruction::parse_asm(func_op, parts)?;
        return Ok(Box::new(ins));
    };

    if command_pure == "CALL" {
        let ins = StackCallInstruction::parse_asm(parts)?;
        return Ok(Box::new(ins));
    };

    Err(CompileError::UnknownCommand(format!("${}", command_pure)))
}
