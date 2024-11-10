//! The User Leaderboard models.

use crate::{
    client::param::pagination::Prisecter,
    model::{
        cache::CacheData,
        league_rank::Rank,
        role::Role,
        user::{AchievementRatingCounts, UserId},
    },
    util::{max_f64, to_unix_ts},
};
use serde::Deserialize;

/// The response for the User Leaderboard data.
///
/// An array of users fulfilling the search criteria.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
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

/// The User Leaderboard data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Leaderboard {
    /// The matched users.
    pub entries: Vec<LeaderboardEntry>,
}

impl AsRef<Leaderboard> for Leaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An entry in the User Leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardEntry {
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
    pub is_supporter: Option<bool>, // EXCEPTION
    /// This user's current TETRA LEAGUE standing.
    pub league: League,
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

impl LeaderboardEntry {
    /// Whether this user is an anonymous.
    pub fn is_anon(&self) -> bool {
        self.role.is_anon()
    }

    /// Whether this user is a bot.
    pub fn is_bot(&self) -> bool {
        self.role.is_bot()
    }

    /// Whether this user is a SYSOP.
    pub fn is_sysop(&self) -> bool {
        self.role.is_sysop()
    }

    /// Whether this user is an administrator.
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Whether this user is a moderator.
    pub fn is_mod(&self) -> bool {
        self.role.is_mod()
    }

    /// Whether this user is a community moderator.
    pub fn is_halfmod(&self) -> bool {
        self.role.is_halfmod()
    }

    /// Whether this user is banned.
    pub fn is_banned(&self) -> bool {
        self.role.is_banned()
    }

    /// Whether this user is hidden.
    pub fn is_hidden(&self) -> bool {
        self.role.is_hidden()
    }

    /// Returns an UNIX timestamp of when the account was created.
    /// If this account was created before join dates were recorded,
    /// returns `None`.
    pub fn account_created_at(&self) -> Option<i64> {
        self.account_created_at.as_ref().map(|ts| to_unix_ts(ts))
    }

    /// Returns the level based on the user's xp.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
            as u32
    }

    /// Returns the national flag URL of the user's country.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }

    /// Whether this user is a supporter.
    pub fn is_supporter(&self) -> bool {
        self.is_supporter.unwrap_or(false)
    }
}

impl AsRef<LeaderboardEntry> for LeaderboardEntry {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's current TETRA LEAGUE standing.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct League {
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

impl AsRef<League> for League {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The response for the Historical User Leaderboard data.
///
/// An array of historical user blobs fulfilling the search criteria.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HistoricalLeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
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

/// The Historical User Leaderboard data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HistoricalLeaderboard {
    /// The matched historical user blobs.
    pub entries: Vec<HistoricalEntry>,
}

impl AsRef<HistoricalLeaderboard> for HistoricalLeaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An entry in the Historical User Leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HistoricalEntry {
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

impl AsRef<HistoricalEntry> for HistoricalEntry {
    fn as_ref(&self) -> &Self {
        self
    }
}
