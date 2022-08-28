use tetr_ch::client::Client;

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Get the server statics.
    let stats = match client.get_server_stats().await {
        Ok(s) => s,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Print the server statics (some examples).
    println!("Total users: {}", stats.data.as_ref().unwrap().usercount);
    println!("Registered users: {}", stats.registered_players());
    println!("Anonymous users: {}", stats.data.as_ref().unwrap().anoncount);
    println!("Ranked users: {}", stats.data.as_ref().unwrap().rankedcount);

    // Learn more about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/server_stats/struct.ServerStats.html
}
