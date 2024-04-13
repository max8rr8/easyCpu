use crate::cpu;
use std::collections::HashMap;
use std::fmt;

use crate::compile::CompileError;

pub struct CompileContext {
    pub current_pc: u16,
    pub label_map: HashMap<(usize, String), Option<u16>>,
    pub scope_stack: Vec<usize>,
    pub instructions: Vec<cpu::Instruction>
}

impl CompileContext {
    pub fn instruct(&mut self, instruction: cpu::Instruction) {
        self.instructions.push(instruction);
        self.current_pc += 1;
    }

    pub fn patch_instruct(&mut self, pc: u16, instruction: cpu::Instruction) {
        self.instructions[pc as usize] = instruction;
    }
}

pub trait Instruction: fmt::Debug {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError>;
}

pub fn compile_instructions(
    instructions: Vec<Box<dyn Instruction>>,
    ctx: &mut CompileContext,
) -> Result<(), CompileError> {
    let compiled: Result<Vec<_>, _> = instructions.into_iter().map(|x| x.compile(ctx)).collect();
    let compiled = compiled?;

    Ok(())
}
