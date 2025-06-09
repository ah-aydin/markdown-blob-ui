use gloo_net::http::Response;
use gloo_net::Error;
use serde::de::DeserializeOwned;

use crate::api_types::ApiError;

pub async fn parse_response<T>(response: Result<Response, Error>) -> Result<T, Option<ApiError>>
where
    T: DeserializeOwned,
{
    match response {
        Ok(res) => {
            if res.status() == 200 {
                let signup_response: T = deserialize_json(res).await;
                Ok(signup_response)
            } else if res.status() < 500 {
                let api_error: ApiError = deserialize_json(res).await;
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

async fn deserialize_json<T>(response: Response) -> T
where
    T: DeserializeOwned,
{
    response.json::<T>().await.unwrap()
}
