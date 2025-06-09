pub mod auth;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error_type: String,
    pub message: String,
}
