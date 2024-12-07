//! Searches for a record of the specified user with the specified timestamp.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 08_search-for-record
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the user ID to search for.
    let user_id = "621db46d1d638ea850be2aa0";

    // Search for the record.
    let response = match client
        .search_record(
            user_id,
            // Gamemode: `blitz` (BLITZ)
            RecordGamemode::Blitz,
            // Timestamp: `1680053762145` (`2023-03-29T01:36:02.145Z`)
            1680053762145,
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
    println!("Replay URL: {}", data.replay_url());

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/record/struct.Record.html
}
