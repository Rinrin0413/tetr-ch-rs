//! The Records Leaderboard models.

use crate::model::{cache::CacheData, summary::record::Record};
use serde::Deserialize;

/// The response for the Records Leaderboard data.
///
/// A list of Records fulfilling the search criteria.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RecordsLeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
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

/// The Records Leaderboard data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RecordsLeaderboard {
    /// The matched records.
    pub entries: Vec<Record>,
}
