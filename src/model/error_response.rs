//! A model for the error response.

use serde::Deserialize;

/// An error response.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ErrorResponse {
    /// The error message.
    ///
    /// e.g. "No such user! | Either you mistyped something, or the account no longer exists."
    pub msg: Option<String>,
}

impl AsRef<ErrorResponse> for ErrorResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}
