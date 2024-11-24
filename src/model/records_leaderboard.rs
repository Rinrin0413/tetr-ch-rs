//! Models for the endpoint "Records Leaderboard".
//!
//! About the endpoint "Records Leaderboard",
//! see the [API document](https://tetr.io/about/api/#recordsleaderboard).

use crate::model::{cache::CacheData, error_response::ErrorResponse, summary::record::Record};
use serde::Deserialize;

/// A struct for the response for the endpoint "Records Leaderboard".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RecordsLeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<RecordsLeaderboard>,
}

impl AsRef<RecordsLeaderboardResponse> for RecordsLeaderboardResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An array of records.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RecordsLeaderboard {
    /// The matched records.
    pub entries: Vec<Record>,
}
