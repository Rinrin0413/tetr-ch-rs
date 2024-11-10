//! Client for API requests.

use self::{
    param::{
        news_stream::NewsStream,
        pagination::Bound,
        record::{self, Gamemode},
        search_user::SocialConnection,
        user_leaderboard::{self, LeaderboardType},
    },
    response::response,
};
use crate::{
    error::ResponseError,
    model::{
        achievement_info::AchievementInfoResponse,
        labs::{
            league_ranks::LabsLeagueRanksResponse, leagueflow::LabsLeagueflowResponse,
            scoreflow::LabsScoreflowResponse,
        },
        leaderboard::{HistoricalLeaderboardResponse, LeaderboardResponse},
        news::{NewsAllResponse, NewsLatestResponse},
        records_leaderboard::RecordsLeaderboardResponse,
        searched_user::SearchedUserResponse,
        server_activity::ServerActivityResponse,
        server_stats::ServerStatsResponse,
        summary::{
            achievements::AchievementsResponse,
            blitz::BlitzResponse,
            forty_lines::FortyLinesResponse,
            league::LeagueResponse,
            zen::ZenResponse,
            zenith::{ZenithExResponse, ZenithResponse},
            AllSummariesResponse,
        },
        user::UserResponse,
        user_records::UserRecordsResponse,
    },
};
use reqwest::{self};

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

    /// Returns the object containing all the user's summaries in one.
    ///
    /// ***consider whether you really need this.
    /// If you only collect data for one or two game modes,
    /// use the individual summaries' methods instead.**
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
    /// // Get All the User Summaries.
    /// let user = client.get_user_all_summaries("rinrin-rs").await?;
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
    pub async fn get_user_all_summaries(self, user: &str) -> RspErr<AllSummariesResponse> {
        let url = format!("{}users/{}/summaries", API_URL, user.to_lowercase());
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

    /// Returns the object describing a summary of the user's QUICK PLAY games.
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
    /// // Get the User Summary QUICK PLAY.
    /// let user = client.get_user_zenith("rinrin-rs").await?;
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
    pub async fn get_user_zenith(self, user: &str) -> RspErr<ZenithResponse> {
        let url = format!("{}users/{}/summaries/zenith", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the object describing a summary of the user's EXPERT QUICK PLAY games.
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
    /// // Get the User Summary EXPERT QUICK PLAY.
    /// let user = client.get_user_zenith_ex("rinrin-rs").await?;
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
    pub async fn get_user_zenith_ex(self, user: &str) -> RspErr<ZenithExResponse> {
        let url = format!(
            "{}users/{}/summaries/zenithex",
            API_URL,
            user.to_lowercase()
        );
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the object describing a summary of the user's TETRA LEAGUE standing.
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
    /// // Get the User Summary TETRA LEAGUE.
    /// let user = client.get_user_league("rinrin-rs").await?;
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
    pub async fn get_user_league(self, user: &str) -> RspErr<LeagueResponse> {
        let url = format!("{}users/{}/summaries/league", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the object describing a summary of the user's ZEN progress.
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
    /// // Get the User Summary ZEN.
    /// let user = client.get_user_zen("rinrin-rs").await?;
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
    pub async fn get_user_zen(self, user: &str) -> RspErr<ZenResponse> {
        let url = format!("{}users/{}/summaries/zen", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the object containing all the user's achievements.
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
    /// // Get the User Summary Achievements.
    /// let user = client.get_user_achievements("rinrin-rs").await?;
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
    pub async fn get_user_achievements(self, user: &str) -> RspErr<AchievementsResponse> {
        let url = format!(
            "{}users/{}/summaries/achievements",
            API_URL,
            user.to_lowercase()
        );
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the array of users fulfilling the search criteria.
    ///
    /// # Arguments
    ///
    /// - `leaderboard`: The leaderboard to sort users by.
    /// - `search_criteria`: The search criteria to filter users by.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::user_leaderboard::{self, LeaderboardType}
    /// };
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// let criteria = user_leaderboard::SearchCriteria::new()
    ///     // Upper bound is `[15200, 0, 0]`
    ///     .after([15200.,0.,0.])
    ///     // Three entries
    ///     .limit(3)
    ///     // Filter by Japan
    ///     .country("jp");
    ///
    /// // Get the User Leaderboard.
    /// let user = client.get_leaderboard(
    ///     LeaderboardType::League,
    ///     Some(criteria)
    /// ).await?;
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
    pub async fn get_leaderboard(
        self,
        leaderboard: LeaderboardType,
        search_criteria: Option<user_leaderboard::SearchCriteria>,
    ) -> RspErr<LeaderboardResponse> {
        let mut query_params = Vec::new();
        if let Some(criteria) = search_criteria {
            if criteria.is_invalid_limit_range() {
                panic!(
                    "The query parameter`limit` must be between 0 and 100.\n\
                    Received: {}",
                    criteria.limit.unwrap()
                );
            }
            query_params = criteria.build();
        }
        let url = format!("{}users/by/{}", API_URL, leaderboard.to_param());
        let res = self.client.get(url).query(&query_params).send().await;
        response(res).await
    }

    /// Returns the array of historical user blobs fulfilling the search criteria.
    ///
    /// # Arguments
    ///
    /// - `season`: The season to look up. (e.g. `"1"`)
    /// - `search_criteria`: The search criteria to filter users by.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::user_leaderboard::{self, LeaderboardType}
    /// };
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// let criteria = user_leaderboard::SearchCriteria::new()
    ///     // Upper bound is `[15200, 0, 0]`
    ///     .after([15200.,0.,0.])
    ///     // Three entries
    ///     .limit(3)
    ///     // Filter by Japan
    ///     .country("jp");
    ///
    /// // Get the User Leaderboard.
    /// let user = client.get_historical_league_leaderboard(
    ///     "1",
    ///     Some(criteria)
    /// ).await?;
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
    pub async fn get_historical_league_leaderboard(
        self,
        season: &str,
        search_criteria: Option<user_leaderboard::SearchCriteria>,
    ) -> RspErr<HistoricalLeaderboardResponse> {
        let mut query_params = Vec::new();
        if let Some(criteria) = search_criteria {
            if criteria.is_invalid_limit_range() {
                panic!(
                    "The query parameter`limit` must be between 0 and 100.\n\
                    Received: {}",
                    criteria.limit.unwrap()
                );
            }
            query_params = criteria.build();
        }
        let url = format!(
            "{}users/history/{}/{}",
            API_URL,
            LeaderboardType::League.to_param(),
            season
        );
        let res = self.client.get(url).query(&query_params).send().await;
        response(res).await
    }

    /// Returns the list of Records fulfilling the search criteria.
    ///
    /// # Arguments
    ///
    /// - `user`: The username or user ID to look up.
    /// - `gamemode`: The game mode to look up.
    /// - `leaderboard`: The personal leaderboard to look up.
    /// - `search_criteria`: The search criteria to filter records by.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::record::{self, Gamemode, LeaderboardType}
    /// };
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Set the search criteria.
    /// let criteria = record::SearchCriteria::new()
    ///     // Upper bound is `[500000, 0, 0]`
    ///     .after([500000.,0.,0.])
    ///     // Three entries
    ///     .limit(3);
    ///
    /// // Get the User Records.
    /// let user = client.get_user_records(
    ///     "rinrin-rs",
    ///     Gamemode::FortyLines,
    ///     LeaderboardType::Top,
    ///     Some(criteria)
    /// ).await?;
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
    pub async fn get_user_records(
        self,
        user: &str,
        gamemode: Gamemode,
        leaderboard: record::LeaderboardType,
        search_criteria: Option<record::SearchCriteria>,
    ) -> RspErr<UserRecordsResponse> {
        let mut query_params = Vec::new();
        if let Some(criteria) = search_criteria {
            if criteria.is_invalid_limit_range() {
                panic!(
                    "The query parameter`limit` must be between 0 and 100.\n\
                    Received: {}",
                    criteria.limit.unwrap()
                );
            }
            query_params = criteria.build();
        }
        let url = format!(
            "{}users/{}/records/{}/{}",
            API_URL,
            user.to_lowercase(),
            gamemode.to_param(),
            leaderboard.to_param()
        );
        let res = self.client.get(url).query(&query_params).send().await;
        response(res).await
    }

    /// Returns the list of Records fulfilling the search criteria.
    ///
    /// # Arguments
    ///
    /// - `leaderboard`: The leaderboard to look up.
    /// - `search_criteria`: The search criteria to filter records by.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     records_leaderboard::{
    ///         RecordsLeaderboardId,
    ///         RecordsLeaderboardSearchCriteria,
    ///         Scope
    ///     }
    /// };
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Set the search criteria.
    /// let criteria = RecordsLeaderboardSearchCriteria::new()
    ///     // Upper bound is `[500000, 0, 0]`
    ///     .after([500000.,0.,0.])
    ///     // Three entries
    ///     .limit(3);
    ///
    /// // Get the Records Leaderboard.
    /// let user = client.get_records_leaderboard(
    ///     RecordsLeaderboardId::new(
    ///         "blitz",
    ///         Scope::Country("JP".to_string()),
    ///         Some("@2024w31")
    ///     ),
    ///    Some(criteria)
    /// ).await?;
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
    pub async fn get_records_leaderboard(
        self,
        leaderboard: records_leaderboard::RecordsLeaderboardId,
        search_criteria: Option<records_leaderboard::RecordsLeaderboardSearchCriteria>,
    ) -> RspErr<RecordsLeaderboardResponse> {
        let mut query_params = Vec::new();
        if let Some(criteria) = search_criteria {
            if criteria.is_invalid_limit_range() {
                panic!(
                    "The query parameter`limit` must be between 0 and 100.\n\
                    Received: {}",
                    criteria.limit.unwrap()
                );
            }
            query_params = criteria.build();
        }
        let url = format!("{}records/{}", API_URL, leaderboard.to_param());
        let res = self.client.get(url).query(&query_params).send().await;
        response(res).await
    }

    /// Searches for a record.
    ///
    /// Only one record is returned.
    /// It is generally not possible for a player to play the same gamemode twice in a millisecond.
    ///
    /// # Arguments
    ///
    /// - `user_id`: The user ID to look up.
    /// - `gamemode`: The game mode to look up.
    /// - `timestamp`: The timestamp of the record to find.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{param::record::Gamemode, Client};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the User Record.
    /// let user = client.search_record(
    ///     "621db46d1d638ea850be2aa0",
    ///     Gamemode::Blitz,
    ///     1680053762145
    /// ).await?;
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
    pub async fn search_record(
        self,
        user_id: &str,
        gamemode: Gamemode,
        timestamp: i64,
    ) -> RspErr<serde_json::Value> {
        let query_params = [
            ("user", user_id.to_string()),
            ("gamemode", gamemode.to_param()),
            ("ts", timestamp.to_string()),
        ];
        let url = format!("{}records/reverse", API_URL);
        let res = self.client.get(url).query(&query_params).send().await;
        response(res).await
    }

    /// Returns the latest news items in any stream.
    ///
    /// # Arguments
    ///
    /// - `limit`:The amount of entries to return,
    /// between 1 and 100. 25 by default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the All Latest News.
    /// let user = client.get_news_all(Some(3)).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Error
    ///
    /// Returns a [`ResponseError::DeserializeErr`] if there are some mismatches in the API docs,
    /// or when this library is defective.
    ///
    /// Returns a [`ResponseError::RequestErr`] redirect loop was detected or redirect limit was exhausted.
    ///
    /// Returns a [`ResponseError::HttpErr`] if the HTTP request fails.
    pub async fn get_news_all(self, limit: Option<u8>) -> RspErr<NewsAllResponse> {
        let mut query_param = Vec::new();
        if let Some(l) = limit {
            if !(1..=100).contains(&l) {
                // !(1 <= limit && limit <= 100)
                panic!(
                    "The query parameter`limit` must be between 1 and 100.\n\
                    Received: {}",
                    l
                );
            }
            query_param.push(("limit", l.to_string()));
        }
        let url = format!("{}news/", API_URL);
        let res = self.client.get(url).query(&query_param).send().await;
        response(res).await
    }

    /// Returns latest news items in the stream.
    ///
    /// # Arguments
    ///
    /// - `stream`: The news stream to look up.
    ///
    /// - `limit`: The amount of entries to return, between 1 and 100.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, param::news_stream::NewsStream};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the latest news.
    /// let user = client.get_news_latest(
    ///     // News of the user `621db46d1d638ea850be2aa0`.
    ///     NewsStream::User("621db46d1d638ea850be2aa0".to_string()),
    ///     // three news.
    ///     3,
    /// ).await?;
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
    ///
    /// # Panics
    ///
    /// Panics if the query parameter `limit` is not between 1 and 100.
    ///
    /// ```should_panic,no_run
    /// use tetr_ch::client::{Client, param::news_stream::NewsStream};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// let user = client.get_news_latest(
    ///     NewsStream::Global,
    ///     // 101 news. (not allowed)
    ///     101,
    /// ).await?;
    /// # Ok(())
    /// # }
    ///
    /// # tokio_test::block_on(run());
    /// ```
    pub async fn get_news_latest(
        self,
        stream: NewsStream,
        limit: u8,
    ) -> RspErr<NewsLatestResponse> {
        if !(1..=100).contains(&limit) {
            // !(1 <= limit && limit <= 100)
            panic!(
                "The query parameter`limit` must be between 1 and 100.\n\
                Received: {}",
                limit
            );
        }
        let url = format!("{}news/{}", API_URL, stream.to_param());
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
    /// use tetr_ch::client::{Client, param::search_user::SocialConnection};
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
        social_connection: SocialConnection,
    ) -> RspErr<SearchedUserResponse> {
        let url = format!("{}users/search/{}", API_URL, social_connection.to_param());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the condensed graph of all of the user's records in the gamemode.
    ///
    /// # Arguments
    ///
    /// - `user`: The username or user ID to look up.
    /// - `gamemode`: The game mode to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::{param::record::Gamemode, Client};
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the Labs Scoreflow.
    /// let user = client.get_labs_scoreflow(
    ///     "rinrin-rs",
    ///     Gamemode::FortyLines
    /// ).await?;
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
    pub async fn get_labs_scoreflow(
        self,
        user: &str,
        gamemode: Gamemode,
    ) -> RspErr<LabsScoreflowResponse> {
        let url = format!(
            "{}labs/scoreflow/{}/{}",
            API_URL,
            user.to_lowercase(),
            gamemode.to_param()
        );
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the condensed graph of all of the user's matches in TETRA LEAGUE.
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
    ///
    /// // Get the Labs Leagueflow.
    /// let user = client.get_labs_leagueflow("rinrin-rs").await?;
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
    pub async fn get_labs_leagueflow(self, user: &str) -> RspErr<LabsLeagueflowResponse> {
        let url = format!("{}labs/leagueflow/{}", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the view over all TETRA LEAGUE ranks and their metadata.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the Labs League Ranks.
    /// let user = client.get_labs_league_ranks().await?;
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
    pub async fn get_labs_league_ranks(self) -> RspErr<LabsLeagueRanksResponse> {
        let url = format!("{}labs/league_ranks", API_URL);
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Returns the data about the achievement itself, its cutoffs, and its leaderboard.
    ///
    /// # Arguments
    ///
    /// - `achievement_id`: The achievement ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    /// # use std::io;
    ///
    /// # async fn run() -> io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the Achievement Info.
    /// let user = client.get_achievement_info("15").await?;
    /// # Ok(())
    /// # }
    /// ```
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
    pub async fn get_achievement_info(
        self,
        achievement_id: &str,
    ) -> RspErr<AchievementInfoResponse> {
        let url = format!("{}achievements/{}", API_URL, achievement_id);
        let res = self.client.get(url).send().await;
        response(res).await
    }
}

pub mod param;
mod response;

pub mod records_leaderboard {
    //! Features for records leaderboards.

    use super::*;

    /// The records leaderboard ID.
    pub struct RecordsLeaderboardId {
        /// The game mode. e.g. `40l`.
        pub gamemode: String,
        /// The scope.
        pub scope: Scope,
        /// An optional Revolution ID. e.g. `@2024w31`.
        pub revolution_id: Option<String>,
    }

    impl RecordsLeaderboardId {
        /// Creates a new [`RecordsLeaderboardId`].
        ///
        /// # Arguments
        ///
        /// - `gamemode`: The game mode. e.g. `40l`.
        /// - `scope`: The scope. ether [`Scope::Global`] or [`Scope::Country`].
        /// - `revolution_id`: An optional Revolution ID. e.g. `@2024w31`.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::{RecordsLeaderboardId, Scope};
        /// let id = RecordsLeaderboardId::new("40l", Scope::Global, None);
        /// ```
        pub fn new(gamemode: &str, scope: Scope, revolution_id: Option<&str>) -> Self {
            Self {
                gamemode: gamemode.to_owned(),
                scope,
                revolution_id: revolution_id.map(|s| s.to_owned()),
            }
        }

        /// Converts into a parameter.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::records_leaderboard::{RecordsLeaderboardId, Scope};
        /// let id1 = RecordsLeaderboardId::new("40l", Scope::Global, None);
        /// let id2 = RecordsLeaderboardId::new("blitz", Scope::Country("JP".to_string()), None);
        /// let id3 = RecordsLeaderboardId::new("zenith", Scope::Global, Some("@2024w31"));
        /// assert_eq!(id1.to_param(), "40l_global");
        /// assert_eq!(id2.to_param(), "blitz_country_JP");
        /// assert_eq!(id3.to_param(), "zenith_global@2024w31");
        /// ```
        pub(crate) fn to_param(&self) -> String {
            match &self.scope {
                Scope::Global => format!("{}_global", self.gamemode),
                Scope::Country(c) => format!("{}_country_{}", self.gamemode, c.to_uppercase()),
            }
        }
    }

    /// The scope of the records leaderboard.
    pub enum Scope {
        /// The global scope.
        Global,
        /// The country scope.
        /// e.g. `JP`.
        Country(String),
    }

    /// The search criteria for the records leaderboard.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
    ///
    /// // Default search criteria.
    /// let c1 = RecordsLeaderboardSearchCriteria::new();
    ///
    /// // Upper bound is `[500000, 0, 0]`, three entries.
    /// let c2 = RecordsLeaderboardSearchCriteria::new()
    ///     .after([500000., 0., 0.])
    ///     .limit(3);
    ///
    /// // Lower bound is `[500000, 0, 0]`.
    /// // Also the search order is reversed.
    /// let c3 = RecordsLeaderboardSearchCriteria::new()
    ///     .before([500000., 0., 0.]);
    ///
    /// // You can initialize the search criteria to default as follows:
    /// let mut c4 = RecordsLeaderboardSearchCriteria::new().limit(10);
    /// c4.init();
    /// ```
    #[derive(Clone, Debug, Default)]
    pub struct RecordsLeaderboardSearchCriteria {
        /// The bound to paginate.
        pub bound: Option<Bound>,
        /// The amount of entries to return,
        /// between 1 and 100. 25 by default.
        pub limit: Option<u8>,
    }

    impl RecordsLeaderboardSearchCriteria {
        /// Creates a new [`RecordsLeaderboardSearchCriteria`].
        /// The values are set to default.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let criteria = RecordsLeaderboardSearchCriteria::new();
        /// ```
        pub fn new() -> Self {
            Self::default()
        }

        /// Initializes the search criteria.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let mut criteria = RecordsLeaderboardSearchCriteria::new();
        /// criteria.init();
        /// ```
        pub fn init(self) -> Self {
            Self::default()
        }

        /// Sets the upper bound.
        ///
        /// # Arguments
        ///
        /// - `bound`: The upper bound to paginate downwards:
        /// take the lowest seen prisecter and pass that back through this field to continue scrolling.
        ///
        /// A **prisecter** is consisting of three floats.
        /// The `prisecter` field in a response data allows you to continue paginating.
        ///
        /// # Examples
        ///
        /// Sets the upper bound to `[500000.0, 0.0, 0.0]`.
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let mut criteria = RecordsLeaderboardSearchCriteria::new();
        /// criteria.after([500000.0, 0.0, 0.0]);
        /// ```
        pub fn after(self, bound: [f64; 3]) -> Self {
            Self {
                bound: Some(Bound::After(bound)),
                ..self
            }
        }

        /// Sets the lower bound.
        ///
        /// # Arguments
        ///
        /// - `bound`: The lower bound to paginate upwards:
        /// take the highest seen prisecter and pass that back through this field to continue scrolling.
        /// If use this, the search order is reversed
        /// (returning the lowest items that match the query)
        ///
        /// A **prisecter** is consisting of three floats.
        /// The `prisecter` field in a response data allows you to continue paginating.
        ///
        /// # Examples
        ///
        /// Sets the lower bound to `[500000.0, 0.0, 0.0]`.
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let mut criteria = RecordsLeaderboardSearchCriteria::new();
        /// criteria.before([500000.0, 0.0, 0.0]);
        /// ```
        pub fn before(self, bound: [f64; 3]) -> Self {
            Self {
                bound: Some(Bound::Before(bound)),
                ..self
            }
        }

        /// Limits the amount of entries to return.
        ///
        /// # Arguments
        ///
        /// - `limit`: The amount of entries to return.
        /// Between 1 and 100. 25 by default.
        ///
        /// # Examples
        ///
        /// Limits the amount of entries to return to `10`.
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let mut criteria = RecordsLeaderboardSearchCriteria::new();
        /// criteria.limit(10);
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if the argument `limit` is not between `1` and `100`.
        ///
        /// ```should_panic
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let mut criteria = RecordsLeaderboardSearchCriteria::new();
        /// criteria.limit(0);
        /// ```
        ///
        /// ```should_panic
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let mut criteria = RecordsLeaderboardSearchCriteria::new();
        /// criteria.limit(101);
        /// ```
        pub fn limit(self, limit: u8) -> Self {
            if (1..=100).contains(&limit) {
                Self {
                    limit: Some(limit),
                    ..self
                }
            } else {
                panic!(
                    "The argument `limit` must be between 1 and 100.\n\
                    Received: {}",
                    limit
                );
            }
        }

        /// Whether the search criteria `limit` is out of bounds.
        ///
        /// # Examples
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let invalid_criteria = RecordsLeaderboardSearchCriteria {
        ///     limit: Some(0),
        ///     ..RecordsLeaderboardSearchCriteria::new()
        /// };
        /// assert!(invalid_criteria.is_invalid_limit_range());
        /// ```
        ///
        /// ```
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let invalid_criteria = RecordsLeaderboardSearchCriteria {
        ///     limit: Some(101),
        ///     ..RecordsLeaderboardSearchCriteria::new()
        /// };
        /// assert!(invalid_criteria.is_invalid_limit_range());
        /// ```
        pub fn is_invalid_limit_range(&self) -> bool {
            if let Some(l) = self.limit {
                !(1..=100).contains(&l)
            } else {
                false
            }
        }

        /// Builds the search criteria to `Vec<(String, String)>`.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use tetr_ch::client::records_leaderboard::RecordsLeaderboardSearchCriteria;
        /// let criteria = RecordsLeaderboardSearchCriteria::new();
        /// let query_params = criteria.build();
        /// ```
        pub(crate) fn build(self) -> Vec<(String, String)> {
            let mut result = Vec::new();
            if let Some(b) = self.bound {
                result.push(b.to_query_param());
            }
            if let Some(l) = self.limit {
                result.push(("limit".to_string(), l.to_string()));
            }
            result
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
}
