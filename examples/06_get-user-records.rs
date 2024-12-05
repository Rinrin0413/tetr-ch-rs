//! Gets the personal record leaderboard of the specified user, fulfilling the search criteria.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 06_get-user-records
//! ```
//!
//! Want to paginate over this data?
//! Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`] to ensure data consistency.
//! For more details, see the example in `/examples/15_pagination-for-leaderboard.rs`.

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the username or user ID to get the personal records.
    let user = "rinrin-rs";

    // Set the search criteria to filter records by.
    let criteria = record::SearchCriteria::new()
        // Upper bound is `[500000, 0, 0]`
        .after([500000., 0., 0.])
        // Three entries
        .limit(3);

    // Get the records.
    let response = match client
        .get_user_records(
            user,
            // BLITZ
            record::Gamemode::Blitz,
            // Top score leaderboard
            record::LeaderboardType::Top,
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
        .for_each(|record| println!("{}", record.replay_url()));

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/user_records/struct.UserRecords.html
}
