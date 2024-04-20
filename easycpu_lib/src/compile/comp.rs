use std::{rc::Rc, slice};

use crate::{cpu, stack::{compile_stackop, StackOperation}, AsAny};

use super::{status::ContextStatus, CompileError};

pub trait CompContext: AsAny {
    fn instruct(&mut self, instruction: cpu::Instruction);
    fn emit_new_label(&mut self) -> usize;
    fn emit_label(&mut self, id: usize) -> Result<(), CompileError>;
    fn resolve_label(&mut self, label_id: usize) -> Result<u16, CompileError>;

    fn stack(&mut self, op: Box<dyn StackOperation>);

    fn reset(&mut self) {}
}

#[derive(Default)]
pub struct NullCompContext {}

impl CompContext for NullCompContext {
    fn instruct(&mut self, _: cpu::Instruction) {
        panic!("Attempt to instruct null context");
    }

    fn emit_new_label(&mut self) -> usize {
        panic!("Attempt to emit label in null context");
    }

    fn emit_label(&mut self, _: usize) -> Result<(), CompileError> {
        panic!("Attempt to emit label in null context");
    }

    fn resolve_label(&mut self, _: usize) -> Result<u16, CompileError> {
        panic!("Attempt to resolve label in null context");
    }
    
    fn stack(&mut self, _: Box<dyn StackOperation>) {
        panic!("Attempt to stack in null context");
    }
}

pub struct MainCompContext {
    current_pc: u16,
    instructions: Vec<cpu::Instruction>,

    label_pos: Vec<u16>,
    status: Rc<ContextStatus>,
}

impl MainCompContext {
    pub fn new(status: Rc<ContextStatus>) -> Self {
        MainCompContext {
            current_pc: 0,
            instructions: Vec::new(),
            label_pos: Vec::new(),
            status,
        }
    }

    pub fn iter_instructions(&self) -> slice::Iter<'_, cpu::Instruction> {
        self.instructions.iter()
    }
}

impl CompContext for MainCompContext {
    fn instruct(&mut self, instruction: cpu::Instruction) {
        self.instructions.push(instruction);
        self.current_pc += 1;
    }

    fn emit_new_label(&mut self) -> usize {
        let id = self.label_pos.len();
        self.label_pos.push(self.current_pc.wrapping_add(8));
        self.status.recompile();
        id
    }

    fn emit_label(&mut self, id: usize) -> Result<(), CompileError> {
        let prev_value = self.label_pos[id];
        if prev_value != self.current_pc {
            self.label_pos[id] = self.current_pc;
            self.status.recompile();
        }

        Ok(())
    }

    fn resolve_label(&mut self, label_id: usize) -> Result<u16, CompileError> {
        if label_id == usize::MAX {
            return Ok(self.current_pc.wrapping_add(8));
        }

        let label_pos = self.label_pos[label_id];
        Ok(label_pos.wrapping_sub(self.current_pc))
    }

    fn reset(&mut self) {
        self.current_pc = 0;
        self.instructions.clear()
    }
    
    fn stack(&mut self, op: Box<dyn StackOperation>) {
        let ctx = self as &mut dyn CompContext;
        if let Err(error) = compile_stackop(ctx, op) {
            self.status.report_err(error);
        }
    }
}
