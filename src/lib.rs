use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn log_string() {
    log("Testing console log from Rust!");
}

#[wasm_bindgen]
pub fn log_number() {
    log_u32(42);
}

#[wasm_bindgen]
pub fn log_two_strings() {
    log_many("-first-", "-second-");
}

#[wasm_bindgen]
pub fn log_with_macro() {
    console_log!("Hello {}!", "rust macros");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}
