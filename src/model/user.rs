//! The User Info models.

use crate::{
    client::{error::ResponseError, Client},
    model::cache::CacheData,
    util::{deserialize_from_non_str_to_none, max_f64, to_unix_ts},
};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

/// The response for User Info data.
/// An object describing the user in detail.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<User>,
}

impl UserResponse {
    /// Returns the UNIX timestamp when the user's account created, if one exists.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn account_created_at(&self) -> Option<i64> {
        self.get_user().created_at.as_ref().map(|ts| to_unix_ts(ts))
    }

    /// Returns the level based on the user's xp.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn level(&self) -> u32 {
        let xp = self.get_user().xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
            as u32
    }

    /// Returns the user's avatar URL.
    /// If the user has no avatar, returns anon's.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn face(&self) -> String {
        let default = "https://tetr.io/res/avatar.png".to_string();
        if let Some(ar) = self.get_user().avatar_revision {
            if ar == 0 {
                return default;
            }
            format!(
                "https://tetr.io/user-content/avatars/{}.jpg?rv={}",
                self.get_user().id,
                ar
            )
        } else {
            default
        }
    }

    /// Returns the user's banner URL.
    /// If the user has no banner, returns `None`.
    ///
    /// ***Even if the user is not currently a supporter,
    /// the URL may be returned if the banner was once set.**
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn banner(&self) -> Option<String> {
        if let Some(br) = self.get_user().banner_revision {
            if br == 0 {
                return None;
            }
            Some(format!(
                "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                self.get_user().id,
                br
            ))
        } else {
            None
        }
    }

    /// Whether the user has at least one badge.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn has_badges(&self) -> bool {
        !self.get_user().badges.is_empty()
    }

    /// Whether the user is an anonymous.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_anon(&self) -> bool {
        self.get_user().role.is_anon()
    }

    /// Whether the user is a bot.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_bot(&self) -> bool {
        self.get_user().role.is_bot()
    }

    /// Whether the user is a SYSOP.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_sysop(&self) -> bool {
        self.get_user().role.is_sysop()
    }

    /// Whether the user is an administrator.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_admin(&self) -> bool {
        self.get_user().role.is_admin()
    }

    /// Whether the user is a moderator,
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_mod(&self) -> bool {
        self.get_user().role.is_mod()
    }

    /// Whether the user is a community moderator.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_halfmod(&self) -> bool {
        self.get_user().role.is_halfmod()
    }

    /// Whether the user is banned.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_banned(&self) -> bool {
        self.get_user().role.is_banned()
    }

    /// Whether the user is hidden.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_hidden(&self) -> bool {
        self.get_user().role.is_hidden()
    }

    /// Whether the user is bad standing.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_badstanding(&self) -> bool {
        self.get_user().is_badstanding.unwrap_or(false)
    }

    /// Whether the user is a supporter.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn is_supporter(&self) -> bool {
        self.get_user().is_supporter.unwrap_or(false)
    }

    /// Returns the user's profile URL.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.get_user().username)
    }

    /// Returns an `Option<String>`.
    ///
    /// If user is displaying the country,
    /// returns `Some(String)` with an image URL of the national flag based on the user's ISO 3166-1 country code.
    /// If the user is not displaying the country, returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn national_flag_url(&self) -> Option<String> {
        self.get_user()
            .country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }

    /// Returns the badges count.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn badges_count(&self) -> usize {
        self.get_user().badges.len()
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

    /// Returns the [`&User`](crate::model::user::User).
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    fn get_user(&self) -> &User {
        if let Some(d) = self.data.as_ref() {
            d
        } else {
            panic!("There is no user object because the request was not successful.")
        }
    }
}

impl AsRef<UserResponse> for UserResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The User Info data.
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
    pub created_at: Option<String>,
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
    /// Whether this user currently has a bad standing (recently banned).
    #[serde(rename = "badstanding")]
    pub is_badstanding: Option<bool>,
    /// Whether this user is currently supporting TETR.IO <3
    #[serde(rename = "supporter")]
    pub is_supporter: Option<bool>, // EXCEPTION
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
    /// ***This field is optional but the API documentation does not mention it.**
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
    /// Returns UNIX timestamp when the user's account created, if one exists.
    pub fn account_created_at(&self) -> Option<i64> {
        self.created_at.as_ref().map(|ts| to_unix_ts(ts))
    }

    /// Returns the level based on the user's xp.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
            as u32
    }

    /// Returns the user's avatar URL.
    /// If the user has no avatar, returns anon's.
    pub fn face(&self) -> String {
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
    /// If the user has no banner, returns `None`.
    ///
    /// ***Even if the user is not currently a supporter,
    /// the URL may be returned if the banner was once set.**
    pub fn banner(&self) -> Option<String> {
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

    /// Whether the user has at least one badge.
    pub fn has_badge(&self) -> bool {
        !self.badges.is_empty()
    }

    /// Whether this user is an anonymous.
    pub fn is_anon(&self) -> bool {
        self.role.is_anon()
    }

    /// Whether this user is a bot.
    pub fn is_bot(&self) -> bool {
        self.role.is_bot()
    }

    /// Whether this user is a SYSOP.
    pub fn is_sysop(&self) -> bool {
        self.role.is_sysop()
    }

    /// Whether this user is an administrator.
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Whether this user is a moderator.
    pub fn is_mod(&self) -> bool {
        self.role.is_mod()
    }

    /// Whether this user is a community moderator.
    pub fn is_halfmod(&self) -> bool {
        self.role.is_halfmod()
    }

    /// Whether this user is banned.
    pub fn is_banned(&self) -> bool {
        self.role.is_banned()
    }

    /// Whether this user is hidden.
    pub fn is_hidden(&self) -> bool {
        self.role.is_hidden()
    }

    /// Whether this user is bad standing.
    pub fn is_badstanding(&self) -> bool {
        self.is_badstanding.unwrap_or(false)
    }

    /// Whether this user is a supporter.
    pub fn is_supporter(&self) -> bool {
        self.is_supporter.unwrap_or(false)
    }

    /// Returns the user's profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Returns an i

    /// Returns an `Option<String>`.
    ///
    /// If user is displaying the country,
    /// returns `Some(String)` with an image URL of the national flag based on the user's ISO 3166-1 country code.
    /// If the user is not displaying the country, returns `None`.
    pub fn national_flag_url(&self) -> Option<String> {
        self.country
            .as_ref()
            .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
    }

    /// Returns the badges count.
    pub fn badges_count(&self) -> usize {
        self.badges.len()
    }
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's role.
#[derive(Clone, Debug, Deserialize)]
pub enum Role {
    /// The normal user.
    #[serde(rename = "user")]
    User,
    /// The anonymous user.
    #[serde(rename = "anon")]
    Anon,
    /// The bot.
    #[serde(rename = "bot")]
    Bot,
    /// The SYSOP.
    #[serde(rename = "sysop")]
    Sysop,
    /// The administrator.
    #[serde(rename = "admin")]
    Admin,
    /// The moderator.
    #[serde(rename = "mod")]
    Mod,
    /// The community moderator.
    #[serde(rename = "halfmod")]
    Halfmod,
    /// The banned user.
    #[serde(rename = "banned")]
    Banned,
    /// The hidden user.
    #[serde(rename = "hidden")]
    Hidden,
}

impl Role {
    /// Whether the user is an anonymous.
    pub fn is_anon(&self) -> bool {
        matches!(self, Role::Anon)
    }

    /// Whether the user is a bot.
    pub fn is_bot(&self) -> bool {
        matches!(self, Role::Bot)
    }

    /// Whether the user is a SYSOP.
    pub fn is_sysop(&self) -> bool {
        matches!(self, Role::Sysop)
    }

    /// Whether the user is an administrator.
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }

    /// Whether the user is a moderator.
    pub fn is_mod(&self) -> bool {
        matches!(self, Role::Mod)
    }

    /// Whether the user is a community moderator.
    pub fn is_halfmod(&self) -> bool {
        matches!(self, Role::Halfmod)
    }

    /// Whether the user is banned.
    pub fn is_banned(&self) -> bool {
        matches!(self, Role::Banned)
    }

    /// Whether the user is hidden.
    pub fn is_hidden(&self) -> bool {
        matches!(self, Role::Hidden)
    }
}

impl AsRef<Role> for Role {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ToString for Role {
    /// Converts the given value to a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::user::Role;
    /// assert_eq!(Role::User.to_string(), "User");
    /// assert_eq!(Role::Anon.to_string(), "Anonymous");
    /// assert_eq!(Role::Bot.to_string(), "Bot");
    /// assert_eq!(Role::Sysop.to_string(), "SYSOP");
    /// assert_eq!(Role::Admin.to_string(), "Administrator");
    /// assert_eq!(Role::Mod.to_string(), "Moderator");
    /// assert_eq!(Role::Halfmod.to_string(), "Community moderator");
    /// assert_eq!(Role::Banned.to_string(), "Banned user");
    /// assert_eq!(Role::Hidden.to_string(), "Hidden user");
    /// ```
    fn to_string(&self) -> String {
        match self {
            Role::User => "User",
            Role::Anon => "Anonymous",
            Role::Bot => "Bot",
            Role::Sysop => "SYSOP",
            Role::Admin => "Administrator",
            Role::Mod => "Moderator",
            Role::Halfmod => "Community moderator",
            Role::Banned => "Banned user",
            Role::Hidden => "Hidden user",
        }
        .to_string()
    }
}

/// The user's badges.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Badge {
    /// The badge's internal ID,
    /// and the filename of the badge icon
    /// (all PNGs within `/res/badges/`).
    /// Note that badge IDs may include forward slashes.
    /// Please do not encode them!
    /// Follow the folder structure.
    pub id: String,
    /// The badge's group ID.
    /// If multiple badges have the same group ID, they are rendered together.
    pub group: Option<String>,
    /// The badge's label, shown when hovered.
    pub label: String,
    /// Extra flavor text for the badge, shown when hovered.
    ///
    /// ***This field is optional but the API documentation does not mention it.**
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
    pub received_at: Option<String>,
}

impl Badge {
    /// Returns the formatted badge icon URL.
    pub fn badge_icon_url(&self) -> String {
        format!("https://tetr.io/res/badges/{}.png", self.id)
    }

    /// Returns a UNIX timestamp when this badge was achieved.
    pub fn received_at(&self) -> Option<i64> {
        self.received_at.as_ref().map(|ts| to_unix_ts(ts))
    }
}

impl AsRef<Badge> for Badge {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// This user's third party connections.
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

/// This user's connection.
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

/// This user's distinguishment banner.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Distinguishment {
    // The type of distinguishment banner.
    #[serde(rename = "type")]
    pub _type: String,
    /// The detail of distinguishment banner.
    ///
    /// ***This field is not documented in the API documentation.**
    pub detail: Option<String>,
    /// The header of distinguishment banner.
    ///
    /// ***This field is not documented in the API documentation.**
    pub header: Option<String>,
    /// the footer of distinguishment banner.
    ///
    /// ***This field is not documented in the API documentation.**
    pub footer: Option<String>,
}

impl AsRef<Distinguishment> for Distinguishment {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The breakdown of the source of this user's Achievement Rating.
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

/// The user's internal ID.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct UserId(pub String);

type RspErr<T> = Result<T, ResponseError>;

impl UserId {
    /// Returns the user's internal ID.
    pub fn id(&self) -> &str {
        &self.0
    }

    /// Gets the User Info data.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user(&self) -> RspErr<UserResponse> {
        Client::new().get_user(self.id()).await
    }
}

impl AsRef<UserId> for UserId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
