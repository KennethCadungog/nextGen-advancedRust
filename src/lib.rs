use pyo3::prelude::*;

#[pyfunction]
fn sum_of_squares_python(numbers: Vec<i32>)-> i32 {
    numbers.iter().map(|&x| x * x).sum()
}

#[pyfunction]
fn rust_booster(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_of_squares_python, m)?)?;
    Ok(())
}