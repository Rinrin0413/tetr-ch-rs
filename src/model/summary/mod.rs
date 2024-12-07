//! Easy-to-use models of the various objects received from the User Summaries API endpoints.

use crate::model::prelude::*;

pub mod achievements;
pub mod blitz;
pub mod forty_lines;
pub mod league;
pub mod record;
pub mod zen;
pub mod zenith;

/// A struct for the response for the endpoint "User Summary: All".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AllSummariesResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<AllSummaries>,
}

impl AsRef<AllSummariesResponse> for AllSummariesResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A struct that contains all summaries of a user in one.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AllSummaries {
    /// The user's 40 LINES summary data.
    #[serde(rename = "40l")]
    pub forty_lines: forty_lines::FortyLines,
    /// The user's BLITZ summary data.
    pub blitz: blitz::Blitz,
    /// The user's QUICK PLAY summary data.
    pub zenith: zenith::Zenith,
    /// The user's EXPERT QUICK PLAY summary data.
    #[serde(rename = "zenithex")]
    pub zenith_ex: zenith::Zenith,
    /// The user's TETRA LEAGUE summary data.
    pub league: league::LeagueData,
    /// The user's ZEN summary data.
    pub zen: zen::Zen,
    /// The user's achievements.
    pub achievements: Vec<Achievement>,
}
