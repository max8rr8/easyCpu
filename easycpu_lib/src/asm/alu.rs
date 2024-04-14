use crate::compile::CompileError;
use crate::parser::ParseParts;
use crate::cpu;

use crate::compile::{CompileContext, Atom};

#[derive(Copy, Clone, Debug)]
pub enum AluOperation {
    ADD,
    SUB,
    AND,
    OR,
    NAND,
    NOR,
    NEG,
    NOT,
    MOV,
    INC,
    DEC,
    SHR,
    SHL
}

#[derive(Copy, Clone, Debug)]
pub struct AluInstruction {
    pub op: AluOperation,

    pub nx: bool,
    pub ny: bool,
    pub no: bool,

    pub dst: cpu::Register,
    pub src_a: cpu::Register,
    pub src_b: cpu::Register,
}

impl AluOperation {
    pub fn get_second_reg(&self, first_reg: cpu::Register) -> Option<cpu::Register> {
        match self {
            AluOperation::NEG | AluOperation::NOT | AluOperation::MOV => Some(cpu::Register::ZX),
            AluOperation::INC | AluOperation::DEC => Some(cpu::Register::ZX),
            AluOperation::SHL | AluOperation::SHR => Some(first_reg),
            _ => None,
        }
    }

    fn get_flags(&self) -> (bool, bool, bool) {
        match self {
            AluOperation::ADD | AluOperation::AND | AluOperation::MOV | AluOperation::SHL => (false, false, false),
            AluOperation::NEG | AluOperation::NOT => (false, true, true),

            AluOperation::SUB => (true, false, true),
            AluOperation::OR => (true, true, true),

            AluOperation::NAND => (false, false, true),
            AluOperation::NOR => (true, true, false),

            AluOperation::INC => (true, true, true),
            AluOperation::DEC => (false, true, false),
        
            AluOperation::SHR => (true, false, false),
            
        }
    }

    fn match_instruction(&self, ins: cpu::AluInstruction) -> cpu::Instruction {
        match self {
            AluOperation::ADD
            | AluOperation::SUB
            | AluOperation::NEG
            | AluOperation::MOV
            | AluOperation::INC
            | AluOperation::DEC
            | AluOperation::SHL => cpu::Instruction::ADD(ins),

            AluOperation::AND
            | AluOperation::OR
            | AluOperation::NAND
            | AluOperation::NOR
            | AluOperation::NOT
            | AluOperation::SHR => cpu::Instruction::AND(ins),
        }
    }

    pub fn parse_operation(s: &str) -> Option<AluOperation> {
        match s {
            "ADD" => Some(AluOperation::ADD),
            "SUB" => Some(AluOperation::SUB),
            "AND" => Some(AluOperation::AND),
            "OR" => Some(AluOperation::OR),
            "NAND" => Some(AluOperation::NAND),
            "NOR" => Some(AluOperation::NOR),
            "NEG" => Some(AluOperation::NEG),
            "NOT" => Some(AluOperation::NOT),
            "MOV" => Some(AluOperation::MOV),
            "INC" => Some(AluOperation::INC),
            "DEC" => Some(AluOperation::DEC),
            "SHL" => Some(AluOperation::SHL),
            "SHR" => Some(AluOperation::SHR),
            _ => None,
        }
    }

    pub fn instr(&self, dst: cpu::Register, src_a: cpu::Register, src_b: cpu::Register) -> cpu::Instruction {
        let src_b = match self.get_second_reg(src_a) {
            Some(second_override) => second_override,
            None => src_b,
        };
        
        let (op_x, op_y, op_o) = self.get_flags();

        let ins = cpu::AluInstruction {
            dst, src_a, src_b,
            nx: op_x,
            ny: op_y,
            no: op_o,
        };

        self.match_instruction(ins)
    }
}

impl AluInstruction {
    pub fn new(
        op: AluOperation,
        dst: cpu::Register,
        src_a: cpu::Register,
        src_b: cpu::Register,
    ) -> AluInstruction {
        let src_b = match op.get_second_reg(src_a) {
            Some(second_override) => second_override,
            None => src_b,
        };
        AluInstruction {
            op,
            nx: false,
            ny: false,
            no: false,
            dst,
            src_a,
            src_b,
        }
    }

    pub fn set_flags(&mut self, nx: bool, ny: bool, no: bool) -> &mut AluInstruction {
        self.nx = nx;
        self.ny = ny;
        self.no = no;
        self
    }

    pub fn set_flags_from_str(&mut self, s: &str) -> &mut AluInstruction {
        self.set_flags(s.contains('X'), s.contains('Y'), s.contains('O'));
        self
    }

    pub fn parse_asm(
        op: AluOperation,
        flags: &str,
        mut parts: ParseParts,
    ) -> Result<AluInstruction, CompileError> {
        let dst = parts.pop_register()?;
        let src_a = parts.pop_register()?;
        let src_b = match op.get_second_reg(src_a) {
            Some(r) => r,
            None => parts.pop_register()?,
        };
        // parts.pop_front().unwrap();
        // let dst = cpu::Register::ZX;
        // let src_a = cpu::Register::ZX;
        // let src_b = cpu::Register::ZX;

        let mut ins = AluInstruction::new(op, dst, src_a, src_b);
        ins.set_flags_from_str(flags);
        Ok(ins)
    }
}

impl Atom for AluInstruction {
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let (op_x, op_y, op_o) = self.op.get_flags();

        let ins = cpu::AluInstruction {
            dst: self.dst,
            src_a: self.src_a,
            src_b: self.src_b,
            nx: self.nx ^ op_x,
            ny: self.ny ^ op_y,
            no: self.no ^ op_o,
        };

        ctx.instruct(self.op.match_instruction(ins));

        Ok(())
    }
}
