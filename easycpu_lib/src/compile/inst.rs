use crate::cpu;
use std::collections::HashMap;
use std::fmt;

use crate::compile::CompileError;

pub struct CompileContext<'a> {
    pub current_pc: u16,
    pub label_map: &'a HashMap<(usize, &'a String), Option<u16>>,
    pub scope_stack: &'a Vec<usize>,
}

pub trait Instruction: fmt::Debug {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError>;
}

pub fn compile_instructions(
    instructions: Vec<Box<dyn Instruction>>,
    ctx: &CompileContext,
) -> Result<Vec<cpu::Instruction>, CompileError> {
    let compiled: Result<Vec<_>, _> = instructions.into_iter().map(|x| x.compile(ctx)).collect();
    let compiled = compiled?;
    let compiled: Vec<cpu::Instruction> = compiled.into_iter().flatten().collect();

    Ok(compiled)
}
