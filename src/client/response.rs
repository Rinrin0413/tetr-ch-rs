use super::error::{ResponseError, RspErr};
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
pub(super) async fn process_response<T>(response: Result<Response, Error>) -> RspErr<T>
where
    for<'de> T: Deserialize<'de>,
{
    // Whether the request succeeded or not.
    match response {
        Ok(r) => {
            let status = r.status();
            let is_success = status.is_success();
            // Whether the response is an expected structure or not.
            match r.json().await {
                Ok(m) => Ok(m),
                Err(e) => {
                    // Whether the status code is within 200-299 or not.
                    if is_success {
                        Err(ResponseError::DeserializeErr(e))
                    } else {
                        Err(ResponseError::HttpErr(status))
                    }
                }
            }
        }
        Err(e) => Err(ResponseError::RequestErr(e)),
    }
}
