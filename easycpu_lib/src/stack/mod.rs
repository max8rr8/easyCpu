pub mod instr;
pub mod optatom;
pub mod stackop;
pub mod opt;

pub use optatom::StackOptAtom;
pub use stackop::{StackExecCtx, StackOpInstruction, StackOpSignature, StackOperation};
pub use opt::comp::compile_stackop;
