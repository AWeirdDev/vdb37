use pyo3::prelude::*;

use crate::disk;
use crate::kd_tree::KDTree;
use crate::vector::Vector;

#[pyclass]
#[derive(Clone)]
pub struct VectorDatabase {
    pub kd_tree: KDTree,
    pub vectors: Vec<Vector>,
}

#[pymethods]
impl VectorDatabase {
    #[new]
    pub fn new() -> Self {
        Self {
            kd_tree: KDTree::new(),
            vectors: vec![],
        }
    }

    pub fn insert(&mut self, vector: Vector) {
        self.kd_tree.insert(vector.to_owned());
        self.vectors.push(vector);
    }

    pub fn search(&self, query: Vector, k: usize) -> Vec<Vector> {
        self.kd_tree.nearest(&query, k)
    }

    #[getter]
    pub fn vectors(&self) -> Vec<Vector> {
        self.vectors.to_owned()
    }

    pub fn save(&self, base_path: String) {
        if self.vectors.len() == 0 {
            Python::with_gil(|py| {
                Python::run_bound(
                    py,
                    "import warnings; warnings.warn('vdb37: vector database is empty')",
                    None,
                    None,
                )
                .unwrap();
            })
        } else {
            disk::save(base_path, self.to_owned());
        }
    }

    #[staticmethod]
    pub fn load(base_path: String) -> Self {
        disk::load(base_path)
    }

    fn __repr__(&self) -> String {
        if let Some(root) = &self.kd_tree.root {
            return format!(
                "VectorDatabase({:?}, {:?}, {:?})",
                root.vector, root.left, root.right
            );
        }
        format!("VectorDatabase(None)")
    }
}
