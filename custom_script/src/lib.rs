use crate::script::check_js_validity;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub mod script;

#[wasm_bindgen_test]
fn check_js() {
    let string = "function add(a, b) { return a + b; }";
    let string_2 = "function add(a, b) { return ";
    let result = check_js_validity(string);
    let result_2 = check_js_validity(string_2);
    assert_eq!(result, true);
    assert_eq!(result_2, false);
}
