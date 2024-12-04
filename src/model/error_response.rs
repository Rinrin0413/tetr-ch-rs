//! A model for the error response.

use crate::model::prelude::*;

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
