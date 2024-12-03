//! Models for the record data.
//!
//! For more details, see the [API document](https://tetr.io/about/api/#recorddata).

use crate::{
    client::param::pagination::Prisecter,
    model::util::{
        gamemode::Gamemode, league_rank::Rank, record_leaderboard::RecordLeaderboard,
        replay_id::ReplayId, timestamp::Timestamp, user_id::UserId,
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
    ///
    /// ***The API document does not say this field is optional.**
    pub user: Option<PartialUser>,
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
    impl_for_replay_id!();
    impl_for_submitted_at!();
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
    impl_get_user!(id);
    impl_for_username!();
    impl_for_avatar_revision!();
    impl_for_banner_revision!();
    impl_for_country!();
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
    impl_get_user!(id);
    impl_for_username!();
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
    impl_get_user!(id);
    impl_for_username!();
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
