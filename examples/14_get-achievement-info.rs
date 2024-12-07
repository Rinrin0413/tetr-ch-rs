//! Gets the data about the specified achievement itself, its cutoffs, and its leaderboard.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 14_get-achievement-info
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the achievement ID to get data. (e.g. "15")
    let achievement_id = "15";

    // Get the data.
    let response = match client.get_achievement_info(achievement_id).await {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap();
    println!("{:#?}", data.achievement);
    println!("\n{} entries\n", data.leaderboard.len());
    println!("Cutoffs: {:#?}", data.cutoffs);

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/achievement_info/struct.AchievementInfo.html
}
