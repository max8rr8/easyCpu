use crate::cpu;

#[derive(Debug)]
pub enum CompileError {
    NotEnoughArguments,
    NoCommandSupplied,

    UnknownCommand(String),
    UnknownRegister(String),
    UnknownLabel(String),

    ShiftIsTooBig(i8),

    InvalidInstruction(cpu::InstructionError),
    
    TooManyAttempts,
    LabelRedefined(String),

    UnexpectedEndOfFile,
    UnknownToken(char),
    InvalidNumber(String),

    UnmatchedClosingBracket
}
