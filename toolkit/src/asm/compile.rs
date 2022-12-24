use std::collections::HashMap;

use crate::{asm::inst::CompileContext, cpu::InstructionError};

use super::{err::CompileError, parse::Parsed};

pub fn compile(parsed: Vec<Parsed>) -> Result<Vec<u16>, CompileError> {
    let mut labels: HashMap<(usize, &String), Option<u16>> = HashMap::new();

    let mut scope_stack: Vec<usize> = vec![0];
    let mut cur_scope: usize = 0;

    for p in parsed.iter() {
        match p {
            Parsed::Label(label) => {
                let key = (cur_scope, label);
                if labels.contains_key(&key) {
                    return Err(CompileError::LabelRedefined(label.clone()));
                } else {
                    labels.insert(key, None);
                }
            }

            Parsed::EnterLocalScope(id) => {
                scope_stack.push(*id);
                cur_scope = *id;
            }

            Parsed::LeaveLocalScope => {
                scope_stack.pop();
                cur_scope = *scope_stack.last().unwrap_or(&0);
            }

            _ => {}
        }
    }

    let mut compiled: Vec<u16> = Vec::new();
    let mut attempts_left = 1024;
    while attempts_left > 0 {
        let mut should_recompile = false;

        compiled.clear();
        scope_stack.clear();
        
        cur_scope = 0;
        scope_stack.push(0);

        for p in parsed.iter() {
            let current_pc = compiled.len() as u16;
            match p {
                Parsed::Instruction(ins) => {
                    let c = ins.compile(&CompileContext {
                        current_pc,
                        label_map: &labels,
                        scope_stack: &mut scope_stack,
                    })?;
                    let c: Result<Vec<_>, InstructionError> =
                        c.iter().map(|x| x.encode()).collect();
                    let c = c.map_err(|x| CompileError::InvalidInstruction(x))?;
                    compiled.extend(c);
                }

                Parsed::Label(label) => {
                    let key = (cur_scope, label);
                    let label_value = labels.get(&key).unwrap_or(&None);
                    let should_update = match label_value {
                        None => true,
                        Some(v) => v != &current_pc,
                    };
                    if should_update {
                        labels.insert(key, Some(current_pc));
                        should_recompile = true;
                    }
                }

                Parsed::Nop => {}

                Parsed::EnterLocalScope(id) => {
                    scope_stack.push(*id);
                    cur_scope = *id;
                }

                Parsed::LeaveLocalScope => {
                    scope_stack.pop();
                    cur_scope = *scope_stack.last().unwrap_or(&0);
                }
            };
        }

        // dbg!("Compiled:", 1024 - attempts_left, &labels);

        if !should_recompile {
            break;
        }
        attempts_left -= 1;
    }
    if attempts_left == 0 {
        return Err(CompileError::TooManyAttempts);
    }

    Ok(compiled)
}
