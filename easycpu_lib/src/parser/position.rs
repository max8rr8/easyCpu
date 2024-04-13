use std::fmt::Display;

use crate::asm::err::CompileError;


#[derive(Clone, Copy, Debug, Default)]
pub struct ParsePosition {
    pub pos: usize,
    pub line: usize,
    pub line_pos: usize,
}

impl ParsePosition {
    pub fn next(&mut self, ch: char) {
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.line_pos = 0;
        } else {
            self.line_pos += 1;
        }
    }
}

impl Display for ParsePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self.line + 1;
        let line_pos = self.line_pos + 1;
        write!(f, "{line}:{line_pos}")
    }
}

#[derive(Debug)]
pub struct PosCompileError {
    pub error: CompileError,
    pub start_pos: ParsePosition,
    pub end_pos: ParsePosition
}

pub trait CompileErrorWithPos {
  fn with_pos(self, pos: ParsePosition) -> PosCompileError;
  fn with_range(self, start: ParsePosition, end: ParsePosition) -> PosCompileError;
}

impl CompileErrorWithPos for CompileError {
  fn with_pos(self, pos: ParsePosition) -> PosCompileError {
    PosCompileError { 
      error: self,
      start_pos: pos,
      end_pos: pos,
    }
  }

  fn with_range(self, start: ParsePosition, end: ParsePosition) -> PosCompileError {
    PosCompileError { 
      error: self,
      start_pos: start,
      end_pos: end,
    }
  }
}