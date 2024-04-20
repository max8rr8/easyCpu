use crate::cpu;

#[derive(Debug, Clone)]
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
    UnexpectedToken(char),
    UnknownToken(char),
    InvalidNumber(String),

    UnmatchedClosingBracket,
    UnknownModifier(String),
    
    InstructionInStackopt,
}
