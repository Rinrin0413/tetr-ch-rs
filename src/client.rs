//! A module for the [`Client`] struct and supporting types.

use self::{
    error::RspErr,
    param::{
        news_stream::NewsStream,
        record::{self, Gamemode},
        record_leaderboard::{self, RecordsLeaderboardId},
        search_user::SocialConnection,
        user_leaderboard::{self, LeaderboardType},
    },
    response::response,
};
use crate::model::{
    achievement_info::AchievementInfoResponse,
    labs::{
        league_ranks::LabsLeagueRanksResponse, leagueflow::LabsLeagueflowResponse,
        scoreflow::LabsScoreflowResponse,
    },
    leaderboard::{HistoricalLeaderboardResponse, LeaderboardResponse},
    news::{NewsAllResponse, NewsLatestResponse},
    records_leaderboard::RecordsLeaderboardResponse,
    searched_record::SearchedRecordResponse,
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
};
use reqwest::{self};

const API_URL: &str = "https://ch.tetr.io/api/";

/// A client for API requests.
///
/// # Examples
///
/// Creating a new [`Client`] instance and getting information about the user "RINRIN-RS".
///
/// ```no_run
/// use tetr_ch::client::Client;
///
/// # async fn run() -> std::io::Result<()> {
/// // Create a new client.
/// let client = Client::new();
/// // Get the user information.
/// let user = client.get_user("rinrin-rs").await?;
/// # Ok(())
/// # }
/// ```
///
/// [See more examples](https://github.com/Rinrin0413/tetr-ch-rs/tree/master/examples)
#[non_exhaustive]
#[derive(Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    //! # Errors
    //!
    //! The `get_*` methods and `search_*` methods return a `Result<T, ResponseError>`.
    //!
    //! - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    //! if the request failed.
    //! - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    //! if the response did not match the expected format but the HTTP request succeeded.
    //! There may be defectives in this wrapper or the TETRA CHANNEL API document.
    //! - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    //! if the HTTP request failed and the response did not match the expected format.
    //! Even if the HTTP request failed,
    //! it may be possible to deserialize the response containing an error message,
    //! so the deserialization will be tried before returning this error.

    /// Creates a new [`Client`].
    ///
    /// # Examples
    ///
    /// ```
    /// use tetr_ch::client::Client;
    ///
    /// // Create a new client.
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Gets the detailed information about the specified user.
    ///
    /// About the endpoint "User Info",
    /// see the [API document](https://tetr.io/about/api/#usersuser).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the information about the user "RINRIN-RS".
    /// let user = client.get_user("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user(self, user: &str) -> RspErr<UserResponse> {
        let url = format!("{}users/{}", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets some statistics about the TETR.IO.
    ///
    /// About the endpoint "Server Statistics",
    /// see the [API document](https://tetr.io/about/api/#generalstats).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the statistics.
    /// let user = client.get_server_stats().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_stats(self) -> RspErr<ServerStatsResponse> {
        let url = format!("{}general/stats", API_URL);
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the array of the user activity over the last 2 days.
    ///
    /// About the endpoint "Server Activity",
    /// see the [API document](https://tetr.io/about/api/#generalactivity).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the activity.
    /// let user = client.get_server_activity().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_activity(self) -> RspErr<ServerActivityResponse> {
        let url = format!("{}general/activity", API_URL);
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets all the summaries of the specified user.
    ///
    /// ***Consider whether you really need to use this method.
    /// If you only collect data for one or two game modes,
    /// use the methods for the individual summaries instead.**
    ///
    /// About the endpoint "User Summaries",
    /// see the [API document](https://tetr.io/about/api/#usersusersummaries).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get all the summaries of the user "RINRIN-RS".
    /// let user = client.get_user_all_summaries("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_all_summaries(self, user: &str) -> RspErr<AllSummariesResponse> {
        let url = format!("{}users/{}/summaries", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the summary of the specified user's 40 LINES games.
    ///
    /// About the endpoint "User Summary: 40 LINES",
    /// see the [API document](https://tetr.io/about/api/#usersusersummaries40l).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the summary of the 40 LINES games of the user "RINRIN-RS".
    /// let user = client.get_user_40l("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_40l(self, user: &str) -> RspErr<FortyLinesResponse> {
        let url = format!("{}users/{}/summaries/40l", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the summary of the specified user's BLITZ games.
    ///
    /// About the endpoint "User Summary: BLITZ",
    /// see the [API document](https://tetr.io/about/api/#usersusersummariesblitz).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the summary of the BLITZ games of the user "RINRIN-RS".
    /// let user = client.get_user_blitz("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_blitz(self, user: &str) -> RspErr<BlitzResponse> {
        let url = format!("{}users/{}/summaries/blitz", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the summary of the specified user's QUICK PLAY games.
    ///
    /// About the endpoint "User Summary: QUICK PLAY",
    /// see the [API document](https://tetr.io/about/api/#usersusersummarieszenith).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the summary of the QUICK PLAY games of the user "RINRIN-RS".
    /// let user = client.get_user_zenith("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_zenith(self, user: &str) -> RspErr<ZenithResponse> {
        let url = format!("{}users/{}/summaries/zenith", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the summary of the specified user's EXPERT QUICK PLAY games.
    ///
    /// About the endpoint "User Summary: EXPERT QUICK PLAY",
    /// see the [API document](https://tetr.io/about/api/#usersusersummarieszenithex).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the summary of the EXPERT QUICK PLAY games of the user "RINRIN-RS".
    /// let user = client.get_user_zenith_ex("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_zenith_ex(self, user: &str) -> RspErr<ZenithExResponse> {
        let url = format!(
            "{}users/{}/summaries/zenithex",
            API_URL,
            user.to_lowercase()
        );
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the summary of the specified user's TETRA LEAGUE standing.
    ///
    /// About the endpoint "User Summary: TETRA LEAGUE",
    /// see the [API document](https://tetr.io/about/api/#usersusersummariesleague).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the summary of the TETRA LEAGUE standing of the user "RINRIN-RS".
    /// let user = client.get_user_league("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_league(self, user: &str) -> RspErr<LeagueResponse> {
        let url = format!("{}users/{}/summaries/league", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the summary of the specified user's ZEN progress.
    ///
    /// About the endpoint "User Summary: ZEN",
    /// see the [API document](https://tetr.io/about/api/#usersusersummarieszen).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get the summary of the ZEN progress of the user "RINRIN-RS".
    /// let user = client.get_user_zen("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_zen(self, user: &str) -> RspErr<ZenResponse> {
        let url = format!("{}users/{}/summaries/zen", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets all the achievements of the specified user.
    ///
    /// About the endpoint "User Summary: Achievements",
    /// see the [API document](https://tetr.io/about/api/#usersusersummariesachievements).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    /// // Get all the achievements of the user "RINRIN-RS".
    /// let user = client.get_user_achievements("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_achievements(self, user: &str) -> RspErr<AchievementsResponse> {
        let url = format!(
            "{}users/{}/summaries/achievements",
            API_URL,
            user.to_lowercase()
        );
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the user leaderboard fulfilling the search criteria.
    ///
    /// About the endpoint "User Leaderboard",
    /// see the [API document](https://tetr.io/about/api/#usersbyleaderboard).
    ///
    /// # Arguments
    ///
    /// - `leaderboard` - The user leaderboard type.
    /// - `search_criteria` - The search criteria to filter users by.
    ///
    /// # Examples
    ///
    /// Gets the TETRA LEAGUE leaderboard with the following criteria:
    ///
    /// - Upper bound is `[15200, 0, 0]`
    /// - Three entries
    /// - Filter by Japan
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::user_leaderboard::{self, LeaderboardType}
    /// };
    ///
    /// # async fn run() -> std::io::Result<()> {
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
    /// // Get the user leaderboard.
    /// let user = client.get_leaderboard(
    ///     LeaderboardType::League,
    ///     Some(criteria)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Gets the array of the historical user blobs fulfilling the search criteria.
    ///
    /// About the endpoint "Historical User Leaderboard",
    /// see the [API document](https://tetr.io/about/api/#usershistoryleaderboardseason).
    ///
    /// # Arguments
    ///
    /// - `season` - The season to look up. (e.g. `"1"`)
    /// - `search_criteria` - The search criteria to filter users by.
    ///
    /// # Examples
    ///
    /// Gets the array of the historical user blobs with the following criteria:
    ///
    /// - Season 1
    /// - Upper bound is `[15200, 0, 0]`
    /// - Three entries
    /// - Filter by Japan
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::user_leaderboard::{self, LeaderboardType}
    /// };
    ///
    /// # async fn run() -> std::io::Result<()> {
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
    /// // Get the array.
    /// let user = client.get_historical_league_leaderboard(
    ///     // Season 1
    ///     "1",
    ///     Some(criteria)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Gets the personal record leaderboard of the specified user,
    /// fulfilling the search criteria.
    ///
    /// About the endpoint "User Personal Records",
    /// see the [API document](https://tetr.io/about/api/#usersuserrecordsgamemodeleaderboard).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    /// - `gamemode` - The game mode to look up.
    /// - `leaderboard` - The personal leaderboard to look up.
    /// - `search_criteria` - The search criteria to filter records by.
    ///
    /// # Examples
    ///
    /// Gets the personal top score leaderboard of the 40 LINES records of the user "RINRIN-RS" with the following criteria:
    ///
    /// - Upper bound is `[500000, 0, 0]`
    /// - Three entries
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::record::{self, Gamemode, LeaderboardType}
    /// };
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Set the search criteria.
    /// let criteria = record::SearchCriteria::new()
    ///     // Upper bound is `[500000, 0, 0]`
    ///     .after([500000.,0.,0.])
    ///     // Three entries
    ///     .limit(3);
    ///
    /// // Get the leaderboard.
    /// let user = client.get_user_records(
    ///     // Record of the user "RINRIN-RS"
    ///     "rinrin-rs",
    ///     // 40 LINES
    ///     Gamemode::FortyLines,
    ///     // Top score leaderboard
    ///     LeaderboardType::Top,
    ///     Some(criteria)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Gets the record leaderboard fulfilling the search criteria.
    ///
    /// About the endpoint "Records Leaderboard",
    /// see the [API document](https://tetr.io/about/api/#recordsleaderboard).
    ///
    /// # Arguments
    ///
    /// - `leaderboard` - The record leaderboard ID to look up.
    /// - `search_criteria` - The search criteria to filter records by.
    ///
    /// # Examples
    ///
    /// Gets the record leaderboard with the following criteria:
    ///
    /// - Upper bound is `[500000, 0, 0]`
    /// - Three entries
    /// - Game mode: `zenith` (QUICK PLAY)
    /// - Scope: `JP` (Japan)
    /// - Revolution ID: `@2024w31`
    ///
    /// ```no_run
    /// use tetr_ch::client::{
    ///     Client,
    ///     param::record_leaderboard::{self, RecordsLeaderboardId, Scope}
    /// };
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Set the search criteria.
    /// let criteria = record_leaderboard::SearchCriteria::new()
    ///     // Upper bound is `[500000, 0, 0]`
    ///     .after([500000.,0.,0.])
    ///     // Three entries
    ///     .limit(3);
    ///
    /// // Get the record leaderboard.
    /// let user = client.get_records_leaderboard(
    ///     // Record leaderboard ID: `zenith_country_JP@2024w31`
    ///     RecordsLeaderboardId::new(
    ///         // Game mode: `zenith` (QUICK PLAY)
    ///         "zenith",
    ///         // Scope: `JP` (Japan)
    ///         Scope::Country("JP".to_string()),
    ///         // Revolution ID: `@2024w31`
    ///         Some("@2024w31")
    ///     ),
    ///    Some(criteria)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_records_leaderboard(
        self,
        leaderboard: RecordsLeaderboardId,
        search_criteria: Option<record_leaderboard::SearchCriteria>,
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

    /// Searches for a record of the specified user with the specified timestamp.
    ///
    /// Only one record is returned.
    /// It is generally not possible for a player to play the same gamemode twice in a millisecond.
    ///
    /// About the endpoint "Record Search",
    /// see the [API document](https://tetr.io/about/api/#recordsreverse).
    ///
    /// # Arguments
    ///
    /// - `user_id` - The user ID to look up.
    /// - `gamemode` - The game mode to look up.
    /// - `timestamp` - The timestamp of the record to find.
    ///
    /// # Examples
    ///
    /// Gets a record with the following criteria:
    ///
    /// - User ID: `621db46d1d638ea850be2aa0`
    /// - Gamemode: `blitz` (BLITZ)
    /// - Timestamp: `1680053762145` (`2023-03-29T01:36:02.145Z`)
    ///
    /// ```no_run
    /// use tetr_ch::client::{param::record::Gamemode, Client};
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get a record.
    /// let user = client.search_record(
    ///     // User ID: `621db46d1d638ea850be2aa0`
    ///     "621db46d1d638ea850be2aa0",
    ///     // Gamemode: `blitz` (BLITZ)
    ///     Gamemode::Blitz,
    ///     // Timestamp: `1680053762145` (`2023-03-29T01:36:02.145Z`)
    ///     1680053762145
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_record(
        self,
        user_id: &str,
        gamemode: Gamemode,
        timestamp: i64,
    ) -> RspErr<SearchedRecordResponse> {
        let query_params = [
            ("user", user_id.to_string()),
            ("gamemode", gamemode.to_param()),
            ("ts", timestamp.to_string()),
        ];
        let url = format!("{}records/reverse", API_URL);
        let res = self.client.get(url).query(&query_params).send().await;
        response(res).await
    }

    /// Gets the latest news items in any stream.
    ///
    /// About the endpoint "All Latest News",
    /// see the [API document](https://tetr.io/about/api/#newsall).
    ///
    /// # Arguments
    ///
    /// - `limit` - The amount of entries to return,
    /// between 1 and 100. 25 by default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get three latest news.
    /// let user = client.get_news_all(Some(3)).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Gets the latest news items in the specified stream.
    ///
    /// About the endpoint "Latest News",
    /// see the [API document](https://tetr.io/about/api/#newsstream).
    ///
    /// # Arguments
    ///
    /// - `stream` - The news stream to look up.
    /// - `limit` - The amount of entries to return, between 1 and 100.
    ///
    /// # Examples
    ///
    /// Gets three latest news of the user `621db46d1d638ea850be2aa0`.
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, param::news_stream::NewsStream};
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the latest news.
    /// let user = client.get_news_latest(
    ///     // News of the user `621db46d1d638ea850be2aa0`
    ///     NewsStream::User("621db46d1d638ea850be2aa0".to_string()),
    ///     // Three news
    ///     3,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the query parameter `limit` is not between 1 and 100.
    ///
    /// ```should_panic,no_run
    /// use tetr_ch::client::{Client, param::news_stream::NewsStream};
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// let user = client.get_news_latest(
    ///     // Global news
    ///     NewsStream::Global,
    ///     // 101 news (not allowed)
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

    /// Searches for a TETR.IO user account by the social connection.
    ///
    /// About the endpoint "User Search",
    /// see the [API document](https://tetr.io/about/api/#userssearchquery).
    ///
    /// # Arguments
    ///
    /// - `social_connection` - The social connection to look up.
    ///
    /// # Examples
    ///
    /// Searches for a account by Discord ID `724976600873041940`.
    ///
    /// ```no_run
    /// use tetr_ch::client::{Client, param::search_user::SocialConnection};
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Search for a account.
    /// let user = client.search_user(
    ///     // By Discord ID `724976600873041940`
    ///     SocialConnection::Discord("724976600873041940".to_string())
    /// ).await?;
    /// # Ok(())
    /// # }
    ///
    /// # tokio_test::block_on(run());
    /// ```
    pub async fn search_user(
        self,
        social_connection: SocialConnection,
    ) -> RspErr<SearchedUserResponse> {
        let url = format!("{}users/search/{}", API_URL, social_connection.to_param());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the condensed graph of all of the specified user's records in the specified gamemode.
    ///
    /// About the endpoint "Labs Scoreflow",
    /// see the [API document](https://tetr.io/about/api/#labsscoreflowusergamemode).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    /// - `gamemode` - The game mode to look up.
    ///
    /// # Examples
    ///
    /// Gets the graph of the 40 LINES records of the user `RINRIN-RS`.
    ///
    /// ```no_run
    /// use tetr_ch::client::{param::record::Gamemode, Client};
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the graph of the records.
    /// let user = client.get_labs_scoreflow(
    ///     // Records of the user "RINRIN-RS"
    ///     "rinrin-rs",
    ///     // 40 LINES records
    ///     Gamemode::FortyLines
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Gets the condensed graph of all of the specified user's matches in TETRA LEAGUE.
    ///
    /// About the endpoint "Labs Leagueflow,
    /// see the [API document](https://tetr.io/about/api/#labsleagueflowuser).
    ///
    /// # Arguments
    ///
    /// - `user` - The username or user ID to look up.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the graph of the matches of the user `RINRIN-RS` in TETRA LEAGUE.
    /// let user = client.get_labs_leagueflow("rinrin-rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_labs_leagueflow(self, user: &str) -> RspErr<LabsLeagueflowResponse> {
        let url = format!("{}labs/leagueflow/{}", API_URL, user.to_lowercase());
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the view over all TETRA LEAGUE ranks and their metadata.
    ///
    /// About the endpoint "Labs League Ranks",
    /// see the [API document](https://tetr.io/about/api/#labsleagueranks).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the view over all TETRA LEAGUE ranks and their metadata.
    /// let user = client.get_labs_league_ranks().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_labs_league_ranks(self) -> RspErr<LabsLeagueRanksResponse> {
        let url = format!("{}labs/league_ranks", API_URL);
        let res = self.client.get(url).send().await;
        response(res).await
    }

    /// Gets the data about the specified achievement itself, its cutoffs, and its leaderboard.
    ///
    /// About the endpoint "Achievement Info",
    /// see the [API document](https://tetr.io/about/api/#achievementsk).
    ///
    /// # Arguments
    ///
    /// - `achievement_id` - The achievement ID to look up. (e.g. `"15"`)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tetr_ch::client::Client;
    ///
    /// # async fn run() -> std::io::Result<()> {
    /// let client = Client::new();
    ///
    /// // Get the data about the achievement "15".
    /// let user = client.get_achievement_info("15").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_achievement_info(
        self,
        achievement_id: &str,
    ) -> RspErr<AchievementInfoResponse> {
        let url = format!("{}achievements/{}", API_URL, achievement_id);
        let res = self.client.get(url).send().await;
        response(res).await
    }
}

pub mod error;
pub mod param;
mod response;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_new_client() {
        let _ = Client::new();
    }
}
