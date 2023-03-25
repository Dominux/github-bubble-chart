use serde::{Deserialize, Serialize};

use crate::common::types::Int;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRepositorySizeModel {
    pub name: String,
    pub url: String,
    pub size: Int,
    pub language: Option<String>,
}
