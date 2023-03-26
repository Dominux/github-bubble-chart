use serde::{Deserialize, Serialize};

use crate::common::types::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRepositorySizeModel {
    pub name: String,
    pub url: String,
    pub size: Number,
    pub language: Option<String>,
}
