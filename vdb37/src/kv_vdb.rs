use std::collections::HashMap;
use std::fs;

use pyo3::prelude::*;

use crate::disk::{load, save};
use crate::vdb::VectorDatabase;

#[pyclass]
pub struct KvVectorDatabase {
    items: HashMap<String, VectorDatabase>,
}

#[pymethods]
impl KvVectorDatabase {
    #[new]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_db(&mut self, key: String, vdb: &VectorDatabase) {
        self.items.insert(key, vdb.clone());
    }

    pub fn get(&self, key: String) -> VectorDatabase {
        self.items.get(&key).expect("Key not found").to_owned()
    }

    pub fn save(&self, base_path: String) -> String {
        for (key, vdb) in &self.items {
            save(base_path.to_owned(), key.to_owned(), vdb.to_owned());
        }
        base_path
    }

    #[staticmethod]
    fn load(base_path: String) -> Self {
        let mut items: HashMap<String, VectorDatabase> = HashMap::new();

        for entry in fs::read_dir(base_path.to_owned()).expect("Could not read directory") {
            let entry = entry.unwrap().path().display().to_string();

            let vdb = load(base_path.to_owned(), entry.to_owned());
            items.insert(entry, vdb);
        }

        Self { items }
    }
}
