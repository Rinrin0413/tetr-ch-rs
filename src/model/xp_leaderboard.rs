//! The XP leaderboard data.

use crate::{
    model::{
        cache::CacheData,
        user::{Role, UserId},
    },
    util::{max_f64, to_unix_ts},
};
use serde::Deserialize;

/// The response for the XP leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct XPLeaderboardResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// Query parameters used to request.
    ///
    /// # Notes
    ///
    /// This field will never be returned to you in the `None` state.
    /// So you can call the `unwrap`method on this value, for example.
    #[serde(default = "none")]
    pub query: Option<QueryCache>,
    /// The requested XP leaderboard data.
    pub data: Option<XPLeaderboardData>,
}

impl XPLeaderboardResponse {
    /// Whether all query parameters are default.
    pub fn is_default_query(&self) -> bool {
        if let Some(qp) = self.query.as_ref() {
            qp.before.is_none() && qp.after.is_none() && qp.limit.is_none() && qp.country.is_none()
        } else {
            true
        }
    }

    /// Whether the leaderboard is reversed.
    pub fn is_reversed(&self) -> bool {
        self.query.as_ref().unwrap().before.is_some()
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
}

impl AsRef<XPLeaderboardResponse> for XPLeaderboardResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// Returns a `None`.
fn none() -> Option<QueryCache> {
    None
}

/// A cache of query parameters used to the request.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct QueryCache {
    /// The lower bound in XP.
    /// Use this to paginate upwards.
    /// Take the highest seen XP and pass that back through this field to continue scrolling.
    /// If set, the search order is reversed (returning the lowest items that match the query)
    pub before: Option<String>,
    /// The upper bound in XP.
    /// Use this to paginate downwards.
    /// Take the lowest seen XP and pass that back through this field to continue scrolling.
    /// Infinite([`f64::INFINITY`]) by default.
    pub after: Option<String>,
    /// The amount of entries to return.
    /// Between 1 and 100.
    /// 50 by default.
    pub limit: Option<String>,
    /// The ISO 3166-1 country code to filter to.
    pub country: Option<String>,
}

impl QueryCache {
    /// Whether all query parameters are default.
    pub fn is_default_query(&self) -> bool {
        self.before.is_none()
            && self.after.is_none()
            && self.limit.is_none()
            && self.country.is_none()
    }
}

/// A requested XP leaderboard data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct XPLeaderboardData {
    /// An array of the matched users.
    pub users: Vec<User>,
}

impl AsRef<XPLeaderboardData> for XPLeaderboardData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The matched user's data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    #[serde(rename = "username")]
    pub name: String,
    /// The user's role.
    pub role: Role,
    /// When the user account was created.
    /// If not set, this account was created before join dates were recorded.
    #[serde(rename = "ts")]
    pub created_at: Option<String>,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown. Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user is currently supporting TETR.IO <3
    #[serde(rename = "supporter")]
    pub is_supporter: Option<bool>, // EXCEPTION
    /// Whether this user is a verified account.
    #[serde(rename = "verified")]
    pub is_verified: bool,
    /// The user's XP in points.
    pub xp: f64,
    /// The amount of online games played by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gamesplayed")]
    pub play_count: i32,
    /// The amount of online games won by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gameswon")]
    pub win_count: i32,
    /// The amount of seconds this user spent playing, both on- and offline.
    /// If the user has chosen to hide this statistic, it will be -1.
    #[serde(rename = "gametime")]
    pub play_time: f64,
}

impl User {
    /// Returns the level based on the user's xp.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
            as u32
    }

    /// Returns UNIX timestamp when the user's account created, if one exists.
    pub fn account_created_at(&self) -> Option<i64> {
        self.created_at.as_ref().map(|ts| to_unix_ts(ts))
    }

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

    /// Whether the user is a supporter.
    pub fn is_supporter(&self) -> bool {
        self.is_supporter.unwrap_or(false)
    }

    /// Whether the user is verified.
    pub fn is_verified(&self) -> bool {
        self.is_verified
    }

    /// Whether the user is an anonymous.
    #[deprecated(since = "0.3.5", note = "typo in function name. use `is_anon` instead")]
    pub fn is_anonymous(&self) -> bool {
        self.role.is_anon()
    }

    /// Whether the user is an administrator.
    #[deprecated(
        since = "0.3.5",
        note = "typo in function name. use `is_admin` instead"
    )]
    pub fn is_administrator(&self) -> bool {
        self.role.is_admin()
    }

    /// Whether the user is a moderator.
    #[deprecated(since = "0.3.5", note = "typo in function name. use `is_mod` instead")]
    pub fn is_moderator(&self) -> bool {
        self.role.is_mod()
    }
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &Self {
        self
    }
}
