use crate::compile::{CompileContext, CompileError};
use crate::cpu;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct ParsedLabel {
    pub label: String,
}

impl ParsedLabel {
    pub fn resolve(&self, ctx: &mut CompileContext) -> Result<usize, CompileError> {
        ctx.named_resolver.resolve_label_id(&self.label)
    }

    pub fn resolve_pos(&self, ctx: &mut CompileContext) -> Result<u16, CompileError> {
        ctx.named_resolver
            .resolve_label_id(&self.label)
            .and_then(|id| ctx.resolve_label(id))
    }
}

pub fn parse_number(inp: &str) -> Result<i32, CompileError> {
    let (s, is_negative) = match inp.strip_prefix('-') {
        Some(s) => (s, true),
        None => (inp, false),
    };

    let (radix, s) = if let Some(s) = s.strip_prefix("0X") {
        (16, s)
    } else if let Some(s) = s.strip_prefix("0B") {
        (2, s)
    } else if let Some(s) = s.strip_prefix("0O") {
        // octal base 0o uppercase
        (8, s)
    } else {
        (10, s)
    };

    i32::from_str_radix(s, radix)
        .map_err(|_| CompileError::InvalidNumber(String::from(inp)))
        .map(|x| if is_negative { -x } else { x })
}

pub fn convert_to_u16(num: i32) -> Result<u16, CompileError> {
    let num = if num < 0 {
        (u16::MAX as i32) + num + 1 // Converts -1 to 0xffff, -2 to 0xfffe
    } else {
        num
    };

    u16::try_from(num).map_err(|_| CompileError::InvalidNumber(num.to_string()))
}

pub fn parse_u16_constant(inp: &str) -> Result<u16, CompileError> {
    let num = parse_number(inp)?;
    let num = if num < 0 {
        (u16::MAX as i32) + num + 1 // Converts -1 to 0xffff, -2 to 0xfffe
    } else {
        num
    };

    u16::try_from(num).map_err(|_| CompileError::InvalidNumber(String::from(inp)))
}

#[derive(Clone)]
pub struct ParseParts<'a> {
    vec: VecDeque<&'a str>,
}

impl<'a> From<VecDeque<&'a str>> for ParseParts<'a> {
    fn from(vec: VecDeque<&'a str>) -> Self {
        ParseParts { vec }
    }
}

impl<'a> ParseParts<'a> {
    fn pop(&mut self) -> Result<&'a str, CompileError> {
        match self.vec.pop_front() {
            Some(el) => Ok(el),
            None => Err(CompileError::NotEnoughArguments),
        }
    }

    pub fn pop_command(&mut self) -> Result<&'a str, CompileError> {
        self.pop().map_err(|_| CompileError::NoCommandSupplied)
    }

    pub fn pop_register(&mut self) -> Result<cpu::Register, CompileError> {
        let res = self.pop()?;
        match res {
            "R0" => Ok(cpu::Register::ZX),
            "R1" => Ok(cpu::Register::PC),
            "R2" => Ok(cpu::Register::R2),
            "R3" => Ok(cpu::Register::R3),
            "R4" => Ok(cpu::Register::R4),
            "R5" => Ok(cpu::Register::R5),
            "R6" => Ok(cpu::Register::LP),
            "R7" => Ok(cpu::Register::SP),

            "ZX" => Ok(cpu::Register::ZX),
            "PC" => Ok(cpu::Register::PC),
            "LP" => Ok(cpu::Register::LP),
            "SP" => Ok(cpu::Register::SP),

            _ => Err(CompileError::UnknownRegister(String::from(res))),
        }
    }

    pub fn pop_shift(&mut self) -> Result<i8, CompileError> {
        let res = self.pop()?;
        res.parse::<i8>()
            .map_err(|_| CompileError::InvalidNumber(String::from(res)))
    }

    pub fn pop_const(&mut self) -> Result<u16, CompileError> {
        let res = self.pop()?;
        parse_u16_constant(res)
    }

    pub fn pop_label(&mut self) -> Result<ParsedLabel, CompileError> {
        let res = self.pop()?;
        Ok(ParsedLabel {
            label: String::from(res),
        })
    }
}
