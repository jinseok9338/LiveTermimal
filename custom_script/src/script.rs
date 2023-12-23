use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/__generated__/index.js")]
extern "C" {
    fn isValidJavaScript(jsCode: &str) -> bool;
}

#[wasm_bindgen]
pub fn check_js_validity(js_code: &str) -> bool {
    isValidJavaScript(js_code)
}
