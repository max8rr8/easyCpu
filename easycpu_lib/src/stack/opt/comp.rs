use std::collections::VecDeque;

use crate::{
    asm::{alu::AluOperation, mem::MemOperation},
    compile::{comp::CompContext, CompileError},
    cpu,
    stack::{StackExecCtx, StackOpSignature, StackOperation},
};

struct OptCompiler<'a> {
    stack_reg: VecDeque<cpu::Register>,
    comp: &'a mut dyn CompContext,
}

impl<'a> OptCompiler<'a> {
    fn alloc_free_register(&mut self, used: &[cpu::Register]) -> cpu::Register {
        let mut available = vec![
            cpu::Register::R2,
            cpu::Register::R3,
            cpu::Register::R4,
            cpu::Register::R5,
        ];
        available.retain(|x| !used.contains(x));

        for entry in self.stack_reg.iter() {
            available.retain(|x| *x != *entry);
        }

        if !available.is_empty() {
            available[0]
        } else {
            self.reset_single();
            self.alloc_free_register(used)
        }
    }

    fn push_reg_to_stack(&mut self, reg: cpu::Register) {
        self.stack_reg.push_back(reg);
    }

    fn reset_single(&mut self) {
        let entry = self.stack_reg.pop_front();

        if let Some(entry) = entry {
            self.comp.instruct(
                MemOperation::STORE
                    .instr(entry, cpu::Register::SP, 0)
                    .unwrap(),
            );

            self.comp.instruct(AluOperation::INC.instr(
                cpu::Register::SP,
                cpu::Register::SP,
                cpu::Register::ZX,
            ));
        }
    }

    fn reset_stack(&mut self) {
        while !self.stack_reg.is_empty() {
            self.reset_single();
        }
    }

    fn load_one_into_reg(&mut self, used: &[cpu::Register]) -> cpu::Register {
        let reg = self.alloc_free_register(used);

        self.comp.instruct(AluOperation::DEC.instr(
            cpu::Register::SP,
            cpu::Register::SP,
            cpu::Register::ZX,
        ));

        self.comp
            .instruct(MemOperation::LOAD.instr(reg, cpu::Register::SP, 0).unwrap());

        reg
    }

    fn compile_one(&mut self, op: &dyn StackOperation) -> Result<(), CompileError> {
        let signature = op.signature();

        let mut stack_info = StackExecCtx {
            inps: vec![],
            outs: vec![],
            temps: vec![],
        };

        let mut used = Vec::new();
        for _ in 0..signature.takes {
            let reg = self
                .stack_reg
                .pop_back()
                .unwrap_or_else(|| self.load_one_into_reg(&used));
            used.push(reg);
            stack_info.inps.push(reg);
        }
        stack_info.inps.reverse();

        if signature.check(StackOpSignature::FLAG_SAVE_STACK)
            | signature.check(StackOpSignature::FLAG_RESET_STACK)
        {
            self.reset_stack();
        }

        
        for _ in 0..signature.temps {
            let reg = self.alloc_free_register(&used);
            stack_info.temps.push(reg);
            used.push(reg);
        }
        
        let mut used = stack_info.temps.clone(); // Mark inps as usable once more
        for _ in 0..signature.pushes {
            let reg = self.alloc_free_register(&used);
            stack_info.outs.push(reg);
            used.push(reg);
        }

        op.execute(&mut stack_info, self.comp)?;

        for reg in stack_info.outs {
            self.push_reg_to_stack(reg);
        }

        Ok(())
    }
}

pub fn compile(
    ops: Vec<Box<dyn StackOperation>>,
    comp: &mut dyn CompContext,
) -> Result<(), CompileError> {
    let mut compiler = OptCompiler {
        stack_reg: VecDeque::new(),
        comp,
    };

    for op in ops.iter() {
        compiler.compile_one(op.as_ref())?;
    }

    compiler.reset_stack();

    Ok(())
}
