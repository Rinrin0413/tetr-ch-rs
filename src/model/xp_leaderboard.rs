//! The XP leaderboard data.

use crate::{
    model::cache::CacheData,
    util::{max_f64, to_unix_ts},
};
use serde::Deserialize;

/// The response for the XP leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct XPLeaderboardResponse {
    /// Whether the request was successful.
    pub success: bool,
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
    fn as_ref(&self) -> &XPLeaderboardResponse {
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
    fn as_ref(&self) -> &XPLeaderboardData {
        self
    }
}

/// The matched user's data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// The user's internal ID.
    pub _id: String,
    /// The user's username.
    pub username: String,
    /// The user's role (one of `"anon"`, `"user"`, `"bot"`, `"mod"`, `"admin"`, *`"banned"`).  
    ///
    /// ***`"banned"` is not specified in TETRA CHANNEL API docs.**
    pub role: String,
    /// When the user account was created.
    /// If not set, this account was created before join dates were recorded.
    pub ts: Option<String>,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown. Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>, // EXCEPTION
    /// Whether this user is a verified account.
    pub verified: bool,
    /// The user's XP in points.
    pub xp: f64,
    /// The amount of online games played by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    pub gamesplayed: i32,
    /// The amount of online games won by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    pub gameswon: i32,
    /// The amount of seconds this user spent playing, both on- and offline.
    /// If the user has chosen to hide this statistic, it will be -1.
    pub gametime: f64,
}

impl User {
    /// Returns the level based on the user's xp.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        let level =
            ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.)
                .floor() as u32;
        level
    }

    /// Returns UNIX timestamp when the user's account created, if one exists.
    pub fn account_created_at(&self) -> Option<i64> {
        match &self.ts {
            Some(ts) => Some(to_unix_ts(ts)),
            None => None,
        }
    }

    /// Returns an `Option<String>`.
    ///
    /// If user is displaying the country,
    /// returns `Some(String)` with an image URL of the national flag based on the user's ISO 3166-1 country code.
    /// If the user is not displaying the country, returns `None`.
    pub fn national_flag_url(&self) -> Option<String> {
        if let Some(cc) = self.country.as_ref() {
            Some(format!(
                "https://tetr.io/res/flags/{}.png",
                cc.to_lowercase()
            ))
        } else {
            None
        }
    }
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}
