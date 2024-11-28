//! A model for the record leaderboards.

use crate::client::param::record_leaderboard::{RecordsLeaderboardId, Scope};
use serde::Deserialize;
use std::fmt;

/// A record leaderboard.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct RecordLeaderboard(pub String);

impl RecordLeaderboard {
    /// Converts into a [`crate::client::param::record_leaderboard::RecordsLeaderboardId`].
    pub fn into_id(&self) -> RecordsLeaderboardId {
        self._into_id(None)
    }

    /// Converts into a [`crate::client::param::record_leaderboard::RecordsLeaderboardId`] with a Revolution ID.
    pub fn into_id_with_revolution_id(&self, revolution_id: &str) -> RecordsLeaderboardId {
        self._into_id(Some(revolution_id))
    }

    /// Converts into a [`crate::client::param::record_leaderboard::RecordsLeaderboardId`] with an optional Revolution ID.
    fn _into_id(&self, revolution_id: Option<&str>) -> RecordsLeaderboardId {
        let split_id: Vec<&str> = self.0.split('_').collect();
        let gamemode = split_id[0];
        let scope = match split_id[1] {
            "global" => Scope::Global,
            _ => Scope::Country(split_id[2].to_string()),
        };
        RecordsLeaderboardId::new(gamemode, scope, revolution_id)
    }
}

impl AsRef<RecordLeaderboard> for RecordLeaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for RecordLeaderboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
