//! Models for the endpoint "User Summary: BLITZ".
//!
//! About the endpoint "User Summary: BLITZ",
//! see the [API document](https://tetr.io/about/api/#usersusersummariesblitz).

use crate::model::prelude::*;

/// A struct that describes a summary of a user's BLITZ games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Blitz {
    /// The user's BLITZ record, or `None` if never played.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or -1 if not in global leaderboards.
    pub rank: i32,
    /// The user's rank in their country's leaderboards, or -1 if not in any.
    pub rank_local: i32,
}

impl AsRef<Blitz> for Blitz {
    fn as_ref(&self) -> &Self {
        self
    }
}
