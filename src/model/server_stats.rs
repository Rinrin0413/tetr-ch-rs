//! Server stats model.

use crate::model::cache::CacheData;
use serde::Deserialize;

/// The response for the server stats information.
/// Describes the some statistics about the TETR.IO in detail.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerStatsResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested user data.
    pub data: Option<ServerStats>,
}

impl ServerStatsResponse {
    /// Returns the amount of registered players.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn registered_players(&self) -> u64 {
        let ss = self.get_server_stats();
        ss.user_count - ss.anon_count
    }

    /// Returns a UNIX timestamp when this resource was cached.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_at(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_until(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns the reference to the [`&ServerStats`](crate::model::server_stats::ServerStats).
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    fn get_server_stats(&self) -> &ServerStats {
        if let Some(d) = self.data.as_ref() {
            d
        } else {
            panic!("There is no server stats object because the request was not successful.")
        }
    }
}

impl AsRef<ServerStatsResponse> for ServerStatsResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The requested server stats data:
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerStats {
    /// The amount of users on the server,
    /// including anonymous accounts.
    #[serde(rename = "usercount")]
    pub user_count: u64,
    /// The amount of users created a second (through the last minute).
    #[serde(rename = "usercount_delta")]
    pub user_count_delta: f64,
    /// The amount of anonymous accounts on the server.
    #[serde(rename = "anoncount")]
    pub anon_count: u64,
    /// The amount of ranked (visible in TETRA LEAGUE leaderboard) accounts on the server.
    #[serde(rename = "rankedcount")]
    pub ranked_count: u64,
    /// The amount of replays stored on the server.
    #[serde(rename = "replaycount")]
    pub replay_count: u64,
    /// The amount of games played across all users,
    /// including both off- and online modes.
    #[serde(rename = "gamesplayed")]
    pub games_play_count: u64,
    /// The amount of games played a second (through the last minute).
    #[serde(rename = "gamesplayed_delta")]
    pub games_play_count_delta: f64,
    /// The amount of games played across all users,
    /// including both off- and online modes, excluding games that were not completed (e.g. retries)
    #[serde(rename = "gamesfinished")]
    pub games_finish_count: u64,
    /// The amount of seconds spent playing across all users, including both off- and online modes.
    #[serde(rename = "gametime")]
    pub play_time: f64,
    /// The amount of keys pressed across all users, including both off- and online modes.
    pub inputs: u64,
    /// The amount of pieces placed across all users, including both off- and online modes.
    #[serde(rename = "piecesplaced")]
    pub pieces_placed: u64,
}

impl ServerStats {
    /// Returns the amount of registered players.
    pub fn registered_players(&self) -> u64 {
        self.user_count - self.anon_count
    }
}

impl AsRef<ServerStats> for ServerStats {
    fn as_ref(&self) -> &Self {
        self
    }
}
