//! The User Summary Achievements models.

use crate::model::{achievement::Achievement, cache::CacheData};
use serde::Deserialize;

/// The response for the User Summary Achievements data.
/// An object containing all the user's achievements.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementsResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
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
