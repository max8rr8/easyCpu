pub mod exec;

use wasm_bindgen::prelude::*;

use easycpu_lib::{asm::{self, err::CompileError, parse::{parse_listing, Atom}}, cpu, parser::ParsePosition};

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
pub fn compile(listing: &str) -> Result<Vec<u16>, String> {
    let program = parse_listing(listing);
    let program = program.map_err(|x| format!("Failed to parse: {:#?}", x))?;

    let mut errors: Vec<(ParsePosition, ParsePosition, CompileError)> = Vec::new();
    let mut parsed: Vec<Atom> = Vec::new();
    
    for atom in program {
        match atom.compiled {
            Ok(c) => parsed.push(c),
            Err(e) => errors.push((atom.start_pos, atom.end_pos, e)),
        }
    }

    if !errors.is_empty() {
        let s: Vec<String> = errors
            .iter()
            .map(|(start_pos, _end_pos, err)| format!("Line {}: {:#?}", start_pos, err))
            .collect();
        return Err(s.join("\n"));
    }

    let compiled = asm::compile::compile(parsed).map_err(|e| format!("Error: {:#?}", e))?;

    Ok(compiled)
}

#[wasm_bindgen]
pub fn disassemble(assembled: Vec<u16>) -> Result<JsValue, String> {
    let dissassembled: js_sys::Array = assembled.into_iter()
        .map(cpu::Instruction::decode)
        .map(asm::disasm::disassemle_instruction)
        .map(JsValue::from)
        .collect();
    Ok(dissassembled.into())
}
