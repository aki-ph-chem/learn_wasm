mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(your_name: &str) {
    let fmt = format!("Hello, {}!", your_name);
    alert(&fmt);
}
