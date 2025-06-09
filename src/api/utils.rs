use gloo_net::http::Response;
use serde::de::DeserializeOwned;

pub async fn parse_response<T>(response: Response) -> T
where
    T: DeserializeOwned,
{
    response.json::<T>().await.unwrap()
}
