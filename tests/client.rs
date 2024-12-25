use tetr_ch::{
    model::{
        leaderboard::{HistoricalLeaderboard, Leaderboard},
        news::NewsItems,
        records_leaderboard::RecordsLeaderboard,
        user_records::UserRecords,
    },
    prelude::*,
};

type Rsp<T> = Result<tetr_ch::model::response::Response<T>, tetr_ch::client::error::ResponseError>;

#[tokio::test]
async fn client_get_leaderboard_successes_if_valid_limit() {
    let criteria = user_leaderboard::SearchCriteria::new()
        .after([15200., 0., 0.])
        .limit(1)
        .country("jp");
    let _: Rsp<Leaderboard> = Client::new()
        .get_leaderboard(UserLeaderboardType::League, Some(criteria))
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 0.")]
async fn client_get_leaderboard_panics_if_limit_is_zero() {
    let criteria = user_leaderboard::SearchCriteria {
        limit: Some(0),
        ..Default::default()
    };
    let _: Rsp<Leaderboard> = Client::new()
        .get_leaderboard(UserLeaderboardType::League, Some(criteria))
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 101.")]
async fn client_get_leaderboard_panics_if_limit_is_101() {
    let criteria = user_leaderboard::SearchCriteria {
        limit: Some(101),
        ..Default::default()
    };
    let _: Rsp<Leaderboard> = Client::new()
        .get_leaderboard(UserLeaderboardType::League, Some(criteria))
        .await;
}

#[tokio::test]
async fn client_get_historical_league_leaderboard_successes_if_valid_limit() {
    let criteria = user_leaderboard::SearchCriteria::new()
        .after([15200., 0., 0.])
        .limit(1)
        .country("jp");
    let _: Rsp<HistoricalLeaderboard> = Client::new()
        .get_historical_league_leaderboard("1", Some(criteria))
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 0.")]
async fn client_get_historical_league_leaderboard_panics_if_limit_is_zero() {
    let criteria = user_leaderboard::SearchCriteria {
        limit: Some(0),
        ..Default::default()
    };
    let _: Rsp<HistoricalLeaderboard> = Client::new()
        .get_historical_league_leaderboard("1", Some(criteria))
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 101.")]
async fn client_get_historical_league_leaderboard_panics_if_limit_is_101() {
    let criteria = user_leaderboard::SearchCriteria {
        limit: Some(101),
        ..Default::default()
    };
    let _: Rsp<HistoricalLeaderboard> = Client::new()
        .get_historical_league_leaderboard("1", Some(criteria))
        .await;
}

#[tokio::test]
async fn client_get_user_records_successes_if_valid_limit() {
    let criteria = record::SearchCriteria::new()
        .after([500000., 0., 0.])
        .limit(1);
    let _: Rsp<UserRecords> = Client::new()
        .get_user_records(
            "rinrin-rs",
            record::Gamemode::FortyLines,
            record::LeaderboardType::Top,
            Some(criteria),
        )
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 0.")]
async fn client_get_user_records_panics_if_limit_is_zero() {
    let criteria = record::SearchCriteria {
        limit: Some(0),
        ..Default::default()
    };
    let _: Rsp<UserRecords> = Client::new()
        .get_user_records(
            "rinrin-rs",
            record::Gamemode::FortyLines,
            record::LeaderboardType::Top,
            Some(criteria),
        )
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 101.")]
async fn client_get_user_records_panics_if_limit_is_101() {
    let criteria = record::SearchCriteria {
        limit: Some(101),
        ..Default::default()
    };
    let _: Rsp<UserRecords> = Client::new()
        .get_user_records(
            "rinrin-rs",
            record::Gamemode::FortyLines,
            record::LeaderboardType::Top,
            Some(criteria),
        )
        .await;
}

#[tokio::test]
async fn client_get_records_leaderboard_successes_if_valid_limit() {
    let criteria = record_leaderboard::SearchCriteria::new()
        .after([500000., 0., 0.])
        .limit(1);
    let id =
        RecordsLeaderboardId::new("zenith", Scope::Country("JP".to_string()), Some("@2024w31"));
    let _: Rsp<RecordsLeaderboard> = Client::new()
        .get_records_leaderboard(id, Some(criteria))
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 0.")]
async fn client_get_records_leaderboard_panics_if_limit_is_zero() {
    let criteria = record_leaderboard::SearchCriteria {
        limit: Some(0),
        ..Default::default()
    };
    let id =
        RecordsLeaderboardId::new("zenith", Scope::Country("JP".to_string()), Some("@2024w31"));
    let _: Rsp<RecordsLeaderboard> = Client::new()
        .get_records_leaderboard(id, Some(criteria))
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 101.")]
async fn client_get_records_leaderboard_panics_if_limit_is_101() {
    let criteria = record_leaderboard::SearchCriteria {
        limit: Some(101),
        ..Default::default()
    };
    let id =
        RecordsLeaderboardId::new("zenith", Scope::Country("JP".to_string()), Some("@2024w31"));
    let _: Rsp<RecordsLeaderboard> = Client::new()
        .get_records_leaderboard(id, Some(criteria))
        .await;
}

#[tokio::test]
async fn client_get_news_all_successes_if_valid_limit() {
    let _: Rsp<NewsItems> = Client::new().get_news_all(1).await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 0.")]
async fn client_get_news_all_panics_if_limit_is_zero() {
    let _: Rsp<NewsItems> = Client::new().get_news_all(0).await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 101.")]
async fn client_get_news_all_panics_if_limit_is_101() {
    let _: Rsp<NewsItems> = Client::new().get_news_all(101).await;
}

#[tokio::test]
async fn client_get_news_latest_successes_if_valid_limit() {
    let _: Rsp<NewsItems> = Client::new()
        .get_news_latest(
            NewsStreamParam::User("621db46d1d638ea850be2aa0".to_string()),
            1,
        )
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 0.")]
async fn client_get_news_latest_panics_if_limit_is_zero() {
    let _: Rsp<NewsItems> = Client::new()
        .get_news_latest(
            NewsStreamParam::User("621db46d1d638ea850be2aa0".to_string()),
            0,
        )
        .await;
}

#[tokio::test]
#[should_panic(expected = "The limit must be between 1 and 100, but got 101.")]
async fn client_get_news_latest_panics_if_limit_is_101() {
    let _: Rsp<NewsItems> = Client::new()
        .get_news_latest(
            NewsStreamParam::User("621db46d1d638ea850be2aa0".to_string()),
            101,
        )
        .await;
}
