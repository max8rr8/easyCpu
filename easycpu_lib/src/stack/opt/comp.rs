use std::collections::VecDeque;

use crate::{
    asm::{alu::AluOperation, load_const::LoadConstInstruction, mem::MemOperation},
    compile::{comp::CompContext, CompileError},
    cpu,
    stack::{StackExecCtx, StackOpSignature, StackOperation},
};

struct OptCompiler<'a> {
    stack_reg: VecDeque<cpu::Register>,
    spec_saved: VecDeque<cpu::Register>,
    sp_shift: i8,
    comp: &'a mut dyn CompContext,
}

impl<'a> OptCompiler<'a> {
    fn new(comp: &'a mut dyn CompContext) -> Self {
        OptCompiler {
            stack_reg: VecDeque::new(),
            spec_saved: VecDeque::new(),
            sp_shift: 0,
            comp,
        }
    }

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

        for entry in self.spec_saved.iter() {
            available.retain(|x| *x != *entry);
        }

        if !available.is_empty() {
            available[0]
        } else {
            if self.spec_saved.pop_front().is_none() {
                self.save_single();
            }
            self.alloc_free_register(used)
        }
    }

    fn push_reg_to_stack(&mut self, reg: cpu::Register) {
        self.stack_reg.push_back(reg);
    }

    fn ensure_sp(&mut self, min: i8, max: i8) {
        while self.sp_shift < min {
            self.comp.instruct(AluOperation::DEC.instr(
                cpu::Register::SP,
                cpu::Register::SP,
                cpu::Register::ZX,
            ));
            self.sp_shift += 1;
        }

        while self.sp_shift > max {
            self.comp.instruct(AluOperation::INC.instr(
                cpu::Register::SP,
                cpu::Register::SP,
                cpu::Register::ZX,
            ));
            self.sp_shift -= 1;
        }
    }

    fn save_single(&mut self) {
        let entry = self.stack_reg.pop_front();

        if let Some(entry) = entry {
            self.ensure_sp(-3, 3);
            self.comp.instruct(
                MemOperation::STORE
                    .instr(entry, cpu::Register::SP, self.sp_shift)
                    .unwrap(),
            );
            self.spec_saved.push_back(entry);
            self.sp_shift += 1;
        }
    }

    fn save_stack(&mut self, not_care_sp: bool) {
        while !self.stack_reg.is_empty() {
            self.save_single();
        }
        if !not_care_sp {   
            self.ensure_sp(0, 0);
        } else {
            self.sp_shift = 0;
        }
    }

    fn load_one_into_reg(&mut self, used: &[cpu::Register], actually_load: bool) -> cpu::Register {
        self.sp_shift -= 1;

        if let Some(reg) = self.spec_saved.pop_back() {
            return reg;
        }

        if !actually_load {
            return cpu::Register::ZX;
        }

        let reg = self.alloc_free_register(used);
        self.ensure_sp(-3, 3);
        self.comp.instruct(
            MemOperation::LOAD
                .instr(reg, cpu::Register::SP, self.sp_shift)
                .unwrap(),
        );

        reg
    }

    fn load_peek(&mut self, mut peek: u8) -> cpu::Register {
        for reg in self.stack_reg.iter().rev() {
            peek -= 1;
            if peek == 0 {
                return *reg;
            }
        }
        
        let peek = self.sp_shift - peek as i8;

        let reg = self.alloc_free_register(&[]);
        if peek >= -3 {
            self.comp.instruct(
                MemOperation::LOAD
                    .instr(reg, cpu::Register::SP, peek)
                    .expect("Invalid mem"),
            );
        } else {
            self.comp
                .instruct(AluOperation::MOV.instr(reg, cpu::Register::SP, cpu::Register::ZX));

            LoadConstInstruction::instr_add(reg, 0u16.wrapping_add_signed(peek.into()))
                .into_iter()
                .for_each(|i| self.comp.instruct(i));

            self.comp
                .instruct(MemOperation::LOAD.instr(reg, reg, 0).expect("Invalid mem"));
        }

        reg
    }

    fn compile_one(&mut self, op: &dyn StackOperation) -> Result<(), CompileError> {
        let signature = op.signature();

        let mut stack_info = StackExecCtx {
            inps: vec![],
            outs: vec![],
            temps: vec![],
            peek: None,
        };

        let really_load = signature.pushes != 0 || signature.check(StackOpSignature::FLAG_IMPURE);
        let mut used = Vec::new();
        for _ in 0..signature.takes {
            let reg = self
                .stack_reg
                .pop_back()
                .unwrap_or_else(|| self.load_one_into_reg(&used, really_load));
            used.push(reg);
            stack_info.inps.push(reg);
        }
        stack_info.inps.reverse();

        if signature.check(StackOpSignature::FLAG_SAVE_STACK) {
            self.save_stack(signature.check(StackOpSignature::FLAG_NOT_CARE_SP));
            if signature.check(StackOpSignature::FLAG_RESET_STACK) {
                self.spec_saved.clear();
            }
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

        if let Some(peek) = signature.peeks {
            stack_info.peek = Some(self.load_peek(peek));
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
    let mut compiler = OptCompiler::new(comp);

    for op in ops.iter() {
        compiler.compile_one(op.as_ref())?;
    }

    compiler.save_stack(false);

    Ok(())
}

pub fn compile_stackop(
    comp: &mut dyn CompContext,
    op: Box<dyn StackOperation>,
) -> Result<(), CompileError> {
    let mut compiler = OptCompiler::new(comp);
    compiler.compile_one(op.as_ref())?;
    compiler.save_stack(false);
    Ok(())
}
