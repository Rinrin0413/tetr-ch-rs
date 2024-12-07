//! Gets the array of the user activity over the last 2 days.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 11_get-server-activity
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Get the array.
    let response = match client.get_server_activity().await {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap();
    println!("Peak: {:?}", data.peak());
    println!("Trough: {:?}", data.trough());
    println!("Average: {:?}", data.average());

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/server_activity/struct.ServerActivity.html
}
