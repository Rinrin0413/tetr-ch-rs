//! Gets some summaries of the specified user.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 03_get-user-summaries
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the username or user ID to get the summaries.
    let user = "rinrin-rs";

    // Get the summary of the user's 40 LINES games.
    let _ = client.get_user_40l(user).await;

    // Get the summary of the user's BLITZ games.
    let _ = client.get_user_blitz(user).await;

    // Get the summary of the user's QUICK PLAY games.
    let _ = client.get_user_zenith(user).await;

    // Get the summary of the user's EXPERT QUICK PLAY games.
    let _ = client.get_user_zenith_ex(user).await;

    // Get the summary of the user's TETRA LEAGUE standing.
    let _ = client.get_user_league(user).await;

    // Get the summary of the user's ZEN progress.
    let _ = client.get_user_zen(user).await;

    // Get all the achievements of the user.
    let _ = client.get_user_achievements(user).await;

    // Get all the summaries of the user.
    //
    // WARNING: Consider whether you really need to use this method.
    //          If you only collect data for one or two game modes,
    //          use the methods for the individual summaries instead.
    let _ = client.get_user_all_summaries(user).await;

    // For more information about the data structures, see:
    // - 40 LINES: https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/forty_lines/struct.FortyLines.html
    // - BLITZ: https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/blitz/struct.Blitz.html
    // - QUICK PLAY, EXPERT QUICK PLAY: https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/zenith/struct.Zenith.html
    // - TETRA LEAGUE: https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/league/struct.LeagueData.html
    // - ZEN: https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/zen/struct.Zen.html
    // - Achievements: https://docs.rs/tetr_ch/latest/tetr_ch/model/util/achievement/struct.Achievement.html
    // - All summaries: https://docs.rs/tetr_ch/latest/tetr_ch/model/summary/struct.AllSummaries.html
}
