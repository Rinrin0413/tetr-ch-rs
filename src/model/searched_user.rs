//! Models for the endpoint "User Search".
//!
//! About the endpoint "User Search",
//! see the [API document](https://tetr.io/about/api/#userssearchquery).

use crate::model::{cache::CacheData, error_response::ErrorResponse, util::UserId};
use serde::Deserialize;

/// A struct for the response for the endpoint "User Search".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SearchedUserResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
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

/// A searched user.
///
/// Only one user is contained.
/// Generally, you won't see two users with the same social linked, though,
/// as it would be against TETR.IO multiaccounting policies.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserData {
    /// The user information (TETRA.IO user account).
    pub user: Option<UserInfo>,
}

impl AsRef<UserData> for UserData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user information (TETRA.IO user account).
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
    impl_get_user!(id);
    impl_for_username!();
}

impl AsRef<UserInfo> for UserInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}
