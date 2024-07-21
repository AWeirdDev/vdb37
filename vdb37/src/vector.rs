use pyo3::prelude::*;
use std::fmt;

#[derive(Clone, Default)]
#[pyclass]
pub struct Vector {
    pub coordinates: Vec<f64>,
    pub meta: String,
}

#[pymethods]
impl Vector {
    #[new]
    #[pyo3(signature = (coordinates, *, meta=None))]
    pub fn new(coordinates: Vec<f64>, meta: Option<String>) -> Self {
        Self {
            coordinates,
            meta: meta.unwrap_or(String::from("")),
        }
    }

    pub fn distance(&self, other: &Vector) -> f64 {
        self.coordinates
            .iter()
            .zip(&other.coordinates)
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn within(&self, lower: &Vector, upper: &Vector) -> bool {
        self.coordinates
            .iter()
            .zip(&lower.coordinates)
            .zip(&upper.coordinates)
            .all(|((&c, &low), &up)| c >= low && c <= up)
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<Vector coordinates_ln={:?} meta={:?}>",
            self.coordinates.len(),
            self.meta
        )
    }

    #[getter]
    pub fn coordinates(&self) -> Vec<f64> {
        self.coordinates.to_owned()
    }

    #[getter]
    pub fn meta(&self) -> String {
        self.meta.to_owned()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Vector coordinates_ln={:?} meta={:?}>",
            self.coordinates.len(),
            self.meta
        )
    }
}
