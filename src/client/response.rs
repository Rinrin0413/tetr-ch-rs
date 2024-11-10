use super::error::{ResponseError, Status};
use http::StatusCode;
use reqwest::{Error, Response};
use serde::Deserialize;

/// Receives a `Result<Response, Error>` and returns a `Result<T, ResponseError>`.
///
/// # Examples
///
/// ```ignore
/// let res = self.client.get(url).send().await;
/// response(res).await
/// ```
pub(super) async fn response<T>(response: Result<Response, Error>) -> Result<T, ResponseError>
where
    for<'de> T: Deserialize<'de>,
{
    match response {
        Ok(r) => {
            if !r.status().is_success() {
                match StatusCode::from_u16(r.status().as_u16()) {
                    Ok(c) => return Err(ResponseError::HttpErr(Status::Valid(c))),
                    Err(e) => return Err(ResponseError::HttpErr(Status::Invalid(e))),
                }
            }
            match r.json().await {
                Ok(m) => Ok(m),
                Err(e) => Err(ResponseError::DeserializeErr(e.to_string())),
            }
        }
        Err(e) => Err(ResponseError::RequestErr(e.to_string())),
    }
}
