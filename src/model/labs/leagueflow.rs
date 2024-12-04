//! Models for the endpoint "Labs Leagueflow".
//!
//! About the endpoint "Labs Leagueflow",
//! see the [API document](https://tetr.io/about/api/#labsleagueflowuser).

use crate::model::prelude::*;

/// A struct for the response for the endpoint "Labs Leagueflow".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LabsLeagueflowResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<LabsLeagueflow>,
}

impl AsRef<LabsLeagueflowResponse> for LabsLeagueflowResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A condensed graph of all of a user's matches in TETRA LEAGUE.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LabsLeagueflow {
    /// The timestamp of the oldest record found.
    #[serde(rename = "startTime")]
    pub oldest_record_ts: i64,
    /// The points in the chart.
    ///
    /// - 0: The timestamp offset.
    /// Add the [`LabsLeagueflow::oldest_record_ts`] to get the true timestamp.
    /// - 1: The result of the match, where:
    ///     - 1: victory
    ///     - 2: defeat
    ///     - 3: victory by disqualification
    ///     - 4: defeat by disqualification
    ///     - 5: tie
    ///     - 6: no contest
    ///     - 7: match nullified
    /// - 2: The user's TR after the match.
    /// - 3: The opponent's TR before the match.
    /// (If the opponent was unranked, same as 2.)
    pub points: Vec<[i64; 4]>,
}

impl AsRef<LabsLeagueflow> for LabsLeagueflow {
    fn as_ref(&self) -> &Self {
        self
    }
}
