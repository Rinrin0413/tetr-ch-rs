//! Models for the endpoint "User Summary: BLITZ".
//!
//! About the endpoint "User Summary: BLITZ",
//! see the [API document](https://tetr.io/about/api/#usersusersummariesblitz).

use crate::model::{cache::CacheData, error_response::ErrorResponse, summary::record::Record};
use serde::Deserialize;

/// A struct for the response for the endpoint "User Summary: BLITZ".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct BlitzResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<Blitz>,
}

impl AsRef<BlitzResponse> for BlitzResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A struct that describes a summary of a user's BLITZ games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Blitz {
    /// The user's BLITZ record, or `None` if never played.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or -1 if not in global leaderboards.
    pub rank: i32,
    /// The user's rank in their country's leaderboards, or -1 if not in any.
    pub rank_local: i32,
}

impl AsRef<Blitz> for Blitz {
    fn as_ref(&self) -> &Self {
        self
    }
}
