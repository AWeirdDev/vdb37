use pyo3::prelude::*;

mod disk;
mod kd_tree;
mod vdb;
mod vector;

/// A Python module implemented in Rust.
#[pymodule]
fn vdb37(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<vector::Vector>()?;
    m.add_class::<vdb::VectorDatabase>()?;
    m.add_function(wrap_pyfunction!(disk::create_bin, m)?)?;
    m.add_function(wrap_pyfunction!(disk::load_bin, m)?)?;
    Ok(())
}
