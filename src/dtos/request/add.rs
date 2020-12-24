use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddDocument {
    pub document: String,
    pub locale: Option<String>,
    pub categories: Option<Vec<String>>,
}