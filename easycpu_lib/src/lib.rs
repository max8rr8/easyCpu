#![allow(clippy::new_without_default)]

pub mod cpu;
pub mod exec;
pub mod asm;
pub mod parser;
pub mod compile;

pub(crate) mod asany;
pub mod stack;
pub(crate) use asany::AsAny;