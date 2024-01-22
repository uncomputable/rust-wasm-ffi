extern crate libc;

use wasm_bindgen::prelude::*;

extern "C" {
    fn f(x: i32) -> i32;
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn f_safe(x: i32) -> i32 {
    unsafe { f(x) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[wasm_bindgen_test]
    fn test_f_safe() {
        for x in 0..10 {
            assert_eq!(f_safe(x), x + 1);
        }
    }
}
