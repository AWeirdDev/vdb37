use pyo3::prelude::*;
use std::fmt;

#[pyclass]
#[derive(Clone, Default)]
pub struct Vector {
    pub coordinates: Vec<f64>,
}

#[pymethods]
impl Vector {
    #[new]
    pub fn new(coordinates: Vec<f64>) -> Self {
        Self { coordinates }
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
        format!("<Vector coordinates={:?}>", self.coordinates)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Vector ln={:?}>", self.coordinates.len())
    }
}
