use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleSearch {
    phrase: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoricalSearch {
    phrase: String,
    categories: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BooleanQuery {
    AND(String),
    OR(String),
    NOT(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BooleanSearch {
    queries: Vec<BooleanQuery>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SearchQuery {
    Simple(SimpleSearch),
    CategoricalSearch(CategoricalSearch),
    BooleanSearch(BooleanSearch),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MultiOptionSearch {
    queries: Vec<SearchQuery>,
}
