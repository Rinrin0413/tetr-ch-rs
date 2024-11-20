//! A model for the endpoint "User Summary: Achievements".
//!
//! About the endpoint "User Summary: Achievements",
//! see the [API document](https://tetr.io/about/api/#usersusersummariesachievements).

use crate::model::{achievement::Achievement, cache::CacheData, error_response::ErrorResponse};
use serde::Deserialize;

/// A struct for the response for the endpoint "User Summary: Achievements".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementsResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<Vec<Achievement>>,
}

impl AsRef<AchievementsResponse> for AchievementsResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}
