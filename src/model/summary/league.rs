//! Models for the endpoint "User Summary: TETRA LEAGUE".
//!
//! About the endpoint "User Summary: TETRA LEAGUE",
//! see the [API document](https://tetr.io/about/api/#usersusersummariesleague).

use crate::model::{cache::CacheData, error_response::ErrorResponse, util::league_rank::Rank};
use serde::Deserialize;
use std::collections::HashMap;

/// A struct for the response for the endpoint "User Summary: TETRA LEAGUE".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<LeagueData>,
}

impl AsRef<LeagueResponse> for LeagueResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A struct that describes a summary of a user's TETRA LEAGUE standing.
///
/// Season information is only saved if the user had finished placements in the season,
/// and was not banned or hidden.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueData {
    /// The amount of TETRA LEAGUE games played by this user.
    #[serde(rename = "gamesplayed")]
    pub games_played: u32,
    ///  The amount of TETRA LEAGUE games won by this user.
    #[serde(rename = "gameswon")]
    pub games_won: u32,
    /// This user's Glicko-2 rating, or -1 if less than 10 games were played.
    pub glicko: f64,
    /// This user's Glicko-2 Rating Deviation,or -1 if less than 10 games were played.
    /// If over 100, this user is unranked.
    pub rd: Option<f64>,
    /// Whether this user's RD is rising (has not played in the last week).
    pub is_decaying: bool,
    /// This user's TR (Tetra Rating), or -1 if less than 10 games were played.
    pub tr: f64,
    /// This user's GLIXARE score (a % chance of beating an average player),
    /// or -1 if less than 10 games were played.
    pub gxe: f64,
    ///  This user's letter rank. z is unranked.
    pub rank: Rank,
    /// This user's highest achieved rank this season.
    #[serde(rename = "bestrank")]
    pub best_rank: Option<Rank>,
    /// This user's average APM (attack per minute) over the last 10 games.
    pub apm: Option<f64>,
    /// This user's average PPS (pieces per second) over the last 10 games.
    pub pps: Option<f64>,
    /// This user's average VS (versus score) over the last 10 games.
    pub vs: Option<f64>,
    /// This user's position in global leaderboards, or -1 if not applicable.
    pub standing: Option<i32>,
    /// This user's position in local leaderboards, or -1 if not applicable.
    pub standing_local: Option<i32>,
    /// This user's percentile position (0 is best, 1 is worst).
    pub percentile: Option<f64>,
    /// This user's percentile rank, or z if not applicable.
    pub percentile_rank: Option<Rank>,
    /// The next rank this user can achieve, if they win more games,
    /// or `None` if unranked (or the best rank).
    pub next_rank: Option<Rank>,
    /// The previous rank this user can achieve, if they lose more games,
    /// or null if unranked (or the worst rank).
    pub prev_rank: Option<Rank>,
    /// The position of the best player in the user's current rank,
    /// surpass them to go up a rank. -1 if unranked (or the best rank).
    pub next_at: Option<i32>,
    /// The position of the worst player in the user's current rank,
    /// dip below them to go down a rank. -1 if unranked (or the worst rank).
    pub prev_at: Option<i32>,
    /// An object mapping past season IDs to past season final placement information.
    pub past: HashMap<String, PastUser>,
}

impl LeagueData {
    /// Returns the user's progress percentage in the rank.
    ///
    /// But there are cases where values less than 0 or greater than 100 are returned,
    /// because the rank boundaries are fluctuating.  
    /// (e.g. `-88.5` `104.9`, `-0.0`)
    ///
    /// If there is no user's position in global leaderboards,
    /// `None` is returned.
    pub fn rank_progress(&self) -> Option<f64> {
        if let (Some(standing), Some(prev_at), Some(next_at)) =
            (self.standing, self.prev_at, self.next_at)
        {
            if prev_at < 0 || next_at < 0 {
                return None;
            }
            let current_standing = standing as f64;
            let prev_at = prev_at as f64;
            let next_at = next_at as f64;
            return Some((current_standing - prev_at) / (next_at - prev_at) * 100.);
        }
        None
    }
}

impl AsRef<LeagueData> for LeagueData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Past season final placement information of a user.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PastUser {
    /// The season ID.
    pub season: String,
    /// The username the user had at the time.
    pub username: String,
    /// The country the user represented at the time.
    pub country: Option<String>,
    /// This user's final position in the season's global leaderboards.
    pub placement: Option<i32>,
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
}

impl PastUser {
    /// Returns the national flag URL of the user's country.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }
}

impl AsRef<PastUser> for PastUser {
    fn as_ref(&self) -> &Self {
        self
    }
}
