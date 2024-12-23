//! Models for responses.

use super::cache::CacheData;
use crate::model::prelude::*;
use std::fmt;

/// A struct for responses.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Response<T: Clone + fmt::Debug + AsRef<T>> {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<T>,
}

impl<T: Clone + fmt::Debug + AsRef<T>> AsRef<Response<T>> for Response<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An error response.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ErrorResponse {
    /// The error message.
    ///
    /// e.g. "No such user! | Either you mistyped something, or the account no longer exists."
    pub msg: Option<String>,
    pub key: Option<String>,
    pub context: Option<String>,
}

impl AsRef<ErrorResponse> for ErrorResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}
