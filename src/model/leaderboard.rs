//! Models for the endpoints "User Leaderboard", "Historical User Leaderboard".
//!
//! - About the endpoint "User Leaderboard",
//! see the [API document](https://tetr.io/about/api/#usersbyleaderboard).
//! - About the endpoint "Historical User Leaderboard",
//! see the [API document](https://tetr.io/about/api/#usershistoryleaderboardseason).

use crate::{
    client::{error::RspErr, param::pagination::Prisecter},
    model::{
        cache::CacheData,
        error_response::ErrorResponse,
        league_rank::Rank,
        role::Role,
        user::{AchievementRatingCounts, UserId, UserResponse},
    },
    util::{max_f64, to_unix_ts},
};
use serde::Deserialize;

/// A struct for the response for the endpoint "User Leaderboard".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<Leaderboard>,
}

impl AsRef<LeaderboardResponse> for LeaderboardResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An array of users. (user leaderboard)
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Leaderboard {
    /// The matched users.
    pub entries: Vec<LeaderboardUser>,
}

impl AsRef<Leaderboard> for Leaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// User data in a user leaderboard.
/// This is used as an entry in the [`Leaderboard`] struct,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardUser {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's role.
    pub role: Role,
    /// When the user account was created.
    /// If not set, this account was created before join dates were recorded.
    #[serde(rename = "ts")]
    pub account_created_at: Option<String>,
    /// The user's XP in points.
    pub xp: f64,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown.
    /// Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user is currently supporting TETR.IO <3
    #[serde(rename = "supporter")]
    #[serde(default)] // If the field is missing, it is false.
    pub is_supporter: bool,
    /// This user's current TETRA LEAGUE standing.
    pub league: PartialLeagueData,
    /// The amount of online games played by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gamesplayed")]
    pub online_games_played: i32,
    /// The amount of online games won by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gameswon")]
    pub online_games_won: i32,
    /// The amount of seconds this user spent playing, both on- and offline.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gametime")]
    pub game_time: f64,
    /// This user's Achievement Rating.
    #[serde(rename = "ar")]
    pub achievement_rating: i32,
    /// The breakdown of the source of this user's Achievement Rating.
    #[serde(rename = "ar_counts")]
    pub achievement_rating_counts: AchievementRatingCounts,
    /// The prisecter of this entry.
    ///
    /// A **prisecter** is consisting of three floats.
    /// It allows you to continue paginating.
    #[serde(rename = "p")]
    pub prisecter: Prisecter,
}

impl LeaderboardUser {
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

    /// Returns the level of the user.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
            as u32
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

    /// Returns a UNIX timestamp when the user account was created.
    ///
    /// If this account was created before join dates were recorded,
    /// `None` is returned.
    ///
    /// # Panics
    ///
    /// Panics if failed to parse the timestamp.
    pub fn account_created_at(&self) -> Option<i64> {
        self.account_created_at.as_ref().map(|ts| to_unix_ts(ts))
    }

    /// Returns the national flag URL of the user's country.
    ///
    /// If the user's country is hidden or unknown, `None` is returned.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }
}

impl AsRef<LeaderboardUser> for LeaderboardUser {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Partial summary of a user's TETRA LEAGUE standing.
/// This is used in the [`LeaderboardUser`] struct,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PartialLeagueData {
    /// The amount of TETRA LEAGUE games played by this user.
    #[serde(rename = "gamesplayed")]
    pub games_played: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    #[serde(rename = "gameswon")]
    pub games_won: u32,
    /// This user's TR (Tetra Rating).
    pub tr: f64,
    /// This user's GLIXARE.
    pub gxe: f64,
    /// This user's rank.
    pub rank: Rank,
    /// This user's highest achieved rank this season.
    #[serde(rename = "bestrank")]
    pub best_rank: Rank,
    /// This user's Glicko-2 rating.
    pub glicko: f64,
    /// This user's Glicko-2 Rating Deviation.
    pub rd: f64,
    /// This user's average APM (attack per minute) over the last 10 games.
    pub apm: f64,
    /// This user's average PPS (pieces per second) over the last 10 games.
    pub pps: f64,
    /// This user's average VS (versus score) over the last 10 games.
    pub vs: f64,
    /// Whether this user's RD is rising (has not played in the last week).
    #[serde(rename = "decaying")]
    pub is_decaying: bool,
}

impl AsRef<PartialLeagueData> for PartialLeagueData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A struct for the response for the endpoint "Historical User Leaderboard".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HistoricalLeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<HistoricalLeaderboard>,
}

impl AsRef<HistoricalLeaderboardResponse> for HistoricalLeaderboardResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An array of historical user blobs. (user leaderboard)
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HistoricalLeaderboard {
    /// The matched historical user blobs.
    pub entries: Vec<PastUserWithPrisecter>,
}

impl AsRef<HistoricalLeaderboard> for HistoricalLeaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Past season final placement information of a user, with a [`Prisecter`].
/// This is used as an entry in the [`HistoricalLeaderboard`] struct,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PastUserWithPrisecter {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The season ID.
    pub season: String,
    /// The username the user had at the time.
    pub username: String,
    /// The country the user represented at the time.
    pub country: Option<String>,
    /// This user's final position in the season's global leaderboards.
    pub placement: i32,
    /// Whether the user was ranked at the time of the season's end.
    #[serde(rename = "ranked")]
    pub is_ranked: bool,
    /// The amount of TETRA LEAGUE games played by this user.
    #[serde(rename = "gamesplayed")]
    pub games_played: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    #[serde(rename = "gameswon")]
    pub games_won: u32,
    /// This user's final Glicko-2 rating.
    pub glicko: f64,
    /// This user's final Glicko-2 Rating Deviation.
    pub rd: f64,
    /// This user's final TR (Tetra Rating).
    pub tr: f64,
    /// This user's final GLIXARE score (a % chance of beating an average player).
    pub gxe: f64,
    /// This user's final letter rank. z is unranked.
    pub rank: Rank,
    /// This user's highest achieved rank in the season.
    #[serde(rename = "bestrank")]
    pub best_rank: Option<Rank>,
    /// This user's average APM (attack per minute) over the last 10 games in the season.
    pub apm: f64,
    /// This user's average PPS (pieces per second) over the last 10 games in the season.
    pub pps: f64,
    /// This user's average VS (versus score) over the last 10 games in the season.
    pub vs: f64,
    /// The prisecter of this entry.
    ///
    /// A **prisecter** is consisting of three floats.
    /// It allows you to continue paginating.
    #[serde(rename = "p")]
    pub prisecter: Prisecter,
}

impl PastUserWithPrisecter {
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

    /// Returns the national flag URL of the user's country.
    ///
    /// If the user's country is hidden or unknown, `None` is returned.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }
}

impl AsRef<PastUserWithPrisecter> for PastUserWithPrisecter {
    fn as_ref(&self) -> &Self {
        self
    }
}
