use std::collections::HashMap;

use crate::{asm::inst::CompileContext, cpu::InstructionError};

use super::{err::CompileError, parse::Parsed};

pub fn compile(parsed: Vec<Parsed>) -> Result<Vec<u16>, CompileError> {
    let mut labels: HashMap<&String, Option<u16>> = HashMap::new();

    for p in parsed.iter() {
        if let Parsed::Label(label) = p {
            if labels.contains_key(label) {
                return Err(CompileError::LabelRedefined(label.clone()));
            } else {
                labels.insert(label, None);
            }
        }
    }


    let mut compiled: Vec<u16> = Vec::new();
    let mut attempts_left = 1024;
    while attempts_left > 0 {
        let mut should_recompile = false;

        compiled.clear();

        for p in parsed.iter() {
            let current_pc = compiled.len() as u16;
            match p {
                Parsed::Instruction(ins) => {
                    let c = ins.compile(&CompileContext {
                        current_pc,
                        label_map: &labels,
                    })?;
                    let c: Result<Vec<_>, InstructionError> =
                        c.iter().map(|x| x.encode()).collect();
                    let c = c.map_err(|x| CompileError::InvalidInstruction(x))?;
                    compiled.extend(c);
                }

                Parsed::Label(label) => {
                    let label_value = labels.get(label).unwrap_or(&None);
                    let should_update = match label_value {
                        None => true,
                        Some(v) => v != &current_pc,
                    };
                    if should_update {
                        labels.insert(label, Some(current_pc));
                        should_recompile = true;
                    }
                }

                Parsed::Nop => {}
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
