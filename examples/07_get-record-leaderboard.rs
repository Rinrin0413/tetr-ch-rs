//! Gets the record leaderboard fulfilling the search criteria.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 07_get-record-leaderboard
//! ```
//!
//! Want to paginate over this data?
//! Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`] to ensure data consistency.
//! For more details, see the example in `/examples/15_pagination-for-leaderboard.rs`.

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the record leaderboard ID to look up.
    // Record leaderboard ID: `zenith_country_JP@2024w31`
    let leaderboard_id = RecordsLeaderboardId::new(
        // Game mode: `zenith` (QUICK PLAY)
        "zenith",
        // Scope: `JP` (Japan)
        Scope::Country("JP".to_string()),
        // Revolution ID: `@2024w31`
        Some("@2024w31"),
    );

    // Set the search criteria to filter records by.
    let criteria = record_leaderboard::SearchCriteria::new()
        // Upper bound is `[500000, 0, 0]`
        .after([500000., 0., 0.])
        // Three entries
        .limit(3);

    // Get the information.
    let response = match client
        .get_records_leaderboard(leaderboard_id, Some(criteria))
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
        .for_each(|entry| println!("{}", entry.replay_url()));

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/records_leaderboard/struct.RecordsLeaderboard.html
}
