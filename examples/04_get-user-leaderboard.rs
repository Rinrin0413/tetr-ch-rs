//! Gets the user leaderboard fulfilling the search criteria.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 04_get-user-leaderboard
//! ```
//!
//! Want to paginate over this data?
//! Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`] to ensure data consistency.
//! For more details, see the example in `/examples/15_pagination-for-leaderboard.rs`.

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the search criteria to filter users by.
    let criteria = user_leaderboard::SearchCriteria::new()
        // Upper bound is `[15200, 0, 0]`
        .after([15200., 0., 0.])
        // Three entries
        .limit(3)
        // Filter by Japan
        .country("jp");

    // Get the leaderboard.
    let response = match client
        .get_leaderboard(
            // A TETRA LEAGUE leaderboard.
            UserLeaderboardType::League,
            Some(criteria),
        )
        .await
    {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap();
    data.entries
        .iter()
        .for_each(|entry| println!("{}", entry.username.to_uppercase()));

    // Some examples of search criteria:

    // Default search criteria.
    // No bounds, no limit, no country filter.
    let _ = user_leaderboard::SearchCriteria::new();

    // Lower bound is `[15200, 0, 0]`.
    // The leaderboard order is reversed.
    let _ = user_leaderboard::SearchCriteria::new().before([15200., 0., 0.]);

    // Use the `init` method to initialize the search criteria to default.
    let mut criteria = user_leaderboard::SearchCriteria::new().country("us");
    criteria.init();

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/leaderboard/struct.Leaderboard.html
}
