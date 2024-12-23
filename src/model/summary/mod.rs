//! Easy-to-use models of the various objects received from the User Summaries API endpoints.

use crate::model::prelude::*;

pub mod blitz;
pub mod forty_lines;
pub mod league;
pub mod record;
pub mod zen;
pub mod zenith;

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
