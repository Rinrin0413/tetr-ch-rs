//! Models for the endpoints "User Leaderboard", "Historical User Leaderboard".
//!
//! - About the endpoint "User Leaderboard",
//!   see the [API document](https://tetr.io/about/api/#usersbyleaderboard).
//! - About the endpoint "Historical User Leaderboard",
//!   see the [API document](https://tetr.io/about/api/#usershistoryleaderboardseason).

use crate::model::{prelude::*, user::AchievementRatingCounts};

/// An array of users. (user leaderboard)
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Leaderboard {
    /// The matched users.
    pub entries: Vec<LeaderboardUser>,
}

impl AsRef<Leaderboard> for Leaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// User data in a user leaderboard.
/// This is used as an entry in the [`Leaderboard`] struct,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardUser {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's role.
    pub role: Role,
    /// When the user account was created.
    /// If not set, this account was created before join dates were recorded.
    #[serde(rename = "ts")]
    pub created_at: Option<Timestamp>,
    /// The user's XP in points.
    pub xp: f64,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown.
    /// Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user is currently supporting TETR.IO <3
    #[serde(rename = "supporter")]
    #[serde(default)] // If the field is missing, it is false.
    pub is_supporter: bool,
    /// This user's current TETRA LEAGUE standing.
    pub league: PartialLeagueData,
    /// The amount of online games played by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gamesplayed")]
    pub online_games_played: i32,
    /// The amount of online games won by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gameswon")]
    pub online_games_won: i32,
    /// The amount of seconds this user spent playing, both on- and offline.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gametime")]
    pub game_time: f64,
    /// This user's Achievement Rating.
    #[serde(rename = "ar")]
    pub achievement_rating: i32,
    /// The breakdown of the source of this user's Achievement Rating.
    #[serde(rename = "ar_counts")]
    pub achievement_rating_counts: AchievementRatingCounts,
    /// The prisecter of this entry.
    ///
    /// A **prisecter** is consisting of three floats.
    /// It allows you to continue paginating.
    #[serde(rename = "p")]
    pub prisecter: Prisecter,
}

impl LeaderboardUser {
    impl_get_user!(id);
    impl_for_xp!();
    impl_for_username!();
    impl_for_role!();
    impl_for_account_created_at!();
    impl_for_country!();
}

impl AsRef<LeaderboardUser> for LeaderboardUser {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Partial summary of a user's TETRA LEAGUE standing.
/// This is used in the [`LeaderboardUser`] struct,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PartialLeagueData {
    /// The amount of TETRA LEAGUE games played by this user.
    #[serde(rename = "gamesplayed")]
    pub games_played: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    #[serde(rename = "gameswon")]
    pub games_won: u32,
    /// This user's TR (Tetra Rating).
    pub tr: f64,
    /// This user's GLIXARE.
    pub gxe: f64,
    /// This user's rank.
    pub rank: Rank,
    /// This user's highest achieved rank this season.
    ///
    /// ***The API document does not say this field is optional.**
    #[serde(rename = "bestrank")]
    pub best_rank: Option<Rank>,
    /// This user's Glicko-2 rating.
    pub glicko: f64,
    /// This user's Glicko-2 Rating Deviation.
    ///
    /// ***The API document does not say this field is optional.**
    pub rd: Option<f64>,
    /// This user's average APM (attack per minute) over the last 10 games.
    ///
    /// ***The API document does not say this field is optional.**
    pub apm: Option<f64>,
    /// This user's average PPS (pieces per second) over the last 10 games.
    ///
    /// ***The API document does not say this field is optional.**
    pub pps: Option<f64>,
    /// This user's average VS (versus score) over the last 10 games.
    ///
    /// ***The API document does not say this field is optional.**
    pub vs: Option<f64>,
    /// Whether this user's RD is rising (has not played in the last week).
    #[serde(rename = "decaying")]
    pub is_decaying: bool,
}

impl AsRef<PartialLeagueData> for PartialLeagueData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An array of historical user blobs. (user leaderboard)
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct HistoricalLeaderboard {
    /// The matched historical user blobs.
    pub entries: Vec<PastUserWithPrisecter>,
}

impl AsRef<HistoricalLeaderboard> for HistoricalLeaderboard {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Past season final placement information of a user, with a [`Prisecter`].
/// This is used as an entry in the [`HistoricalLeaderboard`] struct,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PastUserWithPrisecter {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The season ID.
    pub season: String,
    /// The username the user had at the time.
    pub username: String,
    /// The country the user represented at the time.
    pub country: Option<String>,
    /// This user's final position in the season's global leaderboards.
    pub placement: i32,
    /// Whether the user was ranked at the time of the season's end.
    #[serde(rename = "ranked")]
    pub is_ranked: bool,
    /// The amount of TETRA LEAGUE games played by this user.
    #[serde(rename = "gamesplayed")]
    pub games_played: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    #[serde(rename = "gameswon")]
    pub games_won: u32,
    /// This user's final Glicko-2 rating.
    pub glicko: f64,
    /// This user's final Glicko-2 Rating Deviation.
    pub rd: f64,
    /// This user's final TR (Tetra Rating).
    pub tr: f64,
    /// This user's final GLIXARE score (a % chance of beating an average player).
    pub gxe: f64,
    /// This user's final letter rank. z is unranked.
    pub rank: Rank,
    /// This user's highest achieved rank in the season.
    #[serde(rename = "bestrank")]
    pub best_rank: Option<Rank>,
    /// This user's average APM (attack per minute) over the last 10 games in the season.
    pub apm: f64,
    /// This user's average PPS (pieces per second) over the last 10 games in the season.
    pub pps: f64,
    /// This user's average VS (versus score) over the last 10 games in the season.
    pub vs: f64,
    /// The prisecter of this entry.
    ///
    /// A **prisecter** is consisting of three floats.
    /// It allows you to continue paginating.
    #[serde(rename = "p")]
    pub prisecter: Prisecter,
}

impl PastUserWithPrisecter {
    impl_get_user!(id);
    impl_for_country!();
}

impl AsRef<PastUserWithPrisecter> for PastUserWithPrisecter {
    fn as_ref(&self) -> &Self {
        self
    }
}
