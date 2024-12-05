//! Gets the view over all TETRA LEAGUE ranks and their metadata.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 13_get-all-rank-metadata
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Get the metadata.
    let response = match client.get_labs_league_ranks().await {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap();
    println!("{:?}", data);

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/labs/league_ranks/struct.LabsLeagueRanks.html
}
