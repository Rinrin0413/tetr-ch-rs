//! The Searched User model.

use crate::{
    client::{error::RspErr, Client},
    model::{
        cache::CacheData,
        user::{UserId, UserResponse},
    },
};
use serde::Deserialize;

/// The response for the Searched User data.
/// An object describing the user found.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SearchedUserResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<UserData>,
}

impl AsRef<SearchedUserResponse> for SearchedUserResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The Searched User data.
///
/// Only one user is contained.
/// Generally, you won't see two users with the same social linked, though,
/// as it would be against TETR.IO multiaccounting policies.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserData {
    /// The user information (TETRA.IO user account).
    pub user: UserInfo,
}

impl UserData {
    /// Gets the User Info data.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        self.user.get_user().await
    }

    /// Returns the user's profile URL.
    pub fn profile_url(&self) -> String {
        self.user.profile_url()
    }
}

impl AsRef<UserData> for UserData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user information (TETRA.IO user account).
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserInfo {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    pub username: String,
}

impl UserInfo {
    /// Gets the user's data.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        Client::new().get_user(&self.id.to_string()).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }
}

impl AsRef<UserInfo> for UserInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}
