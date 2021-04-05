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
    pub fn alert(s: &str);
}

pub fn pn_alert(data: Data) -> Result<Data, String> {
    let string = format!("{}", data);
    alert(&string);
    return Ok(data);
}

pub fn js_ffi() -> FFI {
    let mut ffi = ffi_core();
    // replace println!(...) with alert for now...
    ffi.add("print",   FFIFunction::new(Box::new(pn_alert))).unwrap_err();
    ffi.add("println", FFIFunction::new(Box::new(pn_alert))).unwrap_err();
    alert(&format!("{:?}", ffi));
    return ffi;
}

/// Compiles and runs some Passerine, represented as a string.
/// Returns `JsValue::null()` if the program ran with no errors,
/// Otherwise the error as a `JsValue::string(...)`.
#[wasm_bindgen]
pub fn run(js_string: JsValue) -> JsValue {
    alert("converting to string");
    let string = JsValue::as_string(&js_string).unwrap_throw();

    alert("compiling");
    let compiled = lex(Source::source(&string))
        .and_then(parse)
        .and_then(desugar)
        .and_then(hoist)
        .and_then(|t| gen_with_ffi(t, js_ffi()));

    alert("wrapping with closure");
    let closure = match compiled {
        Ok(lambda)  => {
            alert("it's ok!");
            Closure::wrap(lambda)
        },
        Err(syntax) => {
            alert("returning error");
            return JsValue::from_str(&format!("{}", syntax))
        },
    };

    alert("Initializing VM");
    let mut vm = VM::init(closure);
    alert("Running VM");
    let result = vm.run();

    alert("Returning Result");
    return match result {
        Ok(())     => JsValue::null(),
        Err(trace) => JsValue::from_str(&format!("{}", trace))
    }
}
