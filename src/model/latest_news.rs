//! Latest news model.

use crate::{model::cache::CacheData, util::to_unix_ts};
use serde::Deserialize;

/// The response for the latest news.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LatestNewsResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested latest news data.
    pub data: Option<LatestNews>,
}

impl LatestNewsResponse {
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

impl AsRef<LatestNewsResponse> for LatestNewsResponse {
    fn as_ref(&self) -> &LatestNewsResponse {
        self
    }
}

/// The requested latest news.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LatestNews {
    /// The latest news items.
    pub news: Vec<News>,
}

impl AsRef<LatestNews> for LatestNews {
    fn as_ref(&self) -> &LatestNews {
        self
    }
}

/// A news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct News {
    /// The item's internal ID.
    pub _id: String,
    /// The item's stream.
    pub stream: String,
    /// The item's type.
    ///
    /// Currently(August 2022) it has the following types:
    /// - `leaderboard`:
    /// When a user's new personal best enters a global leaderboard.
    /// Seen in the global stream only.
    /// - `personalbest`
    /// When a user gets a personal best.
    /// Seen in user streams only.
    /// - `badge`
    /// When a user gets a badge.
    /// Seen in user streams only.
    /// - `rankup`
    /// When a user gets a new top rank in TETRA LEAGUE.
    /// Seen in user streams only.
    /// - `supporter`
    /// When a user gets TETR.IO Supporter.
    /// Seen in user streams only.
    /// - `supporter_gift`
    /// When a user is gifted TETR.IO Supporter.
    /// Seen in user streams only.
    #[serde(rename = "type")]
    pub _type: String,
    /// The item's records.
    pub data: NewsData,
    /// The item's creation date.
    pub ts: String,
}

impl News {
    /// ~~Returns an icon URL of the TETRA LEAGUE rank.~~
    /// ~~If the user is unranked, returns ?-rank(z) icon URL.~~
    ///
    /// ~~# Panics~~
    ///
    /// ~~Panics if the stream is not type([`Self::_type`]) `rankup`.~~
    /// 
    /// This function does not currently work.
    /// See [here](`NewsData::rank`) for the reason.
    #[deprecated]
    pub fn rank_icon_url(&self) /*-> String*/ {
        /*if let Some(rank) = self.data.rank.as_ref() {
            if let Ok(_) = rank.parse::<u32>() {
                panic!("This stream is not type `rankup`")
            } else {
                format!("https://tetr.io/res/league-ranks/{}.png", rank)
            }
        } else {
            panic!("This stream is not type `rankup`")
        }*/
    }

    /// Returns an badge URL.
    ///
    /// # Panics
    ///
    /// Panics if the stream is not type([`Self::_type`]) `badge`.
    pub fn badge_icon_url(&self) -> String {
        if let Some(i) = self.data._type.as_ref() {
            format!("https://tetr.io/res/badges/{}.png", i)
        } else {
            panic!("This stream is not type `badge`")
        }
    }

    /// Returns a UNIX timestamp when the item was created.
    pub fn creation_at(&self) -> i64 {
        to_unix_ts(&self.ts)
    }
}

impl AsRef<News> for News {
    fn as_ref(&self) -> &News {
        self
    }
}

/// The item's records.
///
/// Which fields are valid depends on the [`News::_type`].
/// Currently(August 2022) it has the following types:
/// - `leaderboard`
/// - `personalbest`
/// - `badge`
/// - `rankup`
/// - `supporter`
/// - `supporter_gift`
///
/// And defined as optional even if the field is currently(August 2022) valid for all types.
/// This is for backward compatibility.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct NewsData {
    /// The username of the player.
    ///
    /// Valid for types:
    /// `leaderboard`, `personalbest`, `badge`, `rankup`, `supporter`, `supporter_gift`
    pub username: Option<String>,
    /// The game mode played.
    ///
    /// Valid for types:
    /// `leaderboard`, `personalbest`
    pub gametype: Option<String>,
    /// - ~~The global rank achieved. (leaderboard)~~
    /// - ~~The new rank. (rankup)~~
    ///
    /// ~~Valid for types:~~
    /// ~~`leaderboard`,~~
    /// ~~`rankup`~~
    /// 
    /// This field is currently(August 2022) too dynamic.
    /// So the developer(Rinrin.rs) was not able to deal it.
    #[serde(default = "none")]
    pub _rank: Option<()>,
    /// The result (score or time) achieved.
    ///
    /// Valid for types:
    /// `leaderboard`, `personalbest`
    pub result: Option<f64>,
    /// The replay's shortID.
    ///
    /// Valid for types:
    /// `leaderboard`, `personalbest`
    pub replayid: Option<String>,
    /// The badge's internal ID,
    /// and the filename of the badge icon (all PNGs within /res/badges/)
    ///
    /// Valid for types:
    /// `badge`
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// The badge's label.
    ///
    /// Valid for types:
    /// `badge`
    pub label: Option<String>,
}

impl AsRef<NewsData> for NewsData {
    fn as_ref(&self) -> &NewsData {
        self
    }
}

fn none() -> Option<()> {
    None
}
