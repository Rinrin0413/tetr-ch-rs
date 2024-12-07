//! Gets the detailed information about the specified user.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 01_get-user-info
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Set the username or user ID to get the information.
    let user = "rinrin-rs";

    // Get the information.
    let response = match client.get_user(user).await {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    // Check if there is an error.
    // An error "No such user!" will be returned here if the user does not exist.
    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap();
    println!("Name: {}", data.username);
    println!("ID: {}", data.id);
    println!("XP: {}", data.xp);
    println!("Level: {}", data.level());
    println!("Role: {}", data.role);
    println!("Country: {:?}", data.country);
    println!("Avatar URL: {}", data.avatar_url());
    println!("Discord: {:?}", data.connections.discord);

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/user/struct.User.html
}
