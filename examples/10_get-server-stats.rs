//! Gets some statistics about the TETR.IO.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 10_get-server-stats
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Get the statistics.
    let response = match client.get_server_stats().await {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap();
    println!("{:#?}", data);

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/server_stats/struct.ServerStats.html
}
