use gloo_net::http::Request;

use crate::api_types::auth::LoginRequest;
use crate::api_types::auth::LoginResponse;
use crate::api_types::auth::SignupRequest;
use crate::api_types::auth::SignupResponse;
use crate::api_types::ApiError;

use super::utils::parse_response;

pub async fn login(email: String, password: String) -> Result<LoginResponse, Option<ApiError>> {
    let req_body = LoginRequest { email, password };
    let req_body_json =
        serde_json::to_string(&req_body).expect("Failed to serialize login request body to JSON");

    let res = Request::post("http://localhost:8080/api/v1/auth/login")
        .header("Content-Type", "application/json")
        .body(req_body_json)
        .expect("Failed to construct request")
        .send()
        .await;

    parse_response(res).await
}

pub async fn signup(email: String, password: String) -> Result<SignupResponse, Option<ApiError>> {
    let req_body = SignupRequest { email, password };
    let req_body_json =
        serde_json::to_string(&req_body).expect("Failed to serialize signup request body to JSON");

    let res = Request::post("http://localhost:8080/api/v1/auth")
        .header("Content-Type", "application/json")
        .body(req_body_json)
        .expect("Failed to construct request")
        .send()
        .await;

    parse_response(res).await
}
