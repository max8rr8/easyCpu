pub mod instr;
pub mod optatom;
pub mod stackop;

pub use optatom::StackOptAtom;
pub use stackop::{
    compile_stackop, StackExecCtx, StackOpInstruction, StackOpSignature, StackOperation,
};
