use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/src/generated/index.js")]
extern "C" {
    fn isValidJavaScript(jsCode: &str) -> bool;
    fn addStringStream(streamString: &str, el: HtmlElement);
}

#[wasm_bindgen]
pub fn check_js_validity(js_code: &str) -> bool {
    isValidJavaScript(js_code)
}

#[wasm_bindgen]
pub fn add_string_stream(stream_string: &str, el: HtmlElement) {
    addStringStream(stream_string, el)
}
