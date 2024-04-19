use crate::{cpu, parser::PosCompileError};

use super::{label::LabelResolver, CompileError};

pub struct CompileContext {
    pub current_pc: u16,
    pub instructions: Vec<cpu::Instruction>,

    pub named_resolver: Box<LabelResolver>,
    label_pos: Vec<u16>,

    pub should_recompile: bool,
    pub errors: Vec<PosCompileError>,
}

impl CompileContext {
    pub fn new() -> Self {
        CompileContext {
            current_pc: 0,
            named_resolver: Box::new(LabelResolver::new()),
            label_pos: Vec::new(),
            instructions: Vec::new(),
            should_recompile: false,
            errors: Vec::new(),
        }
    }

    pub fn instruct(&mut self, instruction: cpu::Instruction) {
        self.instructions.push(instruction);
        self.current_pc += 1;
    }

    pub fn patch_instruct(&mut self, pc: u16, instruction: cpu::Instruction) {
        self.instructions[pc as usize] = instruction;
    }

    pub fn emit_new_label(&mut self) -> usize {
        let id = self.label_pos.len();
        self.label_pos.push(self.current_pc.wrapping_add(8));
        self.should_recompile = true;
        id
    }

    pub fn emit_label(&mut self, id: usize) -> Result<(), CompileError> {
        let prev_value = self.label_pos[id];
        if prev_value != self.current_pc {
            self.label_pos[id] = self.current_pc;
            self.should_recompile = true;
        }

        Ok(())
    }

    pub fn resolve_label(&mut self, label_id: usize) -> Result<u16, CompileError> {
        if label_id == usize::MAX {
            return Ok(self.current_pc.wrapping_add(8));
        }

        let label_pos = self.label_pos[label_id];
        Ok(label_pos.wrapping_sub(self.current_pc))
    }
}
