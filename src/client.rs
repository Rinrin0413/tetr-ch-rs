//! Client for API requests.

use crate::{
    error::{ResponseError, Status},
    model::{
        latest_news::LatestNewsResponse,
        league_leaderboard::{self, LeagueLeaderboardResponse},
        searched_user::SearchedUserResponse,
        server_activity::ServerActivityResponse,
        server_stats::ServerStatsResponse,
        stream::StreamResponse,
        summary::{blitz::BlitzResponse, forty_lines::FortyLinesResponse},
        user::{UserRecordsResponse, UserResponse},
        xp_leaderboard::{self, XPLeaderboardResponse},
    },
};
use http::status::StatusCode;
use reqwest::{self, Error, Response};
use serde::Deserialize;

const API_URL: &str = "https://ch.tetr.io/api/";

/// Client for API requests.
///
/// # Examples
///
/// Creating a Client instance and getting some objects:
///
/// ```no_run
/// use tetr_ch::client::Client;
/// # use std::io;
///
/// # async fn run() -> io::Result<()> {
/// let client = Client::new();
/// // For example, get information for user `RINRIN-RS`.
/// let user = client.get_user("rinrin-rs").await?;
/// # Ok(())
/// # }
/// ```
///
/// [See more examples](https://github.com/Rinrin0413/tetr-ch-rs/examples/)
#[non_exhaustive]
#[derive(Default)]
pub struct Client {
    client: reqwest::Client,
}

type RspErr<T> = Result<T, ResponseError>;

impl Client {
    /// Create a new [`Client`].
    ///
    /// # Examples
    ///
    /// Creating a Client instance:
    ///
    /// ```
    /// use tetr_ch::client;
    ///
    /// let client = client::Client::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Returns the object describing the user in detail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    /// // Get the User Info.
    /// let user = client.get_user("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user(self, user: &str) -> RspErr<UserResponse> {
        let url = format!("{}users/{}", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns some statistics about the TETR.IO.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    /// // Get the Server Statistics.
    /// let user = client.get_server_stats().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_server_stats(self) -> RspErr<ServerStatsResponse> {
        let url = format!("{}general/stats", API_URL);
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns an array of user activity over the last 2 days.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    /// // Get the Server Activity.
    /// let user = client.get_server_activity().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_server_activity(self) -> RspErr<ServerActivityResponse> {
        let url = format!("{}general/activity", API_URL);
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the user records model.
    ///
    /// # Examples
    ///
    /// Getting the records object:
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    /// // Get the user records.
    /// let user = client.get_user_records("621db46d1d638ea850be2aa0").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user_records(self, user: &str) -> RspErr<UserRecordsResponse> {
        let url = format!("{}users/{}/records", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the object describing a summary of the user's 40 LINES games.
    ///
    /// # Arguments
    ///
    /// - `user`: The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    /// // Get the User Summary 40 LINES.
    /// let user = client.get_user_40l("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user_40l(self, user: &str) -> RspErr<FortyLinesResponse> {
        let url = format!("{}users/{}/summaries/40l", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the object describing a summary of the user's BLITZ games.
    ///
    /// # Arguments
    ///
    /// - `user`: The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    /// // Get the User Summary BLITZ.
    /// let user = client.get_user_blitz("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_user_blitz(self, user: &str) -> RspErr<BlitzResponse> {
        let url = format!("{}users/{}/summaries/blitz", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the TETRA LEAGUE leaderboard model.
    ///
    /// # Arguments
    ///
    /// - `query`:
    ///
    /// The query parameters.
    /// This argument requires a [`query::LeagueLeaderboardQuery`].
    ///
    /// # Examples
    ///
    /// Getting the TETRA LEAGUE leaderboard object:
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, query::LeagueLeaderboardQuery};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Set the query parameters.
    /// let query = LeagueLeaderboardQuery::new()
    ///     // 15200TR or less.
    ///     .after(15200.)
    ///     // 3 users.
    ///     .limit(3)
    ///     // Japan.
    ///     .country("jp");
    ///
    /// // Get the TETRA LEAGUE leaderboard.
    /// let user = client.get_league_leaderboard(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [here](query::LeagueLeaderboardQuery) for details on setting query parameters.
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    ///
    /// # Panics
    ///
    /// Panics if the query parameter`limit` is not between 0 and 100.
    ///
    /// ```should_panic,no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     query::{LeagueLeaderboardQuery, Limit}
    /// };
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// let query = LeagueLeaderboardQuery {
    ///     // 101 users (not allowed).
    ///     limit: Some(Limit::Limit(101)),
    ///     ..LeagueLeaderboardQuery::new()
    /// };
    ///
    /// let user = client.get_league_leaderboard(query).await?;
    /// # Ok(())
    /// # }
    ///
    /// # tokio_test::block_on(run());
    /// ```
    pub async fn get_league_leaderboard(
        self,
        query: query::LeagueLeaderboardQuery,
    ) -> RspErr<LeagueLeaderboardResponse> {
        if query.is_invalid_limit_range() {
            panic!(
                "The query parameter`limit` must be between 0 and 100.\n\
                Received: {}",
                query.limit.unwrap().to_string()
            );
        }
        // Cloned the `query` here because the query parameters will be referenced later.
        let (q, url) = if query.will_full_export() {
            (
                query.clone().build_as_full_export(),
                format!("{}/users/lists/league/all", API_URL),
            )
        } else {
            (
                query.clone().build(),
                format!("{}/users/lists/league", API_URL),
            )
        };
        let r = self.client.get(url);
        let res = match q.len() {
            1 => r.query(&[&q[0]]),
            2 => r.query(&[&q[0], &q[1]]),
            3 => r.query(&[&q[0], &q[1], &q[2]]),
            _ => r,
        }
        .send()
        .await;
        match response::<LeagueLeaderboardResponse>(res).await {
            Ok(mut m) => {
                let (before, after) = if let Some(b_a) = query.before_or_after {
                    match b_a {
                        query::BeforeAfter::Before(b) => (Some(b.to_string()), None),
                        query::BeforeAfter::After(b) => (None, Some(b.to_string())),
                    }
                } else {
                    (None, None)
                };
                let limit = query.limit.map(|l| l.to_string());
                let country = query.country;
                m.query = Some(league_leaderboard::QueryCache {
                    before,
                    after,
                    limit,
                    country,
                });
                Ok(m)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the XP leaderboard model.
    ///
    /// # Arguments
    ///
    /// - `query`:
    ///
    /// The query parameters.
    /// This argument requires a [`query::XPLeaderboardQuery`].
    ///
    /// # Examples
    ///
    /// Getting the XP leaderboard object:
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, query::XPLeaderboardQuery};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Set the query parameters.
    /// let query = XPLeaderboardQuery::new()
    ///     // 50,000,000,000,000xp or less.
    ///     .after(50_000_000_000_000.)
    ///     // 10 users.
    ///     .limit(10)
    ///     // Serbia.
    ///     .country("rs");
    ///
    /// // Get the XP leaderboard.
    /// let user = client.get_xp_leaderboard(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See [here](query::XPLeaderboardQuery) for details on setting query parameters.
    ///
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    ///
    /// # Panics
    ///
    /// Panics if the query parameter`limit` is not between 1 and 100.
    ///
    /// ```should_panic,no_run
    /// use tetr_ch::client::{Client, query::XPLeaderboardQuery};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// let query = XPLeaderboardQuery {
    ///     // 101 users(not allowed).
    ///     limit: Some(std::num::NonZeroU8::new(101).unwrap()),
    ///     ..XPLeaderboardQuery::new()
    /// };
    ///
    /// let user = client.get_xp_leaderboard(query).await?;
    /// # Ok(())
    /// # }
    ///
    /// # tokio_test::block_on(run());
    /// ```
    pub async fn get_xp_leaderboard(
        self,
        query: query::XPLeaderboardQuery,
    ) -> RspErr<XPLeaderboardResponse> {
        if query.is_invalid_limit_range() {
            panic!(
                "The query parameter`limit` must be between 1 and 100.\n\
                Received: {}",
                query.limit.unwrap()
            );
        }
        // Cloned the `query` here because the query parameters will be referenced later.
        let q = query.clone().build();
        let url = format!("{}users/lists/xp", API_URL);
        let r = self.client.get(url);
        let res = match q.len() {
            1 => r.query(&[&q[0]]),
            2 => r.query(&[&q[0], &q[1]]),
            3 => r.query(&[&q[0], &q[1], &q[2]]),
            _ => r,
        }
        .send()
        .await;
        match response::<XPLeaderboardResponse>(res).await {
            Ok(mut m) => {
                let (before, after) = if let Some(b_a) = query.before_or_after {
                    match b_a {
                        query::BeforeAfter::Before(b) => (Some(b.to_string()), None),
                        query::BeforeAfter::After(b) => (None, Some(b.to_string())),
                    }
                } else {
                    (None, None)
                };
                let limit = query.limit.map(|l| l.to_string());
                let country = query.country;
                m.query = Some(xp_leaderboard::QueryCache {
                    before,
                    after,
                    limit,
                    country,
                });
                Ok(m)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the stream model.
    ///
    /// # Arguments
    ///
    /// - `stream_type`:
    ///
    /// The type of Stream.
    /// Currently
    /// [`StreamType::FortyLines`](stream::StreamType::FortyLines),
    /// [`StreamType::Blitz`](stream::StreamType::Blitz),
    /// [`StreamType::Any`](stream::StreamType::Any),
    /// or [`StreamType::League`](stream::StreamType::League).
    ///
    /// - `stream_context`:
    ///
    /// The context of the Stream.
    /// Currently
    /// [`StreamContext::Global`](stream::StreamContext::Global),
    /// [`StreamContext::UserBest`](stream::StreamContext::UserBest),
    /// or [`StreamContext::UserRecent`](stream::StreamContext::UserRecent).
    ///
    /// - `stream_identifier` (Optional):
    ///
    /// If applicable.
    /// For example, in the case of "userbest" or "userrecent", the user ID.
    ///
    /// # Examples
    ///
    /// Getting the stream object:
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     stream::{StreamType, StreamContext}
    /// };
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the stream.
    /// let user = client.get_stream(
    ///     // 40 LINES.
    ///     StreamType::FortyLines,
    ///     // User's best.
    ///     StreamContext::UserBest,
    ///     // User ID.
    ///     Some("621db46d1d638ea850be2aa0"),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Go to [`stream::StreamType`] | [`stream::StreamContext`].
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_stream(
        self,
        stream_type: stream::StreamType,
        stream_context: stream::StreamContext,
        stream_identifier: Option<&str>,
    ) -> RspErr<StreamResponse> {
        let stream_id = format!(
            "{}_{}{}",
            stream_type.as_str(),
            stream_context.as_str(),
            if let Some(i) = stream_identifier {
                format!("_{}", i)
            } else {
                String::new()
            }
        );
        let url = format!("{}streams/{}", API_URL, stream_id.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the latest news model.
    ///
    /// # Arguments
    ///
    /// - `subject`:
    ///
    /// The news subject.
    /// This argument requires a [`stream::NewsSubject`].
    ///
    /// - `limit`:
    ///
    /// The amount of entries to return.
    /// Between 1 and 100.
    /// 25 by default.
    ///
    /// # Examples
    ///
    /// Getting the latest news object:
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, stream::NewsSubject};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the latest news.
    /// let user = client.get_latest_news(
    ///     // News of the user `621db46d1d638ea850be2aa0`.
    ///     NewsSubject::User("621db46d1d638ea850be2aa0".to_string()),
    ///     // three news.
    ///     3,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Go to [`stream::StreamType`] | [`stream::StreamContext`].
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    ///
    /// # Panics
    ///
    /// Panics if the query parameter`limit` is not between 1 and 100.
    ///
    /// ```should_panic,no_run
    /// use tetr_ch::client::{Client, stream::NewsSubject};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// let user = client.get_latest_news(
    ///     NewsSubject::User("621db46d1d638ea850be2aa0".to_string()),
    ///     // 101 news.
    ///     101,
    /// ).await?;
    /// # Ok(())
    /// # }
    ///
    /// # tokio_test::block_on(run());
    /// ```
    pub async fn get_latest_news(
        self,
        subject: stream::NewsSubject,
        limit: u8,
    ) -> RspErr<LatestNewsResponse> {
        if !(1..=100).contains(&limit) {
            // !(1 <= limit && limit <= 100)
            panic!(
                "The query parameter`limit` must be between 1 and 100.\n\
                Received: {}",
                limit
            );
        }
        use stream::NewsSubject;
        let url = format!(
            "{}/news/{}",
            API_URL,
            match subject {
                NewsSubject::Any => String::new(),
                NewsSubject::Global => "global".to_string(),
                NewsSubject::User(id) => format!("user_{}", id),
            }
        );
        let res = self.client.get(url).query(&[("limit", limit)]).send().await;
        response(res).await
    }

    /// Searches for a TETR.IO user account by the social account.
    ///
    /// # Arguments
    ///
    /// - `social_connection`:
    ///
    /// The social connection to look up.
    /// This argument requires a [`search_user::SocialConnection`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, search_user::SocialConnection};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Search for a TETR.IO user account.
    /// let user = client.search_user(
    ///     SocialConnection::Discord("724976600873041940".to_string())
    /// ).await?;
    /// # Ok(())
    /// # }
    ///
    /// # tokio_test::block_on(run());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn search_user(
        self,
        social_connection: search_user::SocialConnection,
    ) -> RspErr<SearchedUserResponse> {
        let url = format!("{}users/search/{}", API_URL, social_connection.to_param());
        let res = self.client.get(url).send().await;
        response(res).await
    }
}

/// Receives `Result<Response, Error>` and returns `Result<T, ResponseError>`.
///
/// # Examples
///
/// ```ignore
/// let res = self.client.get(url).send().await;
/// response(res).await
/// ```
async fn response<T>(response: Result<Response, Error>) -> RspErr<T>
where
    for<'de> T: Deserialize<'de>,
{
    match response {
        Ok(r) => {
            if !r.status().is_success() {
                match StatusCode::from_u16(r.status().as_u16()) {
                    Ok(c) => return Err(ResponseError::HttpErr(Status::Valid(c))),
                    Err(e) => return Err(ResponseError::HttpErr(Status::Invalid(e))),
                }
            }
            match r.json().await {
                Ok(m) => Ok(m),
                Err(e) => Err(ResponseError::DeserializeErr(e.to_string())),
            }
        }
        Err(e) => Err(ResponseError::RequestErr(e.to_string())),
    }
}

pub mod query {
    //! Structs for query parameters.

    use std::num::NonZeroU8;

    /// A struct for query parameters for the TETRA LEAGUE leaderboard.
    ///
    /// `None` means default value.
    ///
    /// This structure manages the following four query parameters:
    ///
    /// - `before`(f64): The lower bound in TR.
    /// Use this to paginate upwards.
    /// Take the highest seen TR and pass that back through this field to continue scrolling.
    /// If set, the search order is reversed (returning the lowest items that match the query)
    /// This parameter is ignored if specified to get the full leaderboard.
    ///
    /// - `after`(f64): The upper bound in TR.
    /// Use this to paginate downwards.
    /// Take the lowest seen TR and pass that back through this field to continue scrolling.
    /// This parameter is ignored if specified to get the full leaderboard.
    ///
    /// - `limit`(u8): The amount of entries to return, Between `0` and `100`.
    /// 50 by default.
    /// You can specify to get the full leaderboard by passing `0`.
    /// In this case the `before` and `after` parameters are ignored.
    ///
    /// - `country`(String): The ISO 3166-1 country code to filter to.
    /// Leave unset to not filter by country.
    ///
    /// ***The `before` and `after` parameters may not be combined.**
    ///
    /// # Examples
    ///
    /// ```
    /// use tetr_ch::client::query::LeagueLeaderboardQuery;
    ///
    /// // Default(25000TR or less, 50 entries) query.
    /// let q1 = LeagueLeaderboardQuery::new();
    ///
    /// // 15200TR or less, three entries, filter by Japan.
    /// let q2 = LeagueLeaderboardQuery::new()
    ///     .after(15200.)
    ///     .limit(3)
    ///     .country("jp");
    ///
    /// // 15200TR or higher.
    /// // Also sort by TR ascending.
    /// let q3 = LeagueLeaderboardQuery::new()
    ///     .before(15200.);
    ///
    /// // Full leaderboard.
    /// let q4 = LeagueLeaderboardQuery::new()
    ///     .limit(0);
    ///
    /// // You can restore the query parameters to default as follows:
    /// let mut q5 = LeagueLeaderboardQuery::new().country("us");
    /// q5.init();
    /// ```
    #[derive(Clone, Debug, Default)]
    pub struct LeagueLeaderboardQuery {
        /// The bound in TR.
        ///
        /// The `before` and `after` parameters may not be combined,
        /// so either set the parameter with an enum or set it to default(after) by passing `None`.
        pub before_or_after: Option<BeforeAfter>,
        /// The amount of entries to return.
        pub limit: Option<Limit>,
        /// The ISO 3166-1 country code to filter to. Leave unset to not filter by country.
        /// But some vanity flags exist.
        pub country: Option<String>,
    }

    impl LeagueLeaderboardQuery {
        /// Creates a new[`LeagueLeaderboardQuery`].
        /// Values are set to default.
        ///
        /// # Examples
        ///
        /// Creates a new[`LeagueLeaderboardQuery`] with default parameters.
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new();
        /// ```
        pub fn new() -> Self {
            Self::default()
        }

        /// Initializes the [`LeagueLeaderboardQuery`].
        ///
        /// # Examples
        ///
        /// Initializes the [`LeagueLeaderboardQuery`] with default parameters.
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let mut query = LeagueLeaderboardQuery::new().country("us");
        /// query.init();
        /// ```
        pub fn init(self) -> Self {
            Self::default()
        }

        /// Set the query parameter`before`.
        ///
        /// Disabled by default.
        ///
        /// The `before` and `after` parameters may not be combined,
        /// so even if there is an `after` parameter, the `before` parameter takes precedence and overrides it.
        /// Disabled by default.
        ///
        /// This parameter is ignored if specified to get the full leaderboard.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`before` to `15200`.
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().before(15200.);
        /// ```
        pub fn before(self, bound: f64) -> Self {
            Self {
                before_or_after: Some(BeforeAfter::Before(bound)),
                ..self
            }
        }

        /// Set the query parameter`after`.
        ///
        /// 25000 by default.
        ///
        /// The `before` and `after` parameters may not be combined,
        /// so even if there is a `before` parameter, the `after` parameter takes precedence and overrides it.
        ///
        /// This parameter is ignored if specified to get the full leaderboard.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`after` to `15200`.
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().after(15200.);
        /// ```
        pub fn after(self, bound: f64) -> Self {
            Self {
                before_or_after: Some(BeforeAfter::After(bound)),
                ..self
            }
        }

        /// Set the query parameter`limit`
        /// The amount of entries to return, Between `0` and `100`.
        /// 50 by default.
        ///
        /// You can specify to get the full leaderboard by passing `0`.
        /// In this case the `before` and `after` parameters are ignored.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`limit` to `3`.
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().limit(3);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if argument`limit` is not between `0` and `100`.
        ///
        /// ```should_panic
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().limit(101);
        /// ```
        pub fn limit(self, limit: u8) -> Self {
            if
            /*0 <= limit && */
            limit <= 100 {
                Self {
                    limit: Some(if limit == 0 {
                        Limit::Full
                    } else {
                        Limit::Limit(limit)
                    }),
                    ..self
                }
            } else {
                panic!(
                    "The argument`limit` must be between  and 100.\n\
                    Received: {}",
                    limit
                );
            }
        }

        /// Set the query parameter`country`.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`country` to `jp`.
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().country("jp");
        /// ```
        pub fn country(self, country: &str) -> Self {
            Self {
                country: Some(country.to_owned().to_uppercase()),
                ..self
            }
        }

        /// Whether the query parameters`limit` is out of bounds.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::query::{LeagueLeaderboardQuery, Limit};
        /// let invalid_query = LeagueLeaderboardQuery{
        ///    limit: Some(Limit::Limit(101)),
        ///   ..LeagueLeaderboardQuery::new()
        /// };
        /// assert!(invalid_query.is_invalid_limit_range());
        /// ```
        #[allow(clippy::nonminimal_bool)]
        pub fn is_invalid_limit_range(&self) -> bool {
            if let Some(l) = self.limit.clone() {
                match l {
                    Limit::Limit(l) => !(l <= 100),
                    Limit::Full => false,
                }
            } else {
                false
            }
        }

        /// Whether the query parameters`limit` specifies to get the full leaderboard.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().limit(0);
        /// assert!(query.will_full_export());
        /// ```
        pub fn will_full_export(&self) -> bool {
            if let Some(l) = self.limit.clone() {
                match l {
                    Limit::Limit(l) => l == 0,
                    Limit::Full => true,
                }
            } else {
                false
            }
        }

        /// Builds the query parameters to `Vec<(String, String)>`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new();
        /// let query_params = query.build();
        /// ```
        pub(crate) fn build(mut self) -> Vec<(String, String)> {
            // For not pass "Full" to puery parameters.
            if self.will_full_export() {
                self.limit = Some(Limit::Full);
            }
            let mut result = Vec::new();
            if let Some(b_a) = self.before_or_after.clone() {
                match b_a {
                    BeforeAfter::Before(b) => result.push(("before".to_string(), b.to_string())),
                    BeforeAfter::After(b) => result.push(("after".to_string(), b.to_string())),
                }
            }
            if let Some(l) = self.limit.clone() {
                if !self.will_full_export() {
                    result.push(("limit".to_string(), l.to_string()));
                }
            }
            if let Some(c) = self.country {
                result.push(("country".to_string(), c));
            }
            result
        }

        /// Builds the query parameters to `Vec<(String, String)>` as full export.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let query = LeagueLeaderboardQuery::new().limit(0);
        /// let query_params = query.build_full_export();
        /// ```
        pub(crate) fn build_as_full_export(mut self) -> Vec<(String, String)> {
            // For not pass "Full" to puery parameters.
            if self.will_full_export() {
                self.limit = Some(Limit::Full);
            }
            let mut result = Vec::new();
            result.push(("limit".to_string(), "0".to_string()));
            if let Some(c) = self.country {
                result.push(("country".to_string(), c));
            }
            result
        }

        /// Initializes the [`LeagueLeaderboardQuery`].
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::query::LeagueLeaderboardQuery;
        /// let default_query = LeagueLeaderboardQuery::default();
        /// ```
        fn default() -> Self {
            Self {
                before_or_after: None,
                limit: None,
                country: None,
            }
        }
    }

    /// Amount of entries to return.
    #[derive(Clone, Debug)]
    pub enum Limit {
        /// Between 1 and 100. 50 by default.
        Limit(u8),
        Full,
    }

    impl ToString for Limit {
        fn to_string(&self) -> String {
            match self {
                Limit::Limit(l) => {
                    if l == &0 {
                        "Full".to_string()
                    } else {
                        l.to_string()
                    }
                }
                Limit::Full => "Full".to_string(),
            }
        }
    }

    /// A struct for query parameters for the XP leaderboard.
    ///
    /// `None` means default value.
    ///
    /// This structure manages the following four query parameters:
    ///
    /// - `before`(f64):  The lower bound in XP.
    /// Use this to paginate upwards.
    /// Take the highest seen XP and pass that back through this field to continue scrolling.
    /// If set, the search order is reversed (returning the lowest items that match the query)
    ///
    /// - `after`(f64): The upper bound in XP.
    /// Use this to paginate downwards.
    /// Take the lowest seen XP and pass that back through this field to continue scrolling.
    /// Infinite([`f64::INFINITY`]) by default.
    ///
    /// - `limit`([NonZeroU8]): The amount of entries to return.
    /// Between 1 and 100.
    /// 50 by default.
    ///
    /// - `country`(String): The ISO 3166-1 country code to filter to.
    /// Leave unset to not filter by country.
    ///
    /// ***The `before` and `after` parameters may not be combined.**
    ///
    /// # Examples
    ///
    /// ```
    /// use tetr_ch::client::query::XPLeaderboardQuery;
    ///
    /// // Default(descending, fifty entries) query.
    /// let q1 = XPLeaderboardQuery::new();
    ///
    /// // 50,000,000,000,000xp or less, thirty entries, filter by Japan.
    /// let q2 = XPLeaderboardQuery::new()
    ///     .after(50_000_000_000_000.)
    ///     .limit(3)
    ///     .country("jp");
    ///
    /// // 50,000,000,000,000xp or higher.
    /// // Also sort by XP ascending.
    /// let q3 = XPLeaderboardQuery::new()
    ///     .before(50_000_000_000_000.);
    ///
    /// // You can restore the query parameters to default as follows:
    /// let mut q4 = XPLeaderboardQuery::new().country("us");
    /// q4.init();
    /// ```
    #[derive(Clone, Debug, Default)]
    pub struct XPLeaderboardQuery {
        /// The bound in XP.
        ///
        /// The `before` and `after` parameters may not be combined,
        /// so either set the parameter with an enum or set it to default(after) by passing `None`.
        pub before_or_after: Option<BeforeAfter>,
        /// The amount of entries to return.
        /// Between 1 and 100. 50 by default.
        pub limit: Option<NonZeroU8>,
        /// The ISO 3166-1 country code to filter to. Leave unset to not filter by country.
        /// But some vanity flags exist.
        pub country: Option<String>,
    }

    impl XPLeaderboardQuery {
        /// Creates a new[`XPLeaderboardQuery`].
        /// Values are set to default.
        ///
        /// # Examples
        ///
        /// Creates a new[`XPLeaderboardQuery`] with default parameters.
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let query = XPLeaderboardQuery::new();
        /// ```
        pub fn new() -> Self {
            Self::default()
        }

        /// Initializes the [`XPLeaderboardQuery`].
        ///
        /// # Examples
        ///
        /// Initializes the [`XPLeaderboardQuery`] with default parameters.
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new();
        /// query.init();
        /// ```
        pub fn init(self) -> Self {
            Self::default()
        }

        /// Set the query parameter`before`.
        ///
        /// The `before` and `after` parameters may not be combined,
        /// so even if there is an `after` parameter, the `before` parameter takes precedence and overrides it.
        /// Disabled by default.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`before` to `50,000,000,000,000`.
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new()
        ///     .before(50_000_000_000_000.);
        /// ```
        pub fn before(self, bound: f64) -> Self {
            Self {
                before_or_after: if bound.is_infinite() {
                    Some(BeforeAfter::Before(bound))
                } else {
                    None
                },
                ..self
            }
        }

        /// Set the query parameter`after`.
        ///
        /// The `before` and `after` parameters may not be combined,
        /// so even if there is a `before` parameter, the `after` parameter takes precedence and overrides it.
        /// Infinite([`f64::INFINITY`]) by default.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`after` to `50,000,000,000,000`.
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new()
        ///     .after(50_000_000_000_000.);
        /// ```
        pub fn after(self, bound: f64) -> Self {
            Self {
                before_or_after: Some(BeforeAfter::After(bound)),
                ..self
            }
        }

        /// Set the query parameter`limit`
        /// The amount of entries to return, Between `1` and `100`.
        /// 50 by default.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`limit` to `5`.
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new().limit(5);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if argument`limit` is not between `1` and `100`.
        ///
        /// ```should_panic
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new().limit(0);
        /// ```
        ///
        /// ```should_panic
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new().limit(101);
        /// ```
        pub fn limit(self, limit: u8) -> Self {
            if (1..=100).contains(&limit) {
                // 1 <= limit && limit <= 100
                Self {
                    limit: Some(NonZeroU8::new(limit).unwrap()),
                    ..self
                }
            } else {
                panic!(
                    "The argument`limit` must be between 1 and 100.\n\
                    Received: {}",
                    limit
                );
            }
        }

        /// Set the query parameter`country`.
        ///
        /// # Examples
        ///
        /// Sets the query parameter`country` to `ca`.
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let mut query = XPLeaderboardQuery::new().country("ca");
        /// ```
        pub fn country(self, country: &str) -> Self {
            Self {
                country: Some(country.to_owned().to_uppercase()),
                ..self
            }
        }

        /// Whether the query parameters`limit` is out of bounds.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// use std::num::NonZeroU8;
        ///
        /// let invalid_query = XPLeaderboardQuery{
        ///     limit: Some(NonZeroU8::new(101).unwrap()),
        ///     ..XPLeaderboardQuery::new()
        /// };
        /// assert!(invalid_query.is_invalid_limit_range());
        /// ```
        #[allow(clippy::nonminimal_bool)]
        pub fn is_invalid_limit_range(&self) -> bool {
            if let Some(l) = self.limit {
                !(l <= NonZeroU8::new(100).unwrap())
            } else {
                false
            }
        }

        /// Builds the query parameters to `Vec<(String, String)>`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let query = XPLeaderboardQuery::new();
        /// let query_params = query.build();
        /// ```
        pub(crate) fn build(mut self) -> Vec<(String, String)> {
            // For not pass "inf" to puery parameters.
            if let Some(BeforeAfter::After(b)) = self.before_or_after {
                if b.is_infinite() {
                    self.before_or_after = None;
                }
            }
            let mut result = Vec::new();
            if let Some(b_a) = self.before_or_after.clone() {
                match b_a {
                    BeforeAfter::Before(b) => result.push(("before".to_string(), b.to_string())),
                    BeforeAfter::After(b) => result.push(("after".to_string(), b.to_string())),
                }
            }
            if let Some(l) = self.limit {
                result.push(("limit".to_string(), l.to_string()));
            }
            if let Some(c) = self.country {
                result.push(("country".to_string(), c));
            }
            result
        }

        /// Returns the default [`XPLeaderboardQuery`].
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::query::XPLeaderboardQuery;
        /// let query = XPLeaderboardQuery::default();
        /// ```
        fn default() -> Self {
            Self {
                before_or_after: None,
                limit: None,
                country: None,
            }
        }
    }

    /// The bound.
    ///
    /// The `before` and `after` parameters may not be combined,
    /// so need to either set the parameter.
    #[derive(Clone, Debug)]
    pub enum BeforeAfter {
        /// The lower bound.
        /// Use this to paginate upwards.
        /// Take the highest seen value and pass that back through this field to continue scrolling.
        /// If set, the search order is reversed (returning the lowest items that match the query)
        Before(f64),
        /// Use this to paginate downwards.
        /// Take the lowest seen value and pass that back through this field to continue scrolling.
        After(f64),
    }
}

pub mod stream {
    //! Features for streams.

    /// Enum for the stream type.
    pub enum StreamType {
        /// 40 LINES
        FortyLines,
        /// BLITZ
        Blitz,
        /// Any
        Any,
        /// TETRA LEAGUE
        League,
    }

    impl StreamType {
        /// Converts to a `&str`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::stream::StreamType;
        /// let forty_lines = StreamType::FortyLines;
        /// let blitz = StreamType::Blitz;
        /// let any = StreamType::Any;
        /// let league = StreamType::League;
        /// assert_eq!(forty_lines.as_str(), "40l");
        /// assert_eq!(blitz.as_str(), "blitz");
        /// assert_eq!(any.as_str(), "any");
        /// assert_eq!(league.as_str(), "league");
        /// ```
        pub(crate) fn as_str(&self) -> &str {
            match self {
                StreamType::FortyLines => "40l",
                StreamType::Blitz => "blitz",
                StreamType::Any => "any",
                StreamType::League => "league",
            }
        }
    }

    /// Enum for the stream context.
    pub enum StreamContext {
        Global,
        UserBest,
        UserRecent,
    }

    impl StreamContext {
        /// Converts to a `&str`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::stream::StreamContext;
        /// let global = StreamContext::Global;
        /// let user_best = StreamContext::UserBest;
        /// let user_recent = StreamContext::UserRecent;
        /// assert_eq!(global.as_str(), "global");
        /// assert_eq!(user_best.as_str(), "user_best");
        /// assert_eq!(user_recent.as_str(), "user_recent");
        pub(crate) fn as_str(&self) -> &str {
            match self {
                StreamContext::Global => "global",
                StreamContext::UserBest => "userbest",
                StreamContext::UserRecent => "userrecent",
            }
        }
    }

    /// The news subject.
    pub enum NewsSubject {
        /// News of all users
        Any,
        /// Global news.
        Global,
        /// The news of the user.
        /// Enter the user's **ID** to `String`.
        User(String),
    }
}

pub mod search_user {
    //! Features for searching users.

    /// The social connection.
    ///
    /// The API documentation says searching for the other social links will be added in the near future.
    pub enum SocialConnection {
        /// A Discord ID.
        Discord(String),
    }

    impl SocialConnection {
        /// Converts into a parameter.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::search_user::SocialConnection;
        /// let discord_id = "724976600873041940".to_string();
        /// assert_eq!(SocialConnection::Discord(discord_id).to_param(), "discord:724976600873041940");
        /// ```
        pub(crate) fn to_param(&self) -> String {
            match self {
                SocialConnection::Discord(id) => format!("discord:{}", id),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_new_client() {
        let _ = Client::new();
    }

    #[test]
    fn init_league_query() {
        let mut _query = query::LeagueLeaderboardQuery::new();
        _query.init();
    }

    #[test]
    #[should_panic]
    fn panic_invalid_limit_range_in_league_query() {
        let mut _query = query::LeagueLeaderboardQuery::new();
        _query.limit(101);
    }

    #[test]
    fn init_xp_query() {
        let mut _query = query::XPLeaderboardQuery::new();
        _query.init();
    }

    #[test]
    #[should_panic]
    fn panic_invalid_limit_range_in_xp_query_with101() {
        let mut _query = query::XPLeaderboardQuery::new();
        _query.limit(101);
    }

    #[test]
    #[should_panic]
    fn panic_invalid_limit_range_in_xp_query_with0() {
        let mut _query = query::XPLeaderboardQuery::new();
        _query.limit(101);
    }

    #[test]
    fn fortylines_as_str() {
        assert_eq!(stream::StreamType::FortyLines.as_str(), "40l");
    }

    #[test]
    fn blitz_as_str() {
        assert_eq!(stream::StreamType::Blitz.as_str(), "blitz");
    }

    #[test]
    fn any_as_str() {
        assert_eq!(stream::StreamType::Any.as_str(), "any");
    }

    #[test]
    fn league_as_str() {
        assert_eq!(stream::StreamType::League.as_str(), "league");
    }

    #[test]
    fn global_as_str() {
        assert_eq!(stream::StreamContext::Global.as_str(), "global");
    }

    #[test]
    fn userbest_as_str() {
        assert_eq!(stream::StreamContext::UserBest.as_str(), "userbest");
    }

    #[test]
    fn userrecent_as_str() {
        assert_eq!(stream::StreamContext::UserRecent.as_str(), "userrecent");
    }
}
