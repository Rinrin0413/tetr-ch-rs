//! Searched user model.

use crate::{
    client::Client,
    error::ResponseError,
    model::{
        cache::CacheData,
        user::{UserId, UserRecordsResponse, UserResponse},
    },
};
use serde::Deserialize;

/// The response for the searched user.
/// Describes the found userm or `None` if the user was not found.
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
    /// The found user.
    /// If the user was not found, this is `None`.
    pub data: Option<UserData>,
}

impl SearchedUserResponse {
    /// Gets the user's data.
    /// Returns `None` if the user was not found.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user(&self) -> Option<Result<UserResponse, ResponseError>> {
        if let Some(u) = &self.data {
            Some(Client::new().get_user(u.user.id.id()).await)
        } else {
            None
        }
    }

    /// Gets the user's records data.
    /// Returns `None` if the user was not found.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_records(&self) -> Option<Result<UserRecordsResponse, ResponseError>> {
        if let Some(u) = &self.data {
            Some(Client::new().get_user_records(u.user.id.id()).await)
        } else {
            None
        }
    }

    /// Returns the user's profile URL or `None` if the user was not found.
    pub fn profile_url(&self) -> Option<String> {
        self.data.as_ref().map(|u| u.profile_url())
    }

    /// Returns a UNIX timestamp when this resource was cached.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_at(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_until(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }
}

impl AsRef<SearchedUserResponse> for SearchedUserResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The found user.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserData {
    /// The user info. (TETRA.IO account)
    pub user: UserInfo,
}

impl UserData {
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
    pub async fn get_user(&self) -> Result<UserResponse, ResponseError> {
        Client::new().get_user(self.user.id.id()).await
    }

    /// Gets the user's records data.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_records(&self) -> Result<UserRecordsResponse, ResponseError> {
        Client::new().get_user_records(self.user.id.id()).await
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

/// The user info. (TETRA.IO account)
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserInfo {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    #[serde(rename = "username")]
    pub name: String,
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
    pub async fn get_user(&self) -> Result<UserResponse, ResponseError> {
        Client::new().get_user(self.id.id()).await
    }

    /// Gets the user's records data.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_records(&self) -> Result<UserRecordsResponse, ResponseError> {
        Client::new().get_user_records(self.id.id()).await
    }

    /// Returns the user's profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.name)
    }
}

impl AsRef<UserInfo> for UserInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}
