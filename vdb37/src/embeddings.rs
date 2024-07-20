use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use pyo3::prelude::*;

#[pyclass]
pub struct Embedding {
    model: String,
}

#[pymethods]
impl Embedding {
    #[new]
    fn new(model: String) -> Self {
        Self { model }
    }

    fn embed(&self, documents: Vec<String>) -> Vec<Vec<f64>> {
        let model = TextEmbedding::try_new(InitOptions {
            model_name: match self.model.as_str() {
                "all-MiniLM-L6-v2" => EmbeddingModel::AllMiniLML6V2,
                "all-MiniLM-L12-v2" => EmbeddingModel::AllMiniLML12V2,
                _ => panic!("Unknown model: {}", self.model),
            },
            show_download_progress: true,
            ..Default::default()
        })
        .expect("Failed to load embedding model");

        let embeddings = model
            .embed(documents, None)
            .expect("Failed to embed documents");

        embeddings // convert to f64
            .into_iter()
            .map(|v| v.into_iter().map(|f| f as f64).collect())
            .collect()
    }
}
