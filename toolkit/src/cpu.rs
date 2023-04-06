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

impl From<u16> for Register {
    fn from(e: u16) -> Self {
        match e & 7 {
            0 => Register::ZX,
            1 => Register::PC,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::LP,
            7 => Register::SP,
            _ => panic!("WHAT?")
        }
    }
}

impl ToString for Register {
    fn to_string(&self) -> String {
        match &self {
            Register::ZX => String::from("ZX"),
            Register::PC => String::from("PC"),
            Register::R2 => String::from("R2"),
            Register::R3 => String::from("R3"),
            Register::R4 => String::from("R4"),
            Register::R5 => String::from("R5"),
            Register::LP => String::from("LP"),
            Register::SP => String::from("SP"),
        }
    }
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

    fn decode(ins: u16) -> Self {
        AluInstruction {
            nx: (ins >> 11) & 1 == 1,
            ny: (ins >> 10) & 1 == 1,
            no: (ins >> 9) & 1 == 1,
            
            dst: Register::from((ins >> 6) & 7),
            src_a: Register::from((ins >> 3) & 7),
            src_b: Register::from((ins >> 0) & 7),
        }
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

    fn decode(ins: u16) -> Self {
        let mut shift: i8 = (ins & 3).try_into().unwrap_or(0);
        if (ins & 4) == 4 {
            shift = -shift;
        }

        MemInstruction {
            hi: (ins >> 11) & 1 == 1,
            lo: (ins >> 10) & 1 == 1,
            sw: (ins >> 9) & 1 == 1,
            
            dst: Register::from((ins >> 6) & 7),
            addr: Register::from((ins >> 3) & 7),
            shift,
        }
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

    fn decode(ins: u16) -> Self {
        let mut shift: i8 = (ins & 0x1f).try_into().unwrap_or(0);
        if (ins & 0x20) == 0x20 {
            shift = -shift;
        }

        BranchInstruction {
            eq: (ins >> 11) & 1 == 1,
            gt: (ins >> 10) & 1 == 1,
            lt: (ins >> 9) & 1 == 1,
            
            cond: Register::from((ins >> 6) & 7),
            shift,
        }
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

    pub fn decode(ins: u16) -> Self {
        let command_type = ins >> 12;
        match command_type {
            0b0000 => {
                if ins == 0 {
                    Instruction::NOP
                } else {
                    Instruction::CUSTOM(ins)
                }
            }

            0b0100 => Instruction::AND(AluInstruction::decode(ins)),
            0b0101 => Instruction::ADD(AluInstruction::decode(ins)),

            0b0010 => Instruction::LOAD(MemInstruction::decode(ins)),
            0b0011 => Instruction::STORE(MemInstruction::decode(ins)),

            0b0001 => Instruction::BRANCH(BranchInstruction::decode(ins)),

            _ => Instruction::CUSTOM(ins),
        }
    }
}
