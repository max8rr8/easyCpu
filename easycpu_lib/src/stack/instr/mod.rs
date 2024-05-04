pub mod alu;
pub mod call;
pub mod cons;
pub mod func;
pub mod jump;
pub mod label;
pub mod local;
pub mod mem;
pub mod manip;

pub use alu::AluStackOp;
pub use cons::ConstStackOp;
pub use func::FunctionStackOp;
pub use jump::{JumpStackOp, StackJumpInstruction};
pub use label::{LabelStackOp, StackLabelInstruction};
pub use call::{CallStackOp, StackCallInstruction};
pub use local::LocalStackOp;
pub use mem::MemStackOp;

use crate::{
    asm::{alu::AluOperation, jump::JumpOperation, mem::MemOperation},
    compile::{AtomBox, CompileError},
    parser::ParseParts,
};

use self::{func::FunctionOperation, manip::ManipStackOp};

use super::StackOpInstruction;

pub fn parse_instruction(
    command_pure: &str,
    command_flags: &str,
    mut parts: ParseParts,
) -> Result<AtomBox, CompileError> {
    if let Some(base_op) = local::LocalOperation::parse_operation(command_pure) {
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

    if let Some(op) = ManipStackOp::parse_asm(command_pure, &mut parts)? {
      return Ok(StackOpInstruction::wrap_atombox(op));
    }

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
