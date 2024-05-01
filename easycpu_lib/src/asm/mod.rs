use crate::{compile::compile_program, parser::PosCompileError};

pub mod alu;
pub mod branch;
pub mod jump;
pub mod load_const;
pub mod load_label;
pub mod mem;
pub mod pushpop;

pub mod custom;
pub mod disasm;
pub mod parse;

pub fn parse_and_compile(source: &str) -> Result<Vec<u16>, Vec<PosCompileError>> {
    compile_program(parse::parse_listing(source).map_err(|x| vec![x])?)
}
