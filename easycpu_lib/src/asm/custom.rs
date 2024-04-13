use crate::{compile::{CompileContext, Instruction}, cpu};

use crate::compile::CompileError;


#[derive(Copy, Clone, Debug)]
pub struct CustomInstruction {
    val: u16,
}

impl CustomInstruction {
    pub fn new(val: u16) -> Self {
        CustomInstruction { val }
    }
}

impl Instruction for CustomInstruction {
    fn compile(&self, _: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        Ok(vec![cpu::Instruction::CUSTOM(self.val)])
    }
}

#[derive(Clone, Debug)]
pub struct CustomMultiInstruction {
    val: Vec<u16>,
}

impl CustomMultiInstruction {
    pub fn new(val: Vec<u16>) -> Self {
        CustomMultiInstruction { val }
    }
}

impl Instruction for CustomMultiInstruction {
    fn compile(&self, _: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        Ok(self
            .val
            .iter()
            .map(|x| cpu::Instruction::CUSTOM(*x))
            .collect())
    }
}

impl From<Vec<char>> for CustomMultiInstruction {
    fn from(source: Vec<char>) -> Self {
        let mut val: Vec<u16> = Vec::new();
        let mut is_special = false;
        for mut cur in source.into_iter() {
            if is_special {
                cur = match cur {
                    'n' => '\n',
                    't' => '\t',
                    '0' => '\0',
                    _ => cur,
                };

                is_special = false;
            } else if cur == '\\' {
                is_special = true;
            }

            if !is_special {
                let mut bu: [u8; 1] = [0];
                cur.encode_utf8(&mut bu);
                val.push(bu[0] as u16);
            }
        }
        
        CustomMultiInstruction { val }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NopInstruction {}

#[allow(clippy::new_without_default)]
impl NopInstruction {
    pub fn new() -> Self {
        NopInstruction {}
    }
}

impl Instruction for NopInstruction {
    fn compile(&self, _: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        Ok(vec![cpu::Instruction::NOP])
    }
}
