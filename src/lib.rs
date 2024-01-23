pub mod bar;
pub mod foo;

extern crate libc;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
