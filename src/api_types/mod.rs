pub mod auth;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub error_type: String,
    pub message: String,
}
