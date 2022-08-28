use tetr_ch::client::Client;

#[tokio::main]
async fn main() {
    // Set the user (name or id).
    let user = "rinrin-rs";

    // Create a new client.
    let client = Client::new();

    // Get the user's (best) records.
    let records = match client.get_user_records(user).await {
        Ok(r) => r,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Print the user's (best) records (some examples).
    println!(
        "40 LINES result: {:.3}s",
        records
            .data
            .as_ref()
            .unwrap()
            .records
            .forty_lines
            .record
            .as_ref()
            .unwrap()
            .endcontext
            .final_time
            .unwrap()
            / 1000.
    );
    println!(
        "40 LINES PPS(Pieces Per Second): {}",
        records.forty_lines_pps()
    );
    println!(
        "BLITZ result: {}",
        records
            .data
            .as_ref()
            .unwrap()
            .records
            .blitz
            .record
            .as_ref()
            .unwrap()
            .endcontext
            .score
            .unwrap()
    );
    println!("BLITZ SPP(Score Per Piece): {}", records.blitz_spp());
    println!("ZEN level: {}", records.data.as_ref().unwrap().zen.level);

    // Learn more about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/user/struct.UserRecordsResponse.html
}
