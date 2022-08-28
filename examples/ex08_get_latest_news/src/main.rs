use tetr_ch::client::{Client, stream::NewsSubject};

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Get the latest news.
    let _latest_news1 = match client.get_latest_news(
        // News of user `621db46d1d638ea850be2aa0`.
        NewsSubject::User("621db46d1d638ea850be2aa0".to_string()),
        // Three news.
        3,
    ).await {
        Ok(n) => n,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Some examples for other arguments setups:

    // Latest news of all users.
    let _latest_news2 = match Client::new().get_latest_news(
        NewsSubject::Any,
        3,
    ).await {
        Ok(n) => n,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Global latest news.
    let _latest_news3 = match Client::new().get_latest_news(
        NewsSubject::Global,
        3,
    ).await {
        Ok(n) => n,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };


    // Learn about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/user/struct.UserResponse.html
}
