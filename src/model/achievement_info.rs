//! Models for the endpoint "Achievement Info".
//!
//! About the endpoint "Achievement Info",
//! see the [API document](https://tetr.io/about/api/#achievementsk).

use crate::{
    client::error::RspErr,
    model::{
        cache::CacheData,
        error_response::ErrorResponse,
        user::UserResponse,
        util::{achievement::Achievement, role::Role, user_id::UserId},
    },
};
use serde::Deserialize;

/// A struct for the response for the endpoint "Achievement Info".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementInfoResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
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

/// Data about an achievement itself, its cutoffs, and its leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementInfo {
    /// The achievement info.
    pub achievement: Achievement,
    /// The entries in the achievement's leaderboard.
    pub leaderboard: Vec<AchievementLeaderboardUser>,
    /// Scores required to obtain the achievement:
    pub cutoffs: Cutoffs,
}

impl AsRef<AchievementInfo> for AchievementInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// User's achievement data in an achievement's leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementLeaderboardUser {
    /// The user owning the achievement.
    #[serde(rename = "u")]
    pub user: PartialUser,
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

impl AsRef<AchievementLeaderboardUser> for AchievementLeaderboardUser {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Partial information about a user.
/// This is used in the [`AchievementLeaderboardUser`] struct.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PartialUser {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's role.
    pub role: Role,
    /// Whether the user is supporting TETR.IO.
    #[serde(rename = "supporter")]
    #[serde(default)] // If the field is missing, it is false.
    pub is_supporter: bool,
    /// The user's country, if public.
    pub country: Option<String>,
}

impl PartialUser {
    /// Gets the detailed information about the user.
    ///
    /// # Errors
    ///
    /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    /// if the request failed.
    /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    /// if the response did not match the expected format but the HTTP request succeeded.
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    /// if the HTTP request failed and the response did not match the expected format.
    /// Even if the HTTP request failed,
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        self.id.get_user().await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Whether the user is a normal user.
    pub fn is_normal_user(&self) -> bool {
        self.role.is_normal_user()
    }

    /// Whether the user is an anonymous.
    pub fn is_anon(&self) -> bool {
        self.role.is_anon()
    }

    /// Whether the user is a bot.
    pub fn is_bot(&self) -> bool {
        self.role.is_bot()
    }

    /// Whether the user is a SYSOP.
    pub fn is_sysop(&self) -> bool {
        self.role.is_sysop()
    }

    /// Whether the user is an administrator.
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Whether the user is a moderator.
    pub fn is_mod(&self) -> bool {
        self.role.is_mod()
    }

    /// Whether the user is a community moderator.
    pub fn is_halfmod(&self) -> bool {
        self.role.is_halfmod()
    }

    /// Whether the user is banned.
    pub fn is_banned(&self) -> bool {
        self.role.is_banned()
    }

    /// Whether the user is hidden.
    pub fn is_hidden(&self) -> bool {
        self.role.is_hidden()
    }

    /// Returns the national flag URL of the user's country.
    ///
    /// If the user's country is not public, `None` is returned.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }
}

impl AsRef<PartialUser> for PartialUser {
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
