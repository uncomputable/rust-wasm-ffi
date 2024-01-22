use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
struct Complex {
    real: i32,
    im: i32
}

extern "C" {
    fn add(a: *const Complex, b: *const Complex, result: *const Complex) -> bool;
    fn sub(a: *const Complex, b: *const Complex, result: *const Complex) -> bool;
    fn mul(a: *const Complex, b: *const Complex, result: *const Complex) -> bool;
    fn div(a: *const Complex, b: *const Complex, result: *const Complex) -> bool;
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.im)
    }
}

#[wasm_bindgen]
impl Complex {
    #[wasm_bindgen(constructor)]
    pub fn new(real: i32, im: i32) -> Self {
        Complex {
            real,
            im
        }
    }

    pub fn add(&self, other: &Complex) -> Option<Complex> {
        let result = Complex::default();
        let ok = unsafe {
            add(self, other, &result)
        };
        ok.then_some(result)
    }

    pub fn sub(&self, other: &Complex) -> Option<Complex> {
        let result = Complex::default();
        let ok = unsafe {
            sub(self, other, &result)
        };
        ok.then_some(result)
    }

    pub fn mul(&self, other: &Complex) -> Option<Complex> {
        let result = Complex::default();
        let ok = unsafe {
            mul(self, other, &result)
        };
        ok.then_some(result)
    }

    pub fn div(&self, other: &Complex) -> Option<Complex> {
        let result = Complex::default();
        let ok = unsafe {
            div(self, other, &result)
        };
        ok.then_some(result)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn test_complex_add() {
        let a = Complex::new(1, 2);
        let b = Complex::new(3, 4);
        assert_eq!(Some(Complex::new(4, 6)), a.add(&b))
    }
}

