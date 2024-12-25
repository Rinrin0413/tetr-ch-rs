//! Models for the endpoint "User Summary: 40 LINES".
//!
//! About the endpoint "User Summary: 40 LINES",
//! see the [API document](https://tetr.io/about/api/#usersusersummaries40l).

use crate::model::prelude::*;

/// A struct that describes a summary of a user's 40 LINES games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct FortyLines {
    /// The user's 40 LINES record, or `None` if never played.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or -1 if not in global leaderboards.
    pub rank: i32,
    /// The user's rank in their country's leaderboards, or -1 if not in any.
    pub rank_local: i32,
}

impl AsRef<FortyLines> for FortyLines {
    fn as_ref(&self) -> &Self {
        self
    }
}
