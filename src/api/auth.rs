use gloo_net::http::Request;

use crate::api_types::auth::LoginRequest;
use crate::api_types::auth::LoginResponse;
use crate::api_types::ApiError;

use super::utils::parse_response;

pub async fn login(email: String, password: String) -> Result<LoginResponse, Option<ApiError>> {
    let req_body = LoginRequest { email, password };
    let req_body_json =
        serde_json::to_string(&req_body).expect("Failed to serialize login requet body to JSON");

    let res = Request::post("http://localhost:8080/api/v1/auth/login")
        .header("Content-Type", "application/json")
        .body(req_body_json)
        .expect("Failed to construct request")
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status() == 200 {
                let login_response: LoginResponse = parse_response(res).await;
                Ok(login_response)
            } else if res.status() < 500 {
                let api_error: ApiError = parse_response(res).await;
                Err(Some(api_error))
            } else {
                Err(None)
            }
        }
        Err(e) => {
            println!("{:?}", e);
            Err(None)
        }
    }
}
