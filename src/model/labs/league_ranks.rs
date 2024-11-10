//! The Labs League Ranks models.

use crate::model::cache::CacheData;
use serde::Deserialize;

/// The response for the Labs League Ranks data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LabsLeagueRanksResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<LabsLeagueRanks>,
}

impl AsRef<LabsLeagueRanksResponse> for LabsLeagueRanksResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The Labs League Ranks data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LabsLeagueRanks {
    /// The internal ID of the Labs data point.
    #[serde(rename = "_id")]
    pub id: String,
    /// The stream ID the Labs data point belongs to.
    #[serde(rename = "s")]
    pub stream_id: String,
    /// The time at which the data point was created.
    #[serde(rename = "t")]
    pub created_at: String,
    /// The data point.
    pub data: LeagueRanksData,
}

impl AsRef<LabsLeagueRanks> for LabsLeagueRanks {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data point for the Labs League Ranks data.
///
/// If there are any unwrapped ranks, please create an Issue on GitHub.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueRanksData {
    /// The total amount of players.
    pub total: u32,
    /// The data of the X+ rank.
    #[serde(rename = "x+")]
    pub rank_x_plus: RankData,
    /// The data of the X rank.
    #[serde(rename = "x")]
    pub rank_x: RankData,
    /// The data of the U rank.
    #[serde(rename = "u")]
    pub rank_u: RankData,
    /// The data of the SS rank.
    #[serde(rename = "ss")]
    pub rank_ss: RankData,
    /// The data of the S+ rank.
    #[serde(rename = "s+")]
    pub rank_s_plus: RankData,
    /// The data of the S rank.
    #[serde(rename = "s")]
    pub rank_s: RankData,
    /// The data of the S- rank.
    #[serde(rename = "s-")]
    pub rank_s_minus: RankData,
    /// The data of the A+ rank.
    #[serde(rename = "a+")]
    pub rank_a_plus: RankData,
    /// The data of the A rank.
    #[serde(rename = "a")]
    pub rank_a: RankData,
    /// The data of the A- rank.
    #[serde(rename = "a-")]
    pub rank_a_minus: RankData,
    /// The data of the B+ rank.
    #[serde(rename = "b+")]
    pub rank_b_plus: RankData,
    /// The data of the B rank.
    #[serde(rename = "b")]
    pub rank_b: RankData,
    /// The data of the B- rank.
    #[serde(rename = "b-")]
    pub rank_b_minus: RankData,
    /// The data of the C+ rank.
    #[serde(rename = "c+")]
    pub rank_c_plus: RankData,
    /// The data of the C rank.
    #[serde(rename = "c")]
    pub rank_c: RankData,
    /// The data of the C- rank.
    #[serde(rename = "c-")]
    pub rank_c_minus: RankData,
    /// The data of the D+ rank.
    #[serde(rename = "d+")]
    pub rank_d_plus: RankData,
    /// The data of the D rank.
    #[serde(rename = "d")]
    pub rank_d: RankData,
}

impl AsRef<LeagueRanksData> for LeagueRanksData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data for a rank.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RankData {
    /// The leaderboard position required to attain this rank.
    #[serde(rename = "pos")]
    pub position: u32,
    /// The percentile (0~1) this rank is for.
    pub percentile: f64,
    /// The TR required to obtain a leaderboard position that will award this rank.
    #[serde(rename = "tr")]
    pub tr: f64,
    /// The TR this rank will gravitate toward (using de- and inflation zones).
    #[serde(rename = "targettr")]
    pub target_tr: f64,
    /// The average APM across all players in this rank.
    pub apm: Option<f64>,
    /// The average PPS across all players in this rank.
    pub pps: Option<f64>,
    /// The average Versus Score across all players in this rank.
    pub vs: Option<f64>,
    /// The amount of players with this rank.
    pub count: u32,
}

impl AsRef<RankData> for RankData {
    fn as_ref(&self) -> &Self {
        self
    }
}
