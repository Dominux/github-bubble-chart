use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRepositorySizeModel {
    name: String,
    url: String,
    size: u32,
    language: Option<String>,
}
