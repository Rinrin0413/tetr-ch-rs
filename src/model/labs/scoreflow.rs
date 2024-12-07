//! Models for the endpoint "Labs Scoreflow".
//!
//! About the endpoint "Labs Scoreflow",
//! see the [API document](https://tetr.io/about/api/#labsscoreflowusergamemode).

use crate::model::prelude::*;

/// A struct for the response for the endpoint "Labs Scoreflow".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LabsScoreflowResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<LabsScoreflow>,
}

impl AsRef<LabsScoreflowResponse> for LabsScoreflowResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A condensed graph of all of a user's records in a gamemode.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LabsScoreflow {
    /// The timestamp of the oldest record found.
    #[serde(rename = "startTime")]
    pub oldest_record_ts: i64,
    /// The points in the chart.
    ///
    /// - 0: The timestamp offset.
    ///   Add the [`LabsScoreflow::oldest_record_ts`] to get the true timestamp.
    /// - 1: Whether the score set was a Personal Best.
    ///   0 = not a Personal Best, 1 = Personal Best.
    /// - 2: The score achieved. (For 40 LINES, this is negative.)
    pub points: Vec<[i64; 3]>,
}

impl AsRef<LabsScoreflow> for LabsScoreflow {
    fn as_ref(&self) -> &Self {
        self
    }
}
