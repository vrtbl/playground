use wasm_bindgen::prelude::*;
use passerine::{
    compiler::{lex, parse, desugar, hoist, gen::gen_with_ffi},
    common::{
        source::Source,
        closure::Closure,
        data::Data
    },
    vm::vm::VM,
    core::{ffi_core, ffi::{FFIFunction, FFI}},
};

#[wasm_bindgen]
extern {
    pub fn js_alert(s: &str);
}

pub fn alert(data: Data) -> Result<Data, String> {
    let string = format!("{}", data);
    js_alert(&string);
    return Ok(data);
}

pub fn js_ffi() -> FFI {
    let mut ffi = ffi_core();
    // replace println!(...) with alert for now...
    ffi.add("print",   FFIFunction::new(Box::new(alert))).unwrap_err();
    ffi.add("println", FFIFunction::new(Box::new(alert))).unwrap_err();
    return ffi;
}

/// Compiles and runs some Passerine, represented as a string.
/// Returns `JsValue::null()` if the program ran with no errors,
/// Otherwise the error as a `JsValue::string(...)`.
#[wasm_bindgen]
pub fn run(js_string: JsValue) -> JsValue {
    let string = JsValue::as_string(&js_string).unwrap_throw();

    let compiled = lex(Source::source(&string))
        .and_then(parse)
        .and_then(desugar)
        .and_then(hoist)
        .and_then(|t| gen_with_ffi(t, js_ffi()));

    let closure = match compiled {
        Ok(lambda)  => Closure::wrap(lambda),
        Err(syntax) => return JsValue::from_str(&format!("{}", syntax)),
    };

    let mut vm = VM::init(closure);
    let result = vm.run();

    return match result {
        Ok(())     => JsValue::null(),
        Err(trace) => JsValue::from_str(&format!("{}", trace))
    }
}
