use wasm_bindgen::prelude::*;

extern "C" {
    pub fn f(x: i32) -> i32;
}

#[wasm_bindgen]
pub fn f_safe(x: i32) -> i32 {
    unsafe { f(x) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_f_safe() {
        for x in 0..10 {
            assert_eq!(f_safe(x), x + 1);
        }
    }
}
