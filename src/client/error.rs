//! A module for the error related types for the [`client`](crate::client) module.

use http::status::StatusCode;
use std::fmt;

/// An enum for the response handling errors.
#[derive(Debug)]
pub enum ResponseError {
    /// The request failed.
    RequestErr(reqwest::Error),
    /// The response did not match the expected format but the HTTP request succeeded.
    ///
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    DeserializeErr(reqwest::Error),
    /// The HTTP request failed and the response did not match the expected format.
    ///
    /// Even if the HTTP status code is not within 200-299.
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    HttpErr(StatusCode),
}

impl std::error::Error for ResponseError {}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::DeserializeErr(msg) => write!(f, "{}", msg),
            ResponseError::RequestErr(err) => write!(f, "{}", err),
            ResponseError::HttpErr(status) => write!(f, "{}", status),
        }
    }
}

impl From<ResponseError> for std::io::Error {
    fn from(err: ResponseError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

pub(crate) type RspErr<T> = Result<T, ResponseError>;

/// An enum for the client creation errors.
#[derive(Debug)]
pub enum ClientCreationError {
    /// A TLS backend cannot be initialized, or the resolver cannot load the system configuration.
    BuildErr(reqwest::Error),
    /// The client contains invalid header value characters.
    /// Only visible ASCII characters (32-127) are permitted.
    InvalidHeaderValue(String),
}

impl std::error::Error for ClientCreationError {}

impl fmt::Display for ClientCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientCreationError::BuildErr(err) => write!(f, "{}", err),
            ClientCreationError::InvalidHeaderValue(v) => {
                write!(f, "failed to parse header value `{}`", v)
            }
        }
    }
}

#[cfg(test)]
mod tests {}
