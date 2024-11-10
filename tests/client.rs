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
fn get_latest_global_news_data() {
    let _ = tokio_test::block_on(Client::new().get_news_latest(stream::NewsStream::Global, 3));
}

#[test]
fn get_latest_user_scale_news_data() {
    let _ = tokio_test::block_on(Client::new().get_news_latest(
        stream::NewsStream::User("621db46d1d638ea850be2aa0".to_string()),
        3,
    ));
}

#[test]
#[should_panic]
fn panic_if_invalid_limit_range_in_getting_latest_news() {
    let _ = tokio_test::block_on(Client::new().get_news_latest(stream::NewsStream::Global, 0));
}
