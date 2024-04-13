pub mod inst;
pub mod err;

#[allow(clippy::module_inception)]
pub mod compile;

pub use inst::{Instruction, CompileContext, compile_instructions};
pub use err::CompileError;