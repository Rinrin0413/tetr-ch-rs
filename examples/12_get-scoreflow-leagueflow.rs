//! Gets the condensed graph of all of the specified user’s records or TETRA LEAGUE matches.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 12_get-scoreflow-leagueflow
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the username or user ID to get the graph.
    let user = "rinrin-rs";

    // Get the condensed graph of the user's 40 LINES records.
    let _ = client
        .get_labs_scoreflow(
            user,
            // 40 LINES
            RecordGamemode::FortyLines,
        )
        .await;

    // Get the condensed graph of the user's matches in TETRA LEAGUE.
    let _ = client.get_labs_leagueflow(user).await;

    // For more information about the data structures, see:
    // - Scoreflow: https://docs.rs/tetr_ch/latest/tetr_ch/model/labs/scoreflow/struct.LabsScoreflow.html
    // - Leagueflow: https://docs.rs/tetr_ch/latest/tetr_ch/model/labs/leagueflow/struct.LabsLeagueflow.html
}
