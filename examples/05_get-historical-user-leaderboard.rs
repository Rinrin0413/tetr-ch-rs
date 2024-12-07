//! Gets the array of the historical user blobs fulfilling the search criteria.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 05_get-historical-user-leaderboard
//! ```
//!
//! Want to paginate over this data?
//! Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`] to ensure data consistency.
//! For more details, see the example in `/examples/15_pagination-for-leaderboard.rs`.

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the season to get the array.
    // "1" is the Season 1.
    let season = "1";

    // Set the search criteria to filter users by.
    let criteria = user_leaderboard::SearchCriteria::new()
        // Upper bound is `[24997, 0, 0]`
        .after([24997., 0., 0.])
        // Five entries
        .limit(5)
        // Filter by Japan
        .country("jp");

    // Get the array.
    let response = match client
        .get_historical_league_leaderboard(season, Some(criteria))
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

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/leaderboard/struct.HistoricalLeaderboard.html
}
