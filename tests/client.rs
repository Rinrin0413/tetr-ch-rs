use tetr_ch::client::*;

#[test]
fn get_usr_data() {
    let usr = "rinrin-rs";
    let _ = tokio_test::block_on(Client::new().get_user(usr));
}

#[test]
fn get_server_status_data() {
    let _ = tokio_test::block_on(Client::new().get_server_stats());
}

#[test]
fn get_server_activity_data() {
    let _ = tokio_test::block_on(Client::new().get_server_activity());
}

#[test]
fn get_usr_records_data() {
    let usr = "rinrin-rs";
    let _ = tokio_test::block_on(Client::new().get_user_records_old(usr));
}

#[test]
fn get_league_leaderboard_data() {
    let _ = tokio_test::block_on(
        Client::new().get_league_leaderboard(query::LeagueLeaderboardQuery::new()),
    );
}

#[test]
fn get_league_leaderboard_data_with_two_queries() {
    let _ = tokio_test::block_on(
        Client::new()
            .get_league_leaderboard(query::LeagueLeaderboardQuery::new().limit(2).before(23000.)),
    );
}

#[test]
fn get_league_leaderboard_data_with_three_queries() {
    let _ = tokio_test::block_on(
        Client::new().get_league_leaderboard(
            query::LeagueLeaderboardQuery::new()
                .limit(2)
                .country("us")
                .after(13000.),
        ),
    );
}

#[test]
fn get_full_league_leaderboard_data() {
    let _ = tokio_test::block_on(
        Client::new().get_league_leaderboard(query::LeagueLeaderboardQuery::new().limit(0)),
    );
}

#[test]
#[should_panic]
fn panic_if_invalid_limit_range_exhaustivegetting_league_leaderboard() {
    let q = query::LeagueLeaderboardQuery {
        limit: Some(query::Limit::Limit(101)),
        ..query::LeagueLeaderboardQuery::new()
    };
    let _ = tokio_test::block_on(Client::new().get_league_leaderboard(q));
}

#[test]
fn get_xp_leaderboard_data() {
    let _ =
        tokio_test::block_on(Client::new().get_xp_leaderboard(query::XPLeaderboardQuery::new()));
}

#[test]
fn get_xp_leaderboard_data_with_a_query() {
    let _ = tokio_test::block_on(
        Client::new().get_xp_leaderboard(query::XPLeaderboardQuery::new().limit(2)),
    );
}

#[test]
fn get_xp_leaderboard_data_with_two_queries() {
    let _ = tokio_test::block_on(
        Client::new().get_xp_leaderboard(query::XPLeaderboardQuery::new().limit(2).before(23000.)),
    );
}

#[test]
fn get_xp_leaderboard_data_with_three_queries() {
    let _ = tokio_test::block_on(
        Client::new().get_xp_leaderboard(
            query::XPLeaderboardQuery::new()
                .limit(2)
                .country("us")
                .after(13000.),
        ),
    );
}

#[test]
#[should_panic]
fn panic_if_invalid_limit_range_exhaustive_in_getting_xp_leaderboard() {
    let q = query::XPLeaderboardQuery {
        limit: Some(std::num::NonZeroU8::new(101).unwrap()),
        ..query::XPLeaderboardQuery::new()
    };
    let _ = tokio_test::block_on(Client::new().get_xp_leaderboard(q));
}

#[test]
fn get_stream_data() {
    let _ = tokio_test::block_on(Client::new().get_stream(
        stream::StreamType::FortyLines,
        stream::StreamContext::Global,
        None,
    ));
}

#[test]
fn get_user_40l_best_stream_data() {
    let _ = tokio_test::block_on(Client::new().get_stream(
        stream::StreamType::FortyLines,
        stream::StreamContext::UserBest,
        Some("621db46d1d638ea850be2aa0"),
    ));
}

#[test]
fn get_latest_any_news_data() {
    let _ = tokio_test::block_on(Client::new().get_latest_news(stream::NewsSubject::Any, 3));
}

#[test]
fn get_latest_global_news_data() {
    let _ = tokio_test::block_on(Client::new().get_latest_news(stream::NewsSubject::Global, 3));
}

#[test]
fn get_latest_user_scale_news_data() {
    let _ = tokio_test::block_on(Client::new().get_latest_news(
        stream::NewsSubject::User("621db46d1d638ea850be2aa0".to_string()),
        3,
    ));
}

#[test]
#[should_panic]
fn panic_if_invalid_limit_range_in_getting_latest_news() {
    let _ = tokio_test::block_on(Client::new().get_latest_news(stream::NewsSubject::Any, 0));
}
