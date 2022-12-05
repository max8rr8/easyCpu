#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    ZX = 0,
    PC = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    LP = 6,
    SP = 7,
}

#[derive(Copy, Clone, Debug)]
pub struct AluInstruction {
    pub nx: bool,
    pub ny: bool,
    pub no: bool,

    pub dst: Register,
    pub src_a: Register,
    pub src_b: Register,
}

#[derive(Copy, Clone, Debug)]
pub struct MemInstruction {
    pub hi: bool,
    pub lo: bool,
    pub sw: bool,

    pub dst: Register,
    pub addr: Register,
    pub shift: i8,
}

#[derive(Copy, Clone, Debug)]
pub struct BranchInstruction {
    pub eq: bool,
    pub gt: bool,
    pub lt: bool,

    pub cond: Register,
    pub shift: i8,
}


#[derive(Copy, Clone, Debug)]
#[allow(non_snake_case)]
pub enum Instruction {
    NOP,
    AND(AluInstruction),
    ADD(AluInstruction),
    LOAD(MemInstruction),
    STORE(MemInstruction),
    BRANCH(BranchInstruction),
    CUSTOM(u16),
}

#[derive(Debug)]
pub enum InstructionError {
    InvalidShift,
}


impl AluInstruction {
    fn encode(&self) -> u16 {
        let mut res = 0;

        res |= (self.nx as u16) << 11;
        res |= (self.ny as u16) << 10;
        res |= (self.no as u16) << 9;

        res |= (self.dst as u16) << 6;
        res |= (self.src_a as u16) << 3;
        res |= (self.src_b as u16) << 0;

        res
    }
}

impl MemInstruction {
    fn encode(&self) -> u16 {
        let mut res = 0;

        res |= (self.hi as u16) << 11;
        res |= (self.lo as u16) << 10;
        res |= (self.sw as u16) << 9;

        res |= (self.dst as u16) << 6;
        res |= (self.addr as u16) << 3;
        res |= (self.shift.is_negative() as u16) << 2;
        res |= (self.shift.abs() as u16) << 0;

        res
    }

    fn validate(&self) -> Result<(), InstructionError> {
        if self.shift.abs() >= 4 {
            return Err(InstructionError::InvalidShift);
        }
        Ok(())
    }
}

impl BranchInstruction {
    fn encode(&self) -> u16 {
        let mut res = 0;

        res |= (self.eq as u16) << 11;
        res |= (self.gt as u16) << 10;
        res |= (self.lt as u16) << 9;

        res |= (self.cond as u16) << 6;
        res |= (self.shift.is_negative() as u16) << 5;
        res |= (self.shift.abs() as u16) << 0;

        res
    }

    fn validate(&self) -> Result<(), InstructionError> {
        if self.shift.abs() > 31 {
            return Err(InstructionError::InvalidShift);
        }
        Ok(())
    }
}

impl Instruction {
    fn encode_unsafe(&self) -> u16 {
        match self {
            Instruction::NOP => 0,
            Instruction::AND(ins) => 0b0100_0000_0000_0000 | ins.encode(),
            Instruction::ADD(ins) => 0b0101_0000_0000_0000 | ins.encode(),
            Instruction::LOAD(ins) => 0b0010_0000_0000_0000 | ins.encode(),
            Instruction::STORE(ins) => 0b0011_0000_0000_0000 | ins.encode(),
            Instruction::BRANCH(ins) => 0b0001_0000_0000_0000 | ins.encode(),
            Instruction::CUSTOM(ins) => *ins,
        }
    }

    pub fn validate(&self) -> Result<(), InstructionError> {
        match self {
            Instruction::NOP => Ok(()),
            Instruction::AND(_) | Instruction::ADD(_) => Ok(()),
            Instruction::LOAD(ins) | Instruction::STORE(ins) => ins.validate(),
            Instruction::BRANCH(ins) => ins.validate(),
            Instruction::CUSTOM(_) => Ok(()),
        }
    }

    pub fn encode(&self) -> Result<u16, InstructionError> {
        self.validate()?;
        Ok(self.encode_unsafe())
    }
}
