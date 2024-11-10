//! The User Summary TETRA LEAGUE models.

use crate::model::{cache::CacheData, league_rank::Rank};
use serde::Deserialize;
use std::collections::HashMap;

/// The response for the User Summary TETRA LEAGUE data.
/// An object describing a summary of the user's TETRA LEAGUE standing.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<League>,
}

impl AsRef<LeagueResponse> for LeagueResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The User Summary TETRA LEAGUE data.
///
/// Season information is only saved if the user had finished placements in the season,
/// and was not banned or hidden.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct League {
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
    pub decaying: bool,
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
    pub past: HashMap<String, PastSeason>,
}

impl AsRef<League> for League {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Past season final placement information.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PastSeason {
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

impl AsRef<PastSeason> for PastSeason {
    fn as_ref(&self) -> &Self {
        self
    }
}
