use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddResult {
    pub uid: u64,
    pub status_code: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorAddResult {
    pub msg: Option<String>,
    pub status_code: u16,
}