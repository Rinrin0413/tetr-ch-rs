//! Models for the endpoint "User Personal Records".
//!
//! About the endpoint "User Personal Records",
//! see the [API document](https://tetr.io/about/api/#usersuserrecordsgamemodeleaderboard).

use crate::model::prelude::*;

/// A struct for the response for the endpoint "User Personal Records".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserRecordsResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<UserRecords>,
}

impl AsRef<UserRecordsResponse> for UserRecordsResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An array of user personal records.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserRecords {
    /// The matched records.
    pub entries: Vec<Record>,
}
