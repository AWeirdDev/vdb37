use std::fs::File;
use std::path::Path;

use pyo3::prelude::*;

use crate::vdb::VectorDatabase;
use crate::vector::Vector;

pub fn save(base_path: String, key: String, db: VectorDatabase) {
    let path = Path::new(&base_path).join(key);
    std::fs::create_dir_all(path.to_owned()).expect("Could not create directory");

    create_bin(
        format!("{}/vector.bin", path.to_str().unwrap()),
        db.kd_tree.root.unwrap().vector,
    );
}

#[pyfunction]
pub fn create_bin(filename: String, vector: Vector) {
    let mut file = File::create(filename).expect("Could not create file");
    bincode::serialize_into(&mut file, &vector.coordinates).expect("Could not serialize");
}

pub fn load(base_path: String, key: String) -> VectorDatabase {
    let path = Path::new(&base_path).join(key);

    let bin = load_bin(format!("{}/vector.bin", path.to_str().unwrap()));
    let mut db = VectorDatabase::new();

    db.kd_tree.insert(bin);
    db
}

#[pyfunction]
pub fn load_bin(filename: String) -> Vector {
    let mut file = File::open(filename).expect("Could not open file");
    let v: Vec<f64> = bincode::deserialize_from(&mut file).expect("Could not deserialize");

    Vector::new(v)
}
