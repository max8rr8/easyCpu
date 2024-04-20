use super::{
    comp::MainCompContext,
    AtomBox, CompileContext, CompileError,
};
use crate::parser::{CompileErrorWithPos, ParsePosition, PosCompileError};

pub fn compile_program(program: Vec<AtomBox>) -> Result<Vec<u16>, Vec<PosCompileError>> {
    let mut attempts_left = 1024;

    let mut ctx = CompileContext::new();

    while attempts_left > 0 {
        if !ctx.status.reset() {
            break;
        }

        ctx.comp.reset();

        for atom in program.iter() {
            if let Err(e) = atom.compile(&mut ctx) {
                ctx.status.report_err(e);
            }
        }

        let errors = ctx.status.take_errors();
        if !errors.is_empty() {
            return Err(errors);
        }

        ctx.named_resolver.finish();

        attempts_left -= 1;
    }
    if attempts_left == 0 {
        return Err(vec![
            CompileError::TooManyAttempts.with_pos(ParsePosition::default())
        ]);
    }

    ctx.comp.as_any()
        .downcast_ref::<MainCompContext>()
        .expect("Not a mian inst context")
        .iter_instructions()
        .map(|x| x.encode())
        .collect::<Result<Vec<u16>, _>>()
        .map_err(|x| vec![CompileError::InvalidInstruction(x).with_pos(ParsePosition::default())])
}
