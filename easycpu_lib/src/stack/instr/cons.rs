use crate::asm::alu::AluOperation;
use crate::asm::load_const::LoadConstInstruction;
use crate::compile::comp::CompContext;
use crate::compile::CompileError;
use crate::cpu::{self};
use crate::parser::ParseParts;
use crate::stack::{StackOpSignature, StackOperation};

#[derive(Copy, Clone, Debug)]
pub struct ConstStackOp {
    pub val: u16,
    pub do_add: bool,
}

impl ConstStackOp {
    pub fn new(val: u16, do_add: bool) -> Self {
        Self { val, do_add }
    }

    pub fn parse_operation(cmd: &str) -> Option<bool> {
        match cmd {
            "PCONST" => Some(false),
            "ACONST" => Some(true),
            _ => None,
        }
    }

    pub fn parse_asm(do_add: bool, mut parts: ParseParts) -> Result<Self, CompileError> {
        Ok(Self::new(parts.pop_const()?, do_add))
    }
}

impl StackOperation for ConstStackOp {
    fn signature(&self) -> StackOpSignature {
        StackOpSignature {
            takes: if self.do_add { 1 } else { 0 },
            pushes: 1,
            ..Default::default()
        }
    }

    fn execute(
        &self,
        stack: &mut crate::stack::StackExecCtx,
        comp: &mut dyn CompContext,
    ) -> Result<(), CompileError> {
        let ins = if self.do_add {
            if stack.inps[0] != stack.outs[0] {
                comp.instruct(AluOperation::MOV.instr(
                    stack.outs[0],
                    stack.inps[0],
                    cpu::Register::ZX,
                ));
            }

            LoadConstInstruction::instr_add(stack.outs[0], self.val)
        } else {
            LoadConstInstruction::instr_load(stack.outs[0], self.val)
        };

        for v in ins {
            comp.instruct(v);
        }

        Ok(())
    }

    fn duplicate(&self) -> Box<dyn StackOperation> {
        Box::new(*self)
    }
}
