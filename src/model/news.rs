//! The All Latest News models.

use crate::model::{cache::CacheData, league::Rank};
use serde::Deserialize;

/// The response for the All Latest News data.
///
/// The latest news items in any stream.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct NewsAllResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<NewsItems>,
}

impl AsRef<NewsAllResponse> for NewsAllResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The All Latest News data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct NewsItems {
    /// The latest news items.
    pub news: Vec<News>,
}

impl AsRef<NewsItems> for NewsItems {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A news.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct News {
    /// The item's internal ID.
    #[serde(rename = "_id")]
    pub id: String,
    /// The item's stream.
    pub stream: String,
    /// The item's type.
    pub r#type: String,
    /// The item's records.
    pub data: NewsData,
    /// The item's creation date.
    #[serde(rename = "ts")]
    pub created_at: String,
}

impl AsRef<News> for News {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a news item.
///
/// News data may be stored in different enumerators depending on the type of news item.
///
/// ***New news types may be added at any moment.**
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum NewsData {
    /// When a user's new personal best enters a global leaderboard.
    /// Seen in the global stream only.
    LeaderboardNews(LeaderboardNews),
    /// When a user gets a personal best. Seen in user streams only.
    PersonalBestNews(PersonalBestNews),
    /// When a user gets a badge.
    /// Seen in user streams only.
    BadgeNews(BadgeNews),
    /// When a user gets a new top rank in TETRA LEAGUE.
    /// Seen in user streams only.
    RankUpNews(RankUpNews),
    /// When a user gets TETR.IO Supporter. Seen in user streams only.
    SupporterNews(SupporterNews),
    /// When a user is gifted TETR.IO Supporter. Seen in user streams only.
    SupporterGiftNews(SupporterGiftNews),
    /// An unknown news type.
    Unknown(serde_json::Value),
}

impl AsRef<NewsData> for NewsData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a leaderboard news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardNews {
    /// The username of the person who got the leaderboard spot.
    pub username: String,
    /// The game mode played.
    pub gametype: String,
    /// The global rank achieved.
    pub rank: u32,
    /// The result (score or time) achieved.
    pub result: f64,
    /// The replay's shortID.
    #[serde(rename = "replayid")]
    pub replay_id: String,
}

impl AsRef<LeaderboardNews> for LeaderboardNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a personal best news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PersonalBestNews {
    /// The username of the player.
    pub username: String,
    /// The game mode played.
    pub gametype: String,
    /// The result (score or time) achieved.
    pub result: f64,
    /// The replay's shortID.
    #[serde(rename = "replayid")]
    pub replay_id: String,
}

impl AsRef<PersonalBestNews> for PersonalBestNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a badge news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct BadgeNews {
    /// The username of the player.
    pub username: String,
    /// The badge's internal ID, and the filename of the badge icon
    /// (all PNGs within `/res/badges/`)
    pub r#type: String,
    /// The badge's label.
    pub label: String,
}

impl AsRef<BadgeNews> for BadgeNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a rank up news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RankUpNews {
    /// The username of the player.
    pub username: String,
    /// The new rank.
    pub rank: Rank,
}

impl AsRef<RankUpNews> for RankUpNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a supporter news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SupporterNews {
    /// The username of the player.
    pub username: String,
}

impl AsRef<SupporterNews> for SupporterNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The data of a supporter gift news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SupporterGiftNews {
    /// The username of the recipient.
    pub username: String,
}
