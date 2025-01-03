//! Models for the endpoints "User Summary: QUICK PLAY", "User Summary: EXPERT QUICK PLAY".
//!
//! - About the endpoint "User Summary: QUICK PLAY",
//!   see the [API document](https://tetr.io/about/api/#usersusersummarieszenith).
//! - About the endpoint "User Summary: EXPERT QUICK PLAY",
//!   see the [API document](https://tetr.io/about/api/#usersusersummarieszenithex).

use crate::model::prelude::*;

/// A struct that describes a summary of a user's QUICK PLAY or EXPERT QUICK PLAY games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Zenith {
    /// The user's QUICK PLAY record, or `None` if the user hasn't played this week.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or -1 if not in global leaderboards.
    pub rank: i32,
    /// The user's rank in their country's leaderboards, or -1 if not in any.
    pub rank_local: i32,
    /// The user's career best.
    ///
    /// Career bests are only updated on revolve time
    /// (when the week changes, which is 12AM on Monday, UTC).
    /// This is because if the record is at Floor 10,
    /// the final leaderboard position is considered first
    /// (the mode is multiplayer, after all).
    pub best: ZenithBest,
}

impl AsRef<Zenith> for Zenith {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user's career best QUICK PLAY data.
///
/// Career bests are only updated on revolve time
/// (when the week changes, which is 12AM on Monday, UTC).
/// This is because if the record is at Floor 10,
/// the final leaderboard position is considered first
/// (the mode is multiplayer, after all).
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ZenithBest {
    /// The user's best record, or `None` if the user hasn't placed one yet.
    pub record: Option<Record>,
    /// The rank said record had in global leaderboards at the end of the week,
    /// or -1 if it was not ranked.
    pub rank: i32,
}

impl AsRef<ZenithBest> for ZenithBest {
    fn as_ref(&self) -> &Self {
        self
    }
}
