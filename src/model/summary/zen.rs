//! The User Summary ZEN models.

use crate::model::cache::CacheData;
use serde::Deserialize;

/// The response for the User Summary ZEN data.
/// An object describing a summary of the user's ZEN progress.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ZenResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<Zen>,
}

impl AsRef<ZenResponse> for ZenResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The User Summary ZEN data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Zen {
    /// The user's level.
    pub level: u32,
    /// The user's score.
    pub score: f64,
}

impl AsRef<Zen> for Zen {
    fn as_ref(&self) -> &Self {
        self
    }
}
