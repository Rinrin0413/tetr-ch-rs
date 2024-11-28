//! Models for the endpoint "User Info", and its related types.
//!
//! About the endpoint "User Info",
//! see the [API document](https://tetr.io/about/api/#usersuser).

use crate::{
    client::{error::RspErr, Client},
    model::{
        cache::CacheData,
        error_response::ErrorResponse,
        role::Role,
        util::{badge_id::BadgeId, timestamp::Timestamp},
    },
    util::{deserialize_from_non_str_to_none, max_f64},
};
use serde::Deserialize;
use std::fmt;

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
    pub friend_count: Option<u32>, // EXCEPTION
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
    /// Returns the level of the user.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
            as u32
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Whether the user is a normal user.
    pub fn is_normal_user(&self) -> bool {
        self.role.is_normal_user()
    }

    /// Whether the user is an anonymous.
    pub fn is_anon(&self) -> bool {
        self.role.is_anon()
    }

    /// Whether the user is a bot.
    pub fn is_bot(&self) -> bool {
        self.role.is_bot()
    }

    /// Whether the user is a SYSOP.
    pub fn is_sysop(&self) -> bool {
        self.role.is_sysop()
    }

    /// Whether the user is an administrator.
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Whether the user is a moderator.
    pub fn is_mod(&self) -> bool {
        self.role.is_mod()
    }

    /// Whether the user is a community moderator.
    pub fn is_halfmod(&self) -> bool {
        self.role.is_halfmod()
    }

    /// Whether the user is banned.
    pub fn is_banned(&self) -> bool {
        self.role.is_banned()
    }

    /// Whether the user is hidden.
    pub fn is_hidden(&self) -> bool {
        self.role.is_hidden()
    }

    /// Returns a UNIX timestamp when the user's account created.
    ///
    /// If the account was created before join dates were recorded, `None` is returned.
    ///
    /// # Panics
    ///
    /// Panics if failed to parse the timestamp.
    pub fn created_at(&self) -> Option<i64> {
        self.created_at.as_ref().map(|ts| ts.unix_ts())
    }

    /// Whether the user has any badges.
    pub fn has_badge(&self) -> bool {
        !self.badges.is_empty()
    }

    /// Returns the number of badges the user has.
    pub fn badge_count(&self) -> usize {
        self.badges.len()
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
    pub desc: Option<String>, // EXCEPTION
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
    /// Returns the badge icon URL.
    pub fn badge_icon_url(&self) -> String {
        self.id.icon_url()
    }

    /// Returns a UNIX timestamp when the badge was achieved.
    ///
    /// # Panics
    ///
    /// Panics if failed to parse the timestamp.
    pub fn received_at(&self) -> Option<i64> {
        self.received_at.as_ref().map(|ts| ts.unix_ts())
    }
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
    pub detail: Option<String>, // EXCEPTION
    /// The header of distinguishment banner.
    pub header: Option<String>, // EXCEPTION
    /// The footer of distinguishment banner.
    pub footer: Option<String>, // EXCEPTION
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

/// A user's internal ID.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct UserId(pub String);

impl UserId {
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
        Client::new().get_user(&self.to_string()).await
    }

    /// Returns the user's internal ID.
    #[deprecated(since = "0.6.0", note = "please use the `.to_string()` method instead")]
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl AsRef<UserId> for UserId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
