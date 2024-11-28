//! Models for the TETRA NEWS endpoints.
//!
//! - About the endpoint "All Latest News",
//! see the [API document](https://tetr.io/about/api/#newsall).
//! - About the endpoint "Latest News",
//! see the [API document](https://tetr.io/about/api/#newsstream).

use crate::{
    client::{error::RspErr, Client},
    model::{
        cache::CacheData,
        error_response::ErrorResponse,
        user::UserResponse,
        util::{
            badge_id::BadgeId, gamemode::Gamemode, league_rank::Rank, news_stream::NewsStream,
            replay_id::ReplayId, timestamp::Timestamp,
        },
    },
};
use serde::Deserialize;

/// A struct for the response for the endpoint "All Latest News".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct NewsAllResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
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

/// Latest news items.
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
    pub stream: NewsStream,
    /// The item's type.
    pub r#type: String,
    /// The item's records.
    pub data: NewsData,
    /// The item's creation date.
    #[serde(rename = "ts")]
    pub created_at: Timestamp,
}

impl News {
    /// Returns a UNIX timestamp when the news item was created.
    ///
    /// # Panics
    ///
    /// Panics if failed to parse the timestamp.
    pub fn created_at(&self) -> i64 {
        self.created_at.unix_ts()
    }
}

impl AsRef<News> for News {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A news data.
///
/// News data may be stored in different enumerators depending on the type of news item.
///
/// ***New news types may be added at any moment.**  
/// For more details, see the [API document](https://tetr.io/about/api/#newsdata).
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

impl NewsData {
    /// Whether the news data is a leaderboard news.
    pub fn is_leaderboard_news(&self) -> bool {
        matches!(self, Self::LeaderboardNews(_))
    }

    /// Whether the news data is a personal best news.
    pub fn is_personal_best_news(&self) -> bool {
        matches!(self, Self::PersonalBestNews(_))
    }

    /// Whether the news data is a badge news.
    pub fn is_badge_news(&self) -> bool {
        matches!(self, Self::BadgeNews(_))
    }

    /// Whether the news data is a rank up news.
    pub fn is_rank_up_news(&self) -> bool {
        matches!(self, Self::RankUpNews(_))
    }

    /// Whether the news data is a supporter news.
    pub fn is_supporter_news(&self) -> bool {
        matches!(self, Self::SupporterNews(_))
    }

    /// Whether the news data is a supporter gift news.
    pub fn is_supporter_gift_news(&self) -> bool {
        matches!(self, Self::SupporterGiftNews(_))
    }

    /// Whether the news data is an unknown news type.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

impl AsRef<NewsData> for NewsData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A data of a leaderboard news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeaderboardNews {
    /// The username of the person who got the leaderboard spot.
    pub username: String,
    /// The game mode played.
    pub gametype: Gamemode,
    /// The global rank achieved.
    pub rank: u32,
    /// The result (score or time) achieved.
    pub result: f64,
    /// The replay's shortID.
    #[serde(rename = "replayid")]
    pub replay_id: ReplayId,
}

impl LeaderboardNews {
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
        Client::new().get_user(&self.username).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Returns the replay URL.
    pub fn replay_url(&self) -> String {
        self.replay_id.replay_url()
    }
}

impl AsRef<LeaderboardNews> for LeaderboardNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A data of a personal best news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct PersonalBestNews {
    /// The username of the player.
    pub username: String,
    /// The game mode played.
    pub gametype: Gamemode,
    /// The result (score or time) achieved.
    pub result: f64,
    /// The replay's shortID.
    #[serde(rename = "replayid")]
    pub replay_id: ReplayId,
}

impl PersonalBestNews {
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
        Client::new().get_user(&self.username).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Returns the replay URL.
    pub fn replay_url(&self) -> String {
        self.replay_id.replay_url()
    }
}

impl AsRef<PersonalBestNews> for PersonalBestNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A data of a badge news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct BadgeNews {
    /// The username of the player.
    pub username: String,
    /// The badge's internal ID, and the filename of the badge icon
    /// (all PNGs within `/res/badges/`)
    #[serde(rename = "type")]
    pub id: BadgeId,
    /// The badge's label.
    pub label: String,
}

impl BadgeNews {
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
        Client::new().get_user(&self.username).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }

    /// Returns the badge icon URL.
    pub fn badge_icon_url(&self) -> String {
        self.id.icon_url()
    }
}

impl AsRef<BadgeNews> for BadgeNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A data of a rank up news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RankUpNews {
    /// The username of the player.
    pub username: String,
    /// The new rank.
    pub rank: Rank,
}

impl RankUpNews {
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
        Client::new().get_user(&self.username).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }
}

impl AsRef<RankUpNews> for RankUpNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A data of a supporter news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SupporterNews {
    /// The username of the player.
    pub username: String,
}

impl SupporterNews {
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
        Client::new().get_user(&self.username).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }
}

impl AsRef<SupporterNews> for SupporterNews {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A data of a supporter gift news item.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SupporterGiftNews {
    /// The username of the recipient.
    pub username: String,
}

impl SupporterGiftNews {
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
        Client::new().get_user(&self.username).await
    }

    /// Returns the user's TETRA CHANNEL profile URL.
    pub fn profile_url(&self) -> String {
        format!("https://ch.tetr.io/u/{}", self.username)
    }
}

/// A struct for the response for the endpoint "Latest News".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct NewsLatestResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<NewsItems>,
}

impl AsRef<NewsLatestResponse> for NewsLatestResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}
