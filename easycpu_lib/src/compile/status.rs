use std::cell::RefCell;

use crate::parser::{ParsePosition, PosCompileError};

use super::CompileError;

#[derive(Debug)]
pub struct ContextStatus {
    errors: RefCell<Vec<PosCompileError>>,
    should_recompile: RefCell<bool>,

    pos: RefCell<(ParsePosition, ParsePosition)>,
}

impl ContextStatus {
    pub fn new() -> Self {
        ContextStatus {
            errors: RefCell::new(Vec::new()),
            should_recompile: RefCell::new(true),
            pos: RefCell::default(),
        }
    }

    pub fn reset(&self) -> bool {
        self.should_recompile.replace(false)
    }

    pub fn recompile(&self) {
        self.should_recompile.replace(true);
    }

    pub fn report_err(&self, error: CompileError) {
        let (start_pos, end_pos) = *self.pos.borrow();

        self.errors.borrow_mut().push(PosCompileError {
            error,
            start_pos,
            end_pos,
        });
    }

    pub fn take_errors(&self) -> Vec<PosCompileError> {
        self.errors.take()
    }

    pub fn swap_pos(
        &self,
        new_pos: (ParsePosition, ParsePosition),
    ) -> (ParsePosition, ParsePosition) {
        self.pos.replace(new_pos)
    }
}
