use std::collections::{hash_map::Entry, HashMap};

use crate::{cpu, parser::PosCompileError};

use super::CompileError;

pub struct CompileContext {
    pub current_pc: u16,
    pub label_map: HashMap<(usize, String), Option<u16>>,
    pub scope_stack: Vec<usize>,
    pub instructions: Vec<cpu::Instruction>,
    pub cur_scope: usize,
    pub should_recompile: bool,
    pub resolving_labels: bool,
    pub errors: Vec<PosCompileError>
}

impl CompileContext {
    pub fn instruct(&mut self, instruction: cpu::Instruction) {
        self.instructions.push(instruction);
        self.current_pc += 1;
    }

    pub fn patch_instruct(&mut self, pc: u16, instruction: cpu::Instruction) {
        self.instructions[pc as usize] = instruction;
    }

    pub fn emit_label(&mut self, label: &String) -> Result<(), CompileError> {
        if self.resolving_labels {
            let key = (self.cur_scope, label.to_owned());
            if let Entry::Vacant(e) = self.label_map.entry(key) {
                e.insert(None);
            } else {
                return Err(CompileError::LabelRedefined(label.clone()));
            }
            return Ok(());
        }

        let key = (self.cur_scope, label.to_owned());
        let label_value = self.label_map.get(&key).unwrap_or(&None);
        let should_update = match label_value {
            None => true,
            Some(v) => v != &self.current_pc,
        };

        if should_update {
            self.label_map.insert(key, Some(self.current_pc));
            self.should_recompile = true;
        }

        Ok(())
    }

    pub fn enter_local_scope(&mut self, id: usize) {
        self.scope_stack.push(id);
        self.cur_scope = id;
    }

    pub fn leave_local_scope(&mut self) {
        self.scope_stack.pop();
        self.cur_scope = *self.scope_stack.last().unwrap_or(&0);
    }
}
