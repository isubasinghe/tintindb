use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddSimpleDocument {
    pub document: String,
    pub locale: Option<String>,
    pub guess_locale: Option<bool>,
    pub categories: Option<Vec<String>>,
    pub metadata: Option<Metadata>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TextField {
    Boosted(String, f64),
    Regular(String)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddCustomDocument {
    pub document: Vec<TextField>,
    pub locale: Option<String>,
    pub guess_locale: Option<bool>,
    pub categories: Option<Vec<String>>,
    pub metadata: Option<Metadata>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AddDocument {
    AddSimpleDocument(AddSimpleDocument),
    AddCustomDocument(AddCustomDocument)
}