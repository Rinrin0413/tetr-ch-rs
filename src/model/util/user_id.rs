//! A model for user IDs,

use crate::{
    client::{error::RspErr, Client},
    model::user::UserResponse,
};
use serde::Deserialize;
use std::fmt;

/// A user's internal ID.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct UserId(String);

impl UserId {
    /// Gets the detailed information about the user.
    ///
    /// # Errors
    ///
    /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    /// if the request failed.
    /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    /// if the response did not match the expected format but the HTTP request succeeded.
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    /// if the HTTP request failed and the response did not match the expected format.
    /// Even if the HTTP request failed,
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        Client::new().get_user(&self.to_string()).await
    }

    /// Returns the user's internal ID.
    #[deprecated(since = "0.6.0", note = "please use the `.to_string()` method instead")]
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl AsRef<UserId> for UserId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
