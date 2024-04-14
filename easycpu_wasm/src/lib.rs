pub mod exec;

use wasm_bindgen::prelude::*;

use easycpu_lib::{
    asm::{self, parse_and_compile},
    cpu,
};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn compile(source: &str) -> Result<Vec<u16>, String> {
    parse_and_compile(source).map_err(|errs| {
        errs.into_iter()
            .map(|e| format!("{}: {:#?}", e.start_pos, e.error))
            .collect::<Vec<String>>()
            .join(", ")
    })
}

#[wasm_bindgen]
pub fn disassemble(assembled: Vec<u16>) -> Result<JsValue, String> {
    let dissassembled: js_sys::Array = assembled
        .into_iter()
        .map(cpu::Instruction::decode)
        .map(asm::disasm::disassemle_instruction)
        .map(JsValue::from)
        .collect();
    Ok(dissassembled.into())
}
