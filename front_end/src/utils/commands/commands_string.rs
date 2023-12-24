use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/generated/index.js")]
extern "C" {
    fn isValidJavaScript(jsCode: &str) -> bool;
    fn addStringStream(streamString: &str);
}

#[wasm_bindgen]
pub fn check_js_validity(js_code: &str) -> bool {
    isValidJavaScript(js_code)
}

#[wasm_bindgen]
pub fn add_string_stream(stream_string: &str) {
    addStringStream(stream_string)
}
