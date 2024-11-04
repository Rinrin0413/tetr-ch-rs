//! The User Summary 40 LINES models.

use crate::model::{
    cache::CacheData,
    summary::record::Record,
};
use serde::Deserialize;

/// The response for the User Summary 40 LINES data.
/// An object describing a summary of the user's 40 LINES games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct FortyLinesResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<FortyLines>,
}

impl AsRef<FortyLinesResponse> for FortyLinesResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The User Summary 40 LINES data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct FortyLines {
    /// The user's 40 LINES record, or `None` if never played.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or -1 if not in global leaderboards.
    pub rank: i32,
    /// The user's rank in their country's leaderboards, or -1 if not in any.
    pub rank_local: i32,
}

impl AsRef<FortyLines> for FortyLines {
    fn as_ref(&self) -> &Self {
        self
    }
}
