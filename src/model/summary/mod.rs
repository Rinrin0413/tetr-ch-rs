//! Easy-to-use models of the various objects returned by the User Summaries API endpoint.

use crate::model::{achievement::Achievement, cache::CacheData};
use serde::Deserialize;

pub mod achievements;
pub mod blitz;
pub mod forty_lines;
pub mod league;
pub mod record;
pub mod zen;
pub mod zenith;

/// The response for the User Summary All data.
/// An object containing all the user's summaries in one.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AllSummariesResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
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

/// All the User Summary data.
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
    pub league: league::League,
    /// The user's ZEN summary data.
    pub zen: zen::Zen,
    /// The user's achievements.
    pub achievements: Vec<Achievement>,
}
