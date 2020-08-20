use wasm_bindgen::prelude::*;

extern crate web_sys;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() {
    log("[WASM] internal logging...");
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hey, CUBETIQ Solution!");
}

#[wasm_bindgen]
pub fn log(s: &str) {
    web_sys::console::log_1(&s.into());
}
