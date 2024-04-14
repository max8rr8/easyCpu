use crate::parser::{CompileErrorWithPos, ParsePosition, PosCompileError};
use std::collections::HashMap;

use super::{AtomBox, CompileContext, CompileError};

pub fn compile_program(program: Vec<AtomBox>) -> Result<Vec<u16>, Vec<PosCompileError>> {
    let mut attempts_left = 1024;

    let mut ctx = CompileContext {
        current_pc: 0,
        label_map: HashMap::new(),
        scope_stack: Vec::new(),
        instructions: Vec::new(),
        cur_scope: 0,
        should_recompile: false,
        resolving_labels: true,
        errors: Vec::new(),
    };

    while attempts_left > 0 {
        ctx.should_recompile = false;

        ctx.instructions.clear();
        ctx.scope_stack.clear();
        ctx.current_pc = 0;

        ctx.cur_scope = 0;
        ctx.scope_stack.push(0);

        for atom in program.iter() {
            if let Err(e) = atom.compile(&mut ctx) {
                ctx.errors.push(e.with_pos(ParsePosition::default()));
            }
        }

        if !ctx.errors.is_empty() {
            return Err(ctx.errors);
        }

        if ctx.resolving_labels {
            ctx.resolving_labels = false;
            continue;
        }

        if !ctx.should_recompile {
            break;
        }
        attempts_left -= 1;
    }
    if attempts_left == 0 {
        return Err(vec![
            CompileError::TooManyAttempts.with_pos(ParsePosition::default())
        ]);
    }

    ctx.instructions
        .into_iter()
        .map(|x| x.encode())
        .collect::<Result<Vec<u16>, _>>()
        .map_err(|x| vec![CompileError::InvalidInstruction(x).with_pos(ParsePosition::default())])
}
