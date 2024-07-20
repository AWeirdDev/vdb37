use pyo3::prelude::*;

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

    pub fn __repr__(&self) -> String {
        format!("<Vector coordinates={:?}>", self.coordinates)
    }
}