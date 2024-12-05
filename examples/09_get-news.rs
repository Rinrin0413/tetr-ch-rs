//! Gets the news items.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 09_get-news
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Get three latest news items in any stream.
    let _ = client.get_news_all(3).await;

    // Gets the latest news items in the global news stream.
    let _ = client
        .get_news_latest(
            // The global news stream.
            NewsStreamParam::Global,
            // Three news
            3,
        )
        .await;

    // Gets the latest news items in the specified user's news stream.
    let _ = client
        .get_news_latest(
            // The news stream of the user `621db46d1d638ea850be2aa0`
            NewsStreamParam::User("621db46d1d638ea850be2aa0".to_string()),
            3,
        )
        .await;

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/news/struct.NewsItems.html
}
