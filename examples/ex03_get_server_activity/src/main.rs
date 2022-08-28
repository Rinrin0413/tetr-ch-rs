use tetr_ch::client::Client;

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Get the server activity.
    let activity = match client.get_server_activity().await {
        Ok(a) => a,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Print the all server activity and activity length.
    println!("{:?}", activity.data.as_ref().unwrap().activity);
    println!("\nArray length: {}", activity.data.as_ref().unwrap().activity.len());

    // Learn more about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/server_activity/struct.ServerActivity.html
}
