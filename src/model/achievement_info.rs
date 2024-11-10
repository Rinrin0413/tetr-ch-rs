//! The Achievement Info models.

use crate::model::{cache::CacheData, summary::achievements::Achievement, user::UserId};
use serde::Deserialize;

use super::user::Role;

/// The response for the Achievement Info data.
///
/// Data about the achievement itself, its cutoffs, and its leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementInfoResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<AchievementInfo>,
}

impl AsRef<AchievementInfoResponse> for AchievementInfoResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The Achievement Info data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementInfo {
    /// The achievement info.
    pub achievement: Achievement,
    /// The entries in the achievement's leaderboard.
    pub leaderboard: Vec<AchievementLeaderboardEntry>,
    /// Scores required to obtain the achievement:
    pub cutoffs: Cutoffs,
}

impl AsRef<AchievementInfo> for AchievementInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An entry in an achievement's leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementLeaderboardEntry {
    /// The user owning the achievement.
    #[serde(rename = "u")]
    pub user: User,
    /// The achieved score in the achievement.
    #[serde(rename = "v")]
    pub value: f64,
    /// Additional score for the achievement.
    #[serde(rename = "a")]
    pub additional_value: Option<f64>,
    /// The time the achievement was last updated.
    #[serde(rename = "t")]
    pub last_updated_at: String,
}

impl AsRef<AchievementLeaderboardEntry> for AchievementLeaderboardEntry {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user owning an achievement.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's role.
    pub role: Role,
    /// Whether the user is supporting TETR.IO.
    #[serde(rename = "supporter")]
    pub is_supporter: bool,
    /// The user's country, if public.
    pub country: Option<String>,
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Scores required to obtain the achievement.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Cutoffs {
    /// The total amount of users with this achievement.
    pub total: u32,
    /// If applicable, the score required to obtain a Diamond rank.
    pub diamond: Option<f64>,
    /// If applicable, the score required to obtain a Platinum rank.
    pub platinum: Option<f64>,
    /// If applicable, the score required to obtain a Gold rank.
    pub gold: Option<f64>,
    /// If applicable, the score required to obtain a Silver rank.
    pub silver: Option<f64>,
    /// If applicable, the score required to obtain a Bronze rank.
    pub bronze: Option<f64>,
}

impl AsRef<Cutoffs> for Cutoffs {
    fn as_ref(&self) -> &Self {
        self
    }
}
