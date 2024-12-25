//! Models for the endpoint "Labs Scoreflow".
//!
//! About the endpoint "Labs Scoreflow",
//! see the [API document](https://tetr.io/about/api/#labsscoreflowusergamemode).

use crate::model::prelude::*;

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
