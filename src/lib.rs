use ndarray::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn axpy(a: f64, x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> ArrayD<f64> {
    a * &x + &y
}

#[pyfunction]
fn axpy_py(py: Python, a: f64, x: &PyArrayDyn<f64>, y: &PyArrayDyn<f64>) -> Py<PyArrayDyn<f64>> {
    // axpy のラッパー
    unsafe {
        let x = x.as_array();
        let y = y.as_array();
        axpy(a, x, y).into_pyarray(py).to_owned()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin_handson(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(axpy_py, m)?)?;
    Ok(())
}
