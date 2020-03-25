#[cfg(feature = "javascript_dev")]
mod utils;
#[cfg(feature = "javascript_dev")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python")]
use pyo3::{
    prelude::*,
    wrap_pyfunction,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "python")]
#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// This module is a python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
fn sglib(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;

    Ok(())
}

#[cfg(feature = "javascript_dev")]
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
#[cfg(feature = "javascript_dev")]
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sglibjs!");
}
