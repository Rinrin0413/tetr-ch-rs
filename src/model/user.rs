//! Models for the endpoint "User Info".
//!
//! About the endpoint "User Info",
//! see the [API document](https://tetr.io/about/api/#usersuser).

use crate::{
    model::{
        cache::CacheData,
        error_response::ErrorResponse,
        util::{BadgeId, Role, Timestamp, UserId},
    },
    util::deserialize_from_non_str_to_none,
};
use serde::Deserialize;

/// A struct for the response for the endpoint "User Info".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<User>,
}

impl AsRef<UserResponse> for UserResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A struct that describes a user in detail.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct User {
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
    /// If this user is a bot, the bot's operator.
    #[serde(rename = "botmaster")]
    pub bot_master: Option<String>,
    /// The user's badges
    pub badges: Vec<Badge>,
    /// The user's XP in points.
    pub xp: f64,
    /// The amount of online games played by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gamesplayed")]
    pub play_count: i32,
    /// The amount of online games won by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gameswon")]
    pub won_count: i32,
    /// The amount of seconds this user spent playing,both on- and offline.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gametime")]
    pub play_time: f64,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown.
    /// Some vanity flags exist.
    pub country: Option<String>,
    /// Whether the user currently has a bad standing (recently banned).
    #[serde(rename = "badstanding")]
    #[serde(default)] // If the field is missing, it is false.
    pub is_badstanding: bool,
    /// Whether the user is currently supporting TETR.IO <3
    #[serde(rename = "supporter")]
    #[serde(default)] // If the field is missing, it is false.
    pub is_supporter: bool,
    /// An indicator of their total amount supported,
    /// between 0 and 4 inclusive.
    pub supporter_tier: u8,
    /// This user's avatar ID.
    /// We can get their avatar at  
    /// `https://tetr.io/user-content/avatars/{ USERID }.jpg?rv={ AVATAR_REVISION }`.
    pub avatar_revision: Option<u64>,
    /// his user's banner ID.
    /// We can get their banner at  
    /// `https://tetr.io/user-content/banners/{ USERID }.jpg?rv={ BANNER_REVISION }`.  
    /// Ignore this field if the user is not a supporter.
    pub banner_revision: Option<u64>,
    /// This user's "About Me" section.
    /// Ignore this field if the user is not a supporter.
    pub bio: Option<String>,
    /// This user's third party connections.
    pub connections: Connections,
    /// The amount of players who have added this user to their friends list.
    ///
    /// ***The API document does not say this field is optional.**
    pub friend_count: Option<u32>,
    // This user's distinguishment banner.
    pub distinguishment: Option<Distinguishment>,
    /// This user's featured achievements.
    /// Up to three integers which correspond to Achievement IDs.
    pub achievements: Vec<i32>,
    /// This user's Achievement Rating.
    #[serde(rename = "ar")]
    pub achievement_rating: i32,
    /// The breakdown of the source of this user's Achievement Rating.
    #[serde(rename = "ar_counts")]
    pub achievement_rating_counts: AchievementRatingCounts,
}

impl User {
    impl_for_xp!();
    impl_for_username!();
    impl_for_role!();
    impl_for_account_created_at!();

    /// Whether the user has any badges.
    pub fn has_badge(&self) -> bool {
        !self.badges.is_empty()
    }

    /// Returns the number of badges the user has.
    pub fn badge_count(&self) -> usize {
        self.badges.len()
    }

    impl_for_avatar_revision!();
    impl_for_banner_revision!();
    impl_for_country!();
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user's badge.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Badge {
    /// The badge's internal ID,
    /// and the filename of the badge icon
    /// (all PNGs within `/res/badges/`).
    /// Note that badge IDs may include forward slashes.
    /// Please do not encode them!
    /// Follow the folder structure.
    pub id: BadgeId,
    /// The badge's group ID.
    /// If multiple badges have the same group ID, they are rendered together.
    pub group: Option<String>,
    /// The badge's label, shown when hovered.
    pub label: String,
    /// Extra flavor text for the badge, shown when hovered.
    ///
    /// ***The API document does not say this field is optional.**
    pub desc: Option<String>,
    /// The badge's timestamp, if shown.
    ///
    /// Why it uses `deserialize_with` attribute?
    /// See [this issue](https://github.com/Rinrin0413/tetr-ch-rs/issues/4).
    #[serde(
        rename = "ts",
        deserialize_with = "deserialize_from_non_str_to_none",
        default
    )]
    pub received_at: Option<Timestamp>,
}

impl Badge {
    impl_for_id_badge_id!();
    impl_for_received_at!();
}

impl AsRef<Badge> for Badge {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user's third party connections.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Connections {
    /// This user's connection to Discord.
    ///
    /// - `id`: Discord ID.
    /// - `username`: Discord username.
    /// - `display_username`: Same as `username`.
    pub discord: Option<Connection>,
    /// This user's connection to Twitch.
    ///
    /// - `id`: Twitch user ID.
    /// - `username`: Twitch username (as used in the URL).
    /// - `display_username`: Twitch display name (may include Unicode).
    pub twitch: Option<Connection>,
    /// This user's connection to X
    /// (kept in the API as twitter for readability).
    ///
    /// - `id`: X user ID.
    /// - `username`: X handle (as used in the URL).
    /// - `display_username`: X display name (may include Unicode).
    pub twitter: Option<Connection>,
    /// This user's connection to Reddit.
    ///
    /// - `id`: Reddit user ID.
    /// - `username`: Reddit username.
    /// - `display_username`: Same as `username`.
    pub reddit: Option<Connection>,
    /// This user's connection to YouTube.
    ///
    /// - `id`: YouTube user ID (as used in the URL).
    /// - `username`: YouTube display name.
    /// - `display_username`: Same as `username`.
    pub youtube: Option<Connection>,
    /// This user's connection to Steam.
    ///
    /// - `id`: SteamID.
    /// - `username`: Steam display name.
    /// - `display_username`: Same as `username`.
    pub steam: Option<Connection>,
}

impl AsRef<Connections> for Connections {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user's connection.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Connection {
    /// This user's user ID on the service.
    pub id: String,
    /// This user's username on the service.
    pub username: String,
    /// This user's display username on the service.
    pub display_username: String,
}

impl AsRef<Connection> for Connection {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user's distinguishment banner.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Distinguishment {
    /// The type of distinguishment banner.
    #[serde(rename = "type")]
    pub _type: String,
    /// The detail of distinguishment banner.
    ///
    /// ***The API document does not say about this field.**
    pub detail: Option<String>,
    /// The header of distinguishment banner.
    ///
    /// ***The API document does not say about this field.**
    pub header: Option<String>,
    /// The footer of distinguishment banner.
    ///
    /// ***The API document does not say about this field.**
    pub footer: Option<String>,
}

impl AsRef<Distinguishment> for Distinguishment {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A breakdown of the source of a user's Achievement Rating.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct AchievementRatingCounts {
    /// The amount of ranked Bronze achievements this user has.
    #[serde(rename = "1")]
    pub bronze: Option<u32>,
    /// The amount of ranked Silver achievements this user has.
    #[serde(rename = "2")]
    pub silver: Option<u32>,
    /// The amount of ranked Gold achievements this user has.
    #[serde(rename = "3")]
    pub gold: Option<u32>,
    /// The amount of ranked Platinum achievements this user has.
    #[serde(rename = "4")]
    pub platinum: Option<u32>,
    /// The amount of ranked Diamond achievements this user has.
    #[serde(rename = "5")]
    pub diamond: Option<u32>,
    /// The amount of ranked Issued achievements this user has.
    #[serde(rename = "100")]
    pub issued: Option<u32>,
    /// The amount of competitive achievements this user has ranked into the top 100 with.
    #[serde(rename = "t100")]
    pub top100: Option<u32>,
    /// The amount of competitive achievements this user has ranked into the top 50 with.
    #[serde(rename = "t50")]
    pub top50: Option<u32>,
    ///  The amount of competitive achievements this user has ranked into the top 25 with.
    #[serde(rename = "t25")]
    pub top25: Option<u32>,
    /// The amount of competitive achievements this user has ranked into the top 10 with.
    #[serde(rename = "t10")]
    pub top10: Option<u32>,
    /// The amount of competitive achievements this user has ranked into the top 5 with.
    #[serde(rename = "t5")]
    pub top5: Option<u32>,
    /// The amount of competitive achievements this user has ranked into the top 3 with.
    #[serde(rename = "t3")]
    pub top3: Option<u32>,
}

impl AsRef<AchievementRatingCounts> for AchievementRatingCounts {
    fn as_ref(&self) -> &Self {
        self
    }
}
