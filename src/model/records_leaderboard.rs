//! Models for the endpoint "Records Leaderboard".
//!
//! About the endpoint "Records Leaderboard",
//! see the [API document](https://tetr.io/about/api/#recordsleaderboard).

use crate::model::prelude::*;

/// An array of records.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RecordsLeaderboard {
    /// The matched records.
    pub entries: Vec<Record>,
}
