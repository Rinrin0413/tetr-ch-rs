use tetr_ch::client::Client;

#[tokio::main]
async fn main() {
    // Set the user (name or id).
    let user = "rinrin-rs";

    // Create a new client.
    let client = Client::new();

    // Get the user details.
    let usr = match client.get_user(user).await {
        Ok(u) => u,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Print the user details (some examples).
    println!("Level: {}", usr.level());
    println!("XP: {}", usr.data.as_ref().unwrap().user.xp);
    println!("Role: {}", usr.data.as_ref().unwrap().user.role);
    println!("Rank: {}", usr.data.as_ref().unwrap().user.league.rank);
    println!("Reached {:.2}%", usr.rank_progress().unwrap());
    println!("№{}", usr.data.as_ref().unwrap().user.league.standing);
    println!("№{} (local)", usr.data.as_ref().unwrap().user.league.standing_local);
    println!("Badges count: {}", usr.badges_count());
    println!("Icon URL: {}", usr.face());
    println!("Country: {:?}", usr.data.as_ref().unwrap().user.country);

    // Learn more about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/user/struct.UserResponse.html
}
