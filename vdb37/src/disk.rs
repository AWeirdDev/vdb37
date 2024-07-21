use std::fs::{self, File};
use std::path::Path;

use pyo3::prelude::*;

use crate::vdb::VectorDatabase;
use crate::vector::Vector;

pub fn save(base_path: String, db: VectorDatabase) {
    let path = Path::new(&base_path);
    std::fs::create_dir_all(path.to_owned()).expect("Could not create directory");

    for (i, vec) in db.vectors.iter().enumerate() {
        create_bin(
            format!("{}/vector_{}.bin", path.to_str().unwrap(), i),
            vec.clone(),
        );
    }
}

#[pyfunction]
pub fn create_bin(filename: String, vector: Vector) {
    let mut file = File::create(filename).expect("Could not create file");
    bincode::serialize_into(&mut file, &(&vector.coordinates, &vector.meta))
        .expect("Could not serialize");
}

pub fn load(base_path: String) -> VectorDatabase {
    let path = Path::new(&base_path);
    let mut db = VectorDatabase::new();

    for entry in fs::read_dir(path).expect("Could not read directory") {
        let entry = entry.expect("Could not read entry while reading directory");
        db.insert(load_bin(entry.path().to_str().unwrap().to_string()));
    }

    db
}

#[pyfunction]
pub fn load_bin(filename: String) -> Vector {
    let mut file = File::open(filename).expect("Could not open file");
    let v: (Vec<f64>, String) =
        bincode::deserialize_from(&mut file).expect("Could not deserialize");

    Vector::new(v.0, Some(v.1))
}
