use pyo3::prelude::*;

mod vdb;
mod vector;
mod kd_tree;

/// A Python module implemented in Rust.
#[pymodule]
fn vdb37(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<vector::Vector>()?;
    m.add_class::<vdb::VectorDatabase>()?;
    Ok(())
}
