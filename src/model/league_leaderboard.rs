//! The TETRA LEAGUE leaderboard data.

use crate::{model::cache::CacheData, util::max_f64};
use serde::Deserialize;

/// The response for the TETRA LEAGUE leaderboard.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueLeaderboardResponse {
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
    /// The requested TETRA LEAGUE leaderboard data.
    pub data: Option<LeagueLeaderboardData>,
}

impl LeagueLeaderboardResponse {
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

    /// Whether the gotten leaderboard is a full export.
    pub fn is_full(&self) -> bool {
        if let Some(l) = self.query.as_ref().unwrap().limit.as_ref() {
            l == "Full"
        } else {
            false
        }
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

impl AsRef<LeagueLeaderboardResponse> for LeagueLeaderboardResponse {
    fn as_ref(&self) -> &LeagueLeaderboardResponse {
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
    /// The lower bound in TR.
    /// Use this to paginate upwards.
    /// Take the highest seen TR and pass that back through this field to continue scrolling.
    /// If set, the search order is reversed (returning the lowest items that match the query)
    pub before: Option<String>,
    /// The upper bound in TR.
    /// Use this to paginate downwards.
    /// Take the lowest seen TR and pass that back through this field to continue scrolling.
    /// 25000 by default.
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

    /// Whether the gotten leaderboard is a full export.
    pub fn is_full(&self) -> bool {
        if let Some(l) = self.limit.as_ref() {
            l == "Full"
        } else {
            false
        }
    }
}

impl AsRef<QueryCache> for QueryCache {
    fn as_ref(&self) -> &QueryCache {
        self
    }
}

/// The requested TETRA LEAGUE leaderboard data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueLeaderboardData {
    /// An array of the matched users.
    pub users: Vec<User>,
}

impl AsRef<LeagueLeaderboardData> for LeagueLeaderboardData {
    fn as_ref(&self) -> &LeagueLeaderboardData {
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
    /// The user's XP in points.
    pub xp: f64,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown. Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>, // EXCEPTION
    /// Whether this user is a verified account.
    pub verified: bool,
    /// This user's current TETRA LEAGUE standing.
    pub league: LeagueData,
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

    /// Returns an icon URL of the user's rank.
    /// If the user is unranked, returns ?-rank(z) icon URL.
    /// If the user has no rank, returns `None`.
    pub fn rank_icon_url(&self) -> Option<String> {
        if self.league.gamesplayed < 10 {
            Some(format!(
                "https://tetr.io/res/league-ranks/{}.png",
                self.league.rank
            ))
        } else {
            None
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

/// The user's current TETRA LEAGUE standing.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueData {
    /// The amount of TETRA LEAGUE games played by this user.
    pub gamesplayed: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    pub gameswon: u32,
    /// This user's TR (Tetra Rating), or -1 if less than 10 games were played.
    pub rating: f64,
    /// This user's letter rank. Z is unranked.
    pub rank: String,
    /// This user's Glicko-2 rating.
    pub glicko: Option<f64>,
    /// This user's Glicko-2 Rating Deviation.
    /// If over 100, this user is unranked.
    pub rd: Option<f64>,
    /// This user's average APM (attack per minute) over the last 10 games.
    pub apm: Option<f64>,
    /// This user's average PPS (pieces per second) over the last 10 games.
    pub pps: Option<f64>,
    /// This user's average VS (versus score) over the last 10 games.
    pub vs: Option<f64>,
    /// Whether this user's RD is rising (has not played in the last week).
    pub decaying: bool,
}

impl LeagueData {
    /// Returns an icon URL of the user's rank.
    /// If the user is unranked, returns ?-rank(z) icon URL.
    /// If the user has no rank, returns `None`.
    pub fn rank_icon_url(&self) -> Option<String> {
        if self.gamesplayed < 10 {
            Some(format!(
                "https://tetr.io/res/league-ranks/{}.png",
                self.rank
            ))
        } else {
            None
        }
    }
}

impl AsRef<LeagueData> for LeagueData {
    fn as_ref(&self) -> &LeagueData {
        self
    }
}
