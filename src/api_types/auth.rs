use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SignupResponse {
    pub id: u64,
    pub email: String,
    pub created_at: u64,
    pub updated_at: u64,
}
