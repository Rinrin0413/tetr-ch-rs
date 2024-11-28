//! Models for the record data.
//!
//! For more details, see the [API document](https://tetr.io/about/api/#recorddata).

use crate::{
    client::{error::RspErr, param::pagination::Prisecter},
    model::{
        league_rank::Rank,
        user::UserResponse,
        util::{
            gamemode::Gamemode, record_leaderboard::RecordLeaderboard, replay_id::ReplayId,
            timestamp::Timestamp, user_id::UserId,
        },
    },
};
use serde::Deserialize;
use std::collections::HashMap;

/// A record data.
/// Includes achieved scores and matches.
///
/// ***This structure may be changed drastically at any time.**  
/// For more details, see the [API document](https://tetr.io/about/api/#recorddata).
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Record {
    /// The Record's ID.
    #[serde(rename = "_id")]
    pub id: String,
    /// The Record's ReplayID.
    #[serde(rename = "replayid")]
    pub replay_id: ReplayId,
    /// Whether the Replay has been pruned.
    #[serde(rename = "stub")]
    pub is_stub: bool,
    /// The played game mode.
    #[serde(rename = "gamemode")]
    pub game_mode: Gamemode,
    /// Whether this is the user's current personal best in the game mode.
    #[serde(rename = "pb")]
    pub is_personal_best: bool,
    /// Whether this was once the user's personal best in the game mode.
    #[serde(rename = "oncepb")]
    pub has_been_personal_best: bool,
    /// The time the Record was submitted.
    #[serde(rename = "ts")]
    pub submitted_at: Timestamp,
    /// If revolved away, the revolution it belongs to.
    pub revolution: Option<String>,
    /// The user owning the Record.
    pub user: Option<PartialUser>, // EXCEPTION
    /// Other users mentioned in the Record.
    ///
    /// If not empty, this is a multiplayer game
    /// (this changes the enumerator of the [`Record::results`] field).
    #[serde(rename = "otherusers")]
    pub other_users: Vec<PartialUser>,
    /// The leaderboards this Record is mentioned in.
    ///
    /// e.g. `["40l_global", "40l_country_JP"]`
    pub leaderboards: Vec<RecordLeaderboard>,
    /// Whether this Record is disputed.
    #[serde(rename = "disputed")]
    pub is_disputed: bool,
    /// The results of this Record.
    pub results: Results,
    /// Extra metadata for this Record:
    pub extras: Extras,
    /// The prisecter of this entry
    /// if this record is part of a paginated response.
    ///
    /// A **prisecter** is consisting of three floats.
    /// It allows you to continue paginating.
    #[serde(rename = "p")]
    pub prisecter: Option<Prisecter>,
}

impl Record {
    /// Returns the replay URL.
    pub fn replay_url(&self) -> String {
        self.replay_id.replay_url()
    }

    /// Returns a UNIX timestamp when the record was submitted.
    ///
    /// # Panics
    ///
    /// Panics if failed to parse the timestamp.
    pub fn submitted_at(&self) -> i64 {
        self.submitted_at.unix_ts()
    }
}

impl AsRef<Record> for Record {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Partial information about a user.
/// This is used in the [`Record`] struct.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PartialUser {
    /// The user's user ID.
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's avatar revision (for obtaining avatar URLs).
    pub avatar_revision: Option<u64>,
    /// The user's banner revision (for obtaining banner URLs).
    pub banner_revision: Option<u64>,
    /// The user's country, if public.
    pub country: Option<String>,
    /// Whether the user is supporting TETR.IO.
    #[serde(rename = "supporter")]
    #[serde(default)] // If the field is missing, it is false.
    pub is_supporter: bool,
}

impl PartialUser {
    /// Gets the detailed information about the user.
    ///
    /// # Errors
    ///
    /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    /// if the request failed.
    /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    /// if the response did not match the expected format but the HTTP request succeeded.
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    /// if the HTTP request failed and the response did not match the expected format.
    /// Even if the HTTP request failed,
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        self.id.get_user().await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Returns the user's avatar URL.
    ///
    /// If the user does not have an avatar, the anonymous's avatar URL is returned.
    pub fn avatar_url(&self) -> String {
        let default = "https://tetr.io/res/avatar.png".to_string();
        if let Some(ar) = self.avatar_revision {
            if ar == 0 {
                return default;
            }
            format!(
                "https://tetr.io/user-content/avatars/{}.jpg?rv={}",
                self.id, ar
            )
        } else {
            default
        }
    }

    /// Returns the user's banner URL.
    ///
    /// If the user does not have a banner, `None` is returned.
    ///
    /// ***Ignore the returned value if the user is not a supporter.
    /// Because even if the user is not currently a supporter,
    /// `Some<String>` may be returned if the banner was once set.**
    pub fn banner_url(&self) -> Option<String> {
        if let Some(br) = self.banner_revision {
            if br == 0 {
                return None;
            }
            Some(format!(
                "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                self.id, br
            ))
        } else {
            None
        }
    }

    /// Returns the national flag URL of the user's country.
    ///
    /// If the user's country is hidden or unknown, `None` is returned.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }
}

impl AsRef<PartialUser> for PartialUser {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Results of a Record.
///
/// If [`Record::other_users`] is empty, this is [`SinglePlayer`](`Results::SinglePlayer`).
/// Otherwise, this is [`MultiPlayer`](`Results::MultiPlayer`).
///
/// ***This structure may be changed drastically at any time.
/// See the [official API document](https://tetr.io/about/api/#recorddata) for more information.**
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Results {
    /// Results for a single-player games.
    SinglePlayer(SinglePlayerResults),
    /// Results for a multi-player games.
    MultiPlayer(MultiPlayerResults),
    /// Unknown structure.
    Unknown(serde_json::Value),
}

impl Results {
    /// Whether the results are for a single-player game.
    pub fn is_single_play(&self) -> bool {
        matches!(self, Results::SinglePlayer(_))
    }

    /// Whether the results are for a multi-player game.
    pub fn is_multi_play(&self) -> bool {
        matches!(self, Results::MultiPlayer(_))
    }

    /// Whether the structure of the results is unknown.
    pub fn is_unknown_structure(&self) -> bool {
        matches!(self, Results::Unknown(_))
    }
}

impl AsRef<Results> for Results {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Results for a single-player games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SinglePlayerResults {
    /// The final stats of the game played.
    #[serde(rename = "stats")]
    pub final_stats: serde_json::Value,
    /// Aggregate stats of the game played.
    #[serde(rename = "aggregatestats")]
    pub aggregate_stats: serde_json::Value,
    /// The reason the game has ended.
    #[serde(rename = "gameoverreason")]
    pub game_over_reason: String,
}

impl AsRef<SinglePlayerResults> for SinglePlayerResults {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Results of a multi-player games.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct MultiPlayerResults {
    /// The final leaderboard at the end of the match.
    pub leaderboard: Vec<PlayerStats>,
    /// The scoreboards for every round.
    pub rounds: Vec<Vec<PlayerStatsRound>>,
}

impl AsRef<MultiPlayerResults> for MultiPlayerResults {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Stats of a player in a multi-player game.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PlayerStats {
    /// The player's User ID.
    pub id: UserId,
    /// The player's username.
    pub username: String,
    /// Whether the player is still in the game.
    /// If false, the user has likely been disqualified.
    #[serde(rename = "active")]
    pub is_active: bool,
    /// The amount of rounds won by the player.
    pub wins: u32,
    /// The aggregate stats across all rounds.
    pub stats: serde_json::Value,
}

impl PlayerStats {
    /// Gets the detailed information about the user.
    ///
    /// # Errors
    ///
    /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    /// if the request failed.
    /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    /// if the response did not match the expected format but the HTTP request succeeded.
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    /// if the HTTP request failed and the response did not match the expected format.
    /// Even if the HTTP request failed,
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        self.id.get_user().await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }
}

impl AsRef<PlayerStats> for PlayerStats {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Stats of a round in a multi-player game.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PlayerStatsRound {
    /// The player's User ID.
    pub id: UserId,
    /// The player's username.
    pub username: String,
    /// Whether the player is still in the game.
    /// If false, the user has likely been disqualified for the round.
    #[serde(rename = "active")]
    pub is_active: bool,
    /// Whether the player made it through the round alive.
    #[serde(rename = "alive")]
    pub is_alive: bool,
    /// The time alive in this match.
    pub lifetime: u32,
    /// The aggregate stats for the player for this round.
    pub stats: serde_json::Value,
}

impl PlayerStatsRound {
    /// Gets the detailed information about the user.
    ///
    /// # Errors
    ///
    /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    /// if the request failed.
    /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    /// if the response did not match the expected format but the HTTP request succeeded.
    /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
    /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    /// if the HTTP request failed and the response did not match the expected format.
    /// Even if the HTTP request failed,
    /// it may be possible to deserialize the response containing an error message,
    /// so the deserialization will be tried before returning this error.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        self.id.get_user().await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }
}

impl AsRef<PlayerStatsRound> for PlayerStatsRound {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Extra metadata for a Record.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Extras {
    /// A mapping of user IDs to before-and-afters, if user is being ranked.
    pub league: Option<HashMap<UserId, Vec<PlayerExtraStats>>>,
    /// The result of the game, from the owner's point of view.
    pub result: Option<String>,
    /// Extra data for QUICK PLAY,
    pub zenith: Option<Zenith>,
}

impl AsRef<Extras> for Extras {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Extra stats for a player.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PlayerExtraStats {
    /// The Glicko-2 rating of the user.
    pub glicko: f64,
    /// The RD of the user.
    pub rd: f64,
    /// The TR of the user.
    pub tr: f64,
    /// The rank of the user.
    pub rank: Rank,
    /// The user's position in the global leaderboards.
    pub placement: Option<u32>,
}

impl AsRef<PlayerExtraStats> for PlayerExtraStats {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Extra data for QUICK PLAY.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Zenith {
    /// The mods used in the run.
    pub mods: Vec<String>,
}

impl AsRef<Zenith> for Zenith {
    fn as_ref(&self) -> &Self {
        self
    }
}
