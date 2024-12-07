//! Models for the endpoint "Server Statistics".
//!
//! About the endpoint "Server Statistics",
//! see the [API document](https://tetr.io/about/api/#generalstats).

use crate::model::prelude::*;

/// A struct for the response for the endpoint "Server Statistics".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerStatsResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<ServerStats>,
}

impl AsRef<ServerStatsResponse> for ServerStatsResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Server Statistics about the TETR.IO.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerStats {
    /// The amount of users on the server,
    /// including anonymous accounts.
    #[serde(rename = "usercount")]
    pub user_count: u64,
    /// The amount of users created a second
    /// (through the last minute).
    #[serde(rename = "usercount_delta")]
    pub user_count_delta: f64,
    /// The amount of anonymous accounts on the server.
    #[serde(rename = "anoncount")]
    pub anon_count: u64,
    /// The total amount of accounts ever created
    /// (including pruned anons etc.).
    #[serde(rename = "totalaccounts")]
    pub total_accounts: u64,
    /// The amount of ranked
    /// (visible in TETRA LEAGUE leaderboard) accounts on the server.
    #[serde(rename = "rankedcount")]
    pub ranked_count: u64,
    /// The amount of game records stored on the server.
    #[serde(rename = "recordcount")]
    pub record_count: u64,
    /// The amount of games played across all users,
    /// including both off- and online modes.
    #[serde(rename = "gamesplayed")]
    pub games_play_count: u64,
    /// The amount of games played a second
    /// (through the last minute).
    #[serde(rename = "gamesplayed_delta")]
    pub games_play_count_delta: f64,
    /// The amount of games played across all users,
    /// including both off- and online modes, excluding games that were not completed
    /// (e.g. retries)
    #[serde(rename = "gamesfinished")]
    pub games_finish_count: u64,
    /// The amount of seconds spent playing across all users,
    /// including both off- and online modes.
    #[serde(rename = "gametime")]
    pub play_time: f64,
    /// The amount of keys pressed across all users,
    /// including both off- and online modes.
    pub inputs: u64,
    /// The amount of pieces placed across all users,
    /// including both off- and online modes.
    #[serde(rename = "piecesplaced")]
    pub pieces_place_count: u64,
}

impl ServerStats {
    /// Returns the amount of registered players.
    pub fn registered_players(&self) -> u64 {
        self.user_count - self.anon_count
    }

    /// Returns the amount of minutes spent playing across all users.
    /// including both off- and online modes. 1*60
    pub fn play_time_minutes(&self) -> f64 {
        self.play_time / 60.
    }

    /// Returns the amount of hours spent playing across all users.
    /// including both off- and online modes.
    pub fn play_time_hours(&self) -> f64 {
        self.play_time / 3600.
    }

    /// Returns the amount of days spent playing across all users.
    /// including both off- and online modes.
    pub fn play_time_days(&self) -> f64 {
        self.play_time / 86400.
    }

    /// Returns the amount of months spent playing across all users.
    /// including both off- and online modes.
    pub fn play_time_months(&self) -> f64 {
        self.play_time / 2628000.
    }

    /// Returns the amount of years spent playing across all users.
    /// including both off- and online modes.
    pub fn play_time_years(&self) -> f64 {
        self.play_time / 31536000.0
    }

    /// Returns the average amount of pieces placed per second.
    pub fn avg_pieces_per_second(&self) -> f64 {
        self.pieces_place_count as f64 / self.play_time
    }

    /// Returns the average amount of keys pressed per second.
    pub fn avg_keys_per_second(&self) -> f64 {
        self.inputs as f64 / self.play_time
    }
}

impl AsRef<ServerStats> for ServerStats {
    fn as_ref(&self) -> &Self {
        self
    }
}
