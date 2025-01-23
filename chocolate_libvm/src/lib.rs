use wasm_bindgen::prelude::*;

pub mod instruction;
pub mod program;
pub mod virt;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
