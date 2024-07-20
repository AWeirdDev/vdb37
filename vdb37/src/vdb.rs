use pyo3::prelude::*;

use crate::kd_tree::KDTree;
use crate::vector::Vector;

#[pyclass]
pub struct VectorDatabase {
    kd_tree: KDTree,
}

#[pymethods]
impl VectorDatabase {
    #[new]
    fn new() -> Self {
        Self {
            kd_tree: KDTree::new(),
        }
    }

    fn insert(&mut self, vector: Vector) {
        self.kd_tree.insert(vector);
    }

    fn search(&self, query: Vector, k: usize) -> Vec<Vector> {
        self.kd_tree.nearest(&query, k)
    }

    fn __repr__(&self) -> &str {
        "VectorDatabase()"
    }
}
