//! A module for the error related types for the [`client`](crate::client) module.

use http::status::{InvalidStatusCode, StatusCode};
use std::error::Error;
use std::fmt;

/// A enum for the response handling errors.
#[derive(Debug)]
pub enum ResponseError {
    /// The request failed.
    RequestErr(String),
    /// The response did not match the expected format but the HTTP request succeeded.
    ///
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    DeserializeErr(String),
    /// The HTTP request failed and the response did not match the expected format.
    ///
    /// Even if the HTTP status code is not within 200-299.
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    HttpErr(Status),
}

impl Error for ResponseError {}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::DeserializeErr(msg) => write!(f, "{}", msg),
            ResponseError::RequestErr(msg) => write!(f, "{}", msg),
            ResponseError::HttpErr(status) => {
                if let Status::Valid(sc) = status {
                    write!(f, "HTTP error {}", sc)
                } else {
                    write!(f, "HTTP error (Invalid HTTP status code)")
                }
            }
        }
    }
}

impl From<ResponseError> for std::io::Error {
    fn from(err: ResponseError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

/// A HTTP status code.
#[derive(Debug)]
pub enum Status {
    /// A valid HTTP status code.
    /// If the status code greater or equal to 100 but less than 600.
    Valid(StatusCode),
    /// An invalid HTTP status code.
    /// If the status code less than 100 or greater than 599.
    Invalid(InvalidStatusCode),
}

pub(crate) type RspErr<T> = Result<T, ResponseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_errors_to_string() {
        let de_err = ResponseError::DeserializeErr("Deserialize error".to_string());
        let req_err = ResponseError::RequestErr("Request error".to_string());
        let http_err = ResponseError::HttpErr(Status::Valid(http::StatusCode::SERVICE_UNAVAILABLE));
        let invalid_http_err =
            ResponseError::HttpErr(Status::Invalid(if let Err(isc) = StatusCode::from_u16(0) {
                isc
            } else {
                unreachable!()
            }));
        assert_eq!(de_err.to_string(), "Deserialize error");
        assert_eq!(req_err.to_string(), "Request error");
        assert_eq!(http_err.to_string(), "HTTP error 503 Service Unavailable");
        assert_eq!(
            invalid_http_err.to_string(),
            "HTTP error (Invalid HTTP status code)"
        );
    }
}
