use crate::asm::inst::{compile_instructions, CompileContext};
use crate::asm::{err::CompileError, inst::Instruction};
use crate::cpu::{self};

use crate::asm::alu::{AluInstruction, AluOperation};

use super::base::{StackBaseInstruction, StackBaseOperation};

#[derive(Copy, Clone, Debug)]
pub struct StackAluInstruction {
    pub op: AluOperation,

    pub nx: bool,
    pub ny: bool,
    pub no: bool,
}

impl StackAluInstruction {
    pub fn new(op: AluOperation) -> StackAluInstruction {
        StackAluInstruction {
            op,
            nx: false,
            ny: false,
            no: false,
        }
    }

    pub fn set_flags(&mut self, nx: bool, ny: bool, no: bool) -> &mut StackAluInstruction {
        self.nx = nx;
        self.ny = ny;
        self.no = no;
        self
    }

    pub fn set_flags_from_str(&mut self, s: &str) -> &mut StackAluInstruction {
        self.set_flags(s.contains("X"), s.contains("Y"), s.contains("O"));
        self
    }

    pub fn parse_asm(op: AluOperation, flags: &str) -> StackAluInstruction {
        *StackAluInstruction::new(op).set_flags_from_str(flags)
    }
}

impl Instruction for StackAluInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let mut ins: Vec<Box<dyn Instruction>> = Vec::new();
        

        let second_reg = match self.op.get_second_reg(cpu::Register::R2) {
            None => {
                ins.push(Box::new(StackBaseInstruction::new(
                    StackBaseOperation::POP,
                    cpu::Register::R3,
                )?));
                cpu::Register::R3
            }
            Some(r) => r,
        };

        ins.push(Box::new(StackBaseInstruction::new(
          StackBaseOperation::POP,
          cpu::Register::R2,
        )?));

        ins.push(Box::new(
            *AluInstruction::new(self.op, cpu::Register::R2, cpu::Register::R2, second_reg)
                .set_flags(self.nx, self.ny, self.no),
        ));

        ins.push(Box::new(StackBaseInstruction::new(
            StackBaseOperation::PUSH,
            cpu::Register::R2,
        )?));
        compile_instructions(ins, ctx)
    }
}
