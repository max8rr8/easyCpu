use easycpu_lib::parser::ParsePosition;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl From<ParsePosition> for Position {
    fn from(value: ParsePosition) -> Self {
        Position {
            pos: value.pos,
            line: value.line + 1,
            column: value.line_pos + 1,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CompileError {
    err: easycpu_lib::compile::CompileError,
    pub start: Position,
    pub end: Position,
}

#[wasm_bindgen]
impl CompileError {
    pub fn get_message(&self) -> String {
        format!("{:#?}", self.err)
    }
}

#[wasm_bindgen]
pub fn compile(source: &str) -> Result<Vec<u16>, Array> {
    easycpu_lib::asm::parse_and_compile(source).map_err(|errs| {
        Array::from_iter(errs.into_iter().map(|e| {
            JsValue::from(CompileError {
                err: e.error,
                start: e.start_pos.into(),
                end: e.end_pos.into(),
            })
        }))
    })
}
