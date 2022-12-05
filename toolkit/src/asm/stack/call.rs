use std::collections::HashMap;

use crate::asm::inst::CompileContext;
use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::asm::parse_parts::ParsedLabel;
use crate::asm::{err::CompileError, inst::Instruction, parse_parts::ParseParts};
use crate::cpu::{self};

use super::cons::StackConstInstruction;

#[derive(Clone, Debug)]
pub struct StackCallInstruction {
    pub targ: ParsedLabel,
}

impl StackCallInstruction {
    pub fn new(targ: ParsedLabel) -> Result<StackCallInstruction, CompileError> {
        Ok(StackCallInstruction { targ })
    }

    pub fn parse_asm(mut parts: ParseParts) -> Result<StackCallInstruction, CompileError> {
        StackCallInstruction::new(parts.pop_label()?)
    }
}

impl Instruction for StackCallInstruction {
    fn compile(&self, ctx: &CompileContext) -> Result<Vec<cpu::Instruction>, CompileError> {
        let retu_label_s = String::from("RET_ADDR");
        let retu_label = ParsedLabel {
            label: retu_label_s.clone(),
        };

        let mut cur_len = 16;
        let mut cur_hashmap = HashMap::new();
        let mut ins: Vec<cpu::Instruction> = Vec::new();
        let mut attempts_left = 128;

        while attempts_left > 0 {
            ins.clear();
            cur_hashmap.insert(&retu_label_s, Some(cur_len));

            ins.extend(
                StackConstInstruction::new_label(retu_label.clone()).compile(&CompileContext {
                    current_pc: 0,
                    label_map: &cur_hashmap,
                })?,
            );

            ins.extend(
                JumpInstruction::new(JumpOperation::JMP, self.targ.clone(), cpu::Register::ZX)
                    .compile(&CompileContext {
                        current_pc: ctx.current_pc + ins.len() as u16,
                        label_map: ctx.label_map,
                    })?,
            );

            if ins.len() as u16 == cur_len {
                break;
            }
            cur_len = ins.len() as u16;
            attempts_left -= 1;
        }
        Ok(ins)
    }
}
