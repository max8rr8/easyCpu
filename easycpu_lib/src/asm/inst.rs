use std::collections::HashMap;
use std::fmt;
use crate::cpu;

use crate::asm::err::CompileError;

pub struct CompileContext<'a> {
  pub current_pc: u16,
  pub label_map: &'a HashMap<(usize, &'a String), Option<u16>>,
  pub scope_stack: &'a Vec<usize>
}

pub trait Instruction: fmt::Debug {
  fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError>;
}

#[derive(Copy, Clone, Debug)]
pub struct CustomInstruction {
  val: u16
}

impl CustomInstruction {
  pub fn new(val: u16) -> Self {
      CustomInstruction { val }
  }
}

impl Instruction for CustomInstruction {
  fn compile(&self, _: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
      Ok(vec![
          cpu::Instruction::CUSTOM(self.val)
      ])
  }

}

#[derive(Clone, Debug)]
pub struct CustomMultiInstruction {
  val: Vec<u16>
}

impl CustomMultiInstruction {
  pub fn new(val: Vec<u16>) -> Self {
    CustomMultiInstruction { val }
  }
}

impl Instruction for CustomMultiInstruction {
  fn compile(&self, _: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
      Ok(self.val.iter().map(|x| cpu::Instruction::CUSTOM(*x)).collect())
  }

}



#[derive(Copy, Clone, Debug)]
pub struct NopInstruction {
}

#[allow(clippy::new_without_default)]
impl NopInstruction {
  pub fn new() -> Self {
      NopInstruction {  }
  }
}

impl Instruction for NopInstruction {
  fn compile(&self, _: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
      Ok(vec![
          cpu::Instruction::NOP
      ])
  }
}

pub fn compile_instructions(instructions: Vec<Box<dyn Instruction>>, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
  let compiled: Result<Vec<_>, _> = instructions.into_iter().map(|x| x.compile(ctx)).collect();
  let compiled = compiled?;
  let compiled: Vec<cpu::Instruction> = compiled.into_iter().flatten().collect();

  Ok(compiled)
}