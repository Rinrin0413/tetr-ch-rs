use tetr_ch::client::{query::LeagueLeaderboardQuery, Client};

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Set the query parameters.
    // This is default(25000TR or less, fifty entries) query.
    let query1 = LeagueLeaderboardQuery::new();

    // Get the TETRA LEAGUE leaderboard.
    let _tl_leaderboard1 = match client.get_league_leaderboard(query1).await {
        Ok(l) => l,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };
    

    // Other example queries:

    // 15200TR or less, three entries, filter by Japan.
    let _query2 = LeagueLeaderboardQuery::new()
        .after(15200.)
        .limit(3)
        .country("jp");

    // 15200TR or higher.
    // Also sort by TR ascending.
    let _query3 = LeagueLeaderboardQuery::new().before(15200.);

    // Full leaderboard.
    let _query4 = LeagueLeaderboardQuery::new().limit(0);

    // You can restore the query parameters to default as follows:
    let mut _query5 = LeagueLeaderboardQuery::new().country("us");
    _query5.init();


    // Learn about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/league_leaderboard/struct.LeagueLeaderboardResponse.html
}
