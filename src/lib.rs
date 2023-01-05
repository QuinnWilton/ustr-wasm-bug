use ustr::Ustr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn reproduce() {
    let s = Ustr::from("test");

    log(s.as_str());
}
