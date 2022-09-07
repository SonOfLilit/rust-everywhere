use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn reverse(s: &str) -> PyResult<String> {
    Ok(rust_everywhere::reverse(s))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_everywhere_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reverse, m)?)?;
    Ok(())
}
