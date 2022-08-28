use tetr_ch::client::{query::XPLeaderboardQuery, Client};

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Set the query parameters.
    // This is default(descending, fifty entries) query.
    let query1 = XPLeaderboardQuery::new();

    // Get the XP leaderboard.
    let _xp_leaderboard1 = match client.get_xp_leaderboard(query1).await {
        Ok(l) => l,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };
    

    // Other example queries:

    // 50,000,000,000,000xp or less, thirty entries, filter by Japan.
    let _query2 = XPLeaderboardQuery::new()
        .after(50_000_000_000_000.)
        .limit(3)
        .country("jp");

    // 50,000,000,000,000xp or higher.
    // Also sort by XP ascending.
    let _query3 = XPLeaderboardQuery::new()
        .before(50_000_000_000_000.);

    // You can restore the query parameters to default as follows:
    let mut _query4 = XPLeaderboardQuery::new().country("us");
    _query4.init();


    // Learn about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/xp_leaderboard/struct.XPLeaderboardResponse.html
}
