//! Models for the endpoint "User Summary: ZEN".
//!
//! About the endpoint "User Summary: ZEN",
//! see the [API document](https://tetr.io/about/api/#usersusersummarieszen).

use crate::model::prelude::*;

/// A struct for the response for the endpoint "User Summary: ZEN".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ZenResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
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

/// A struct that describes a summary of a user's ZEN progress.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Zen {
    /// The user's ZEN level.
    pub level: u32,
    /// The user's ZEN score.
    pub score: f64,
}

impl AsRef<Zen> for Zen {
    fn as_ref(&self) -> &Self {
        self
    }
}
