use pyo3::prelude::*;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[pyfunction]
fn hello_world() {
  println!("hello_pyo3!!");
}

/// A Python module implemented in Rust.
#[allow(non_snake_case)]
#[pymodule]
fn corepyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(hello_world, m)?)?;
  m.add_function(wrap_pyfunction!(add, m)?)?;
  Ok(())
}
