//! Pagination for the leaderboards.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 15_pagination-for-leaderboard
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    // `Client` struct that created by `Client::new()` does not have `X-Session-ID` header.
    // So use `Client::with_session_id()` to create a new `Client`,
    // if you are often re-requesting the same datasets.
    // This not only assures the data you receive is consistent,
    // it also helps reduce database calls on their(TETRA CHANNEL API) side.

    // Create a new client with an automatically generated session ID.
    // Or, if you have a session ID that wanna use, pass it as an argument.
    let client = Client::with_session_id(None).unwrap();

    // You can get the session ID by calling the `session_id` method.
    println!("Generated session ID: {}", client.session_id().unwrap());

    // Get the top 1 ~ 50 on the TETRA LEAGUE leaderboard.
    let users = client
        .get_leaderboard(
            UserLeaderboardType::League,
            // 50 entries.
            Some(user_leaderboard::SearchCriteria::new().limit(50)),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .entries;

    println!("№1 {}", users[0].username.to_uppercase());
    // ...
    println!("№50 {}", users[49].username.to_uppercase());

    // A prisecter is consisting of three floats. It allows you to continue paginating.
    let prisecter = &users[49].prisecter;

    // Get the top 51 ~ 100 on the TETRA LEAGUE leaderboard using the prisecter.
    let users = client
        .get_leaderboard(
            UserLeaderboardType::League,
            Some(
                user_leaderboard::SearchCriteria::new()
                    .limit(100)
                    // Set the upper bound with the prisecter.
                    .after(prisecter.to_array()),
            ),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .entries;

    println!("№51 {}", users[0].username.to_uppercase());
    // ...
    println!("№100 {}", users[49].username.to_uppercase());
}
