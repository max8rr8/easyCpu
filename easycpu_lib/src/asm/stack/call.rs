use crate::asm::jump::{JumpInstruction, JumpOperation};
use crate::compile::CompileError;
use crate::compile::{CompileContext, Instruction};
use crate::cpu::{self};
use crate::parser::{ParseParts, ParsedLabel};

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
    fn compile(&self, ctx: &mut CompileContext) -> Result<(), CompileError> {
        let retu_label_s = String::from("RET_ADDR");
        let retu_label = ParsedLabel {
            label: retu_label_s.clone(),
        };

        let mut cur_pc = ctx.current_pc + 8;
        let mut attempts_left = 128;

        while attempts_left > 0 {
            let mut cur_hashmap = ctx.label_map.clone();
            cur_hashmap.insert((0xffff, retu_label_s.clone()), Some(cur_pc));
            let mut new_stack = ctx.scope_stack.clone();
            new_stack.push(0xffff);

            let mut localctx = CompileContext {
                current_pc: ctx.current_pc,
                label_map: cur_hashmap,
                scope_stack: new_stack,
                instructions: Vec::new(),
            };

            StackConstInstruction::new_label(retu_label.clone()).compile(&mut localctx)?;
            JumpInstruction::new(JumpOperation::JMP, self.targ.clone(), cpu::Register::ZX)
                .compile(&mut localctx)?;

            if localctx.current_pc == cur_pc {
                for ins in localctx.instructions.into_iter() {
                    ctx.instruct(ins);
                }
                
                return Ok(());
            }
            cur_pc = localctx.current_pc;
            attempts_left -= 1;
        }

        panic!("Failed to compile return");
    }
}
