//! Searches for a TETR.IO user account by the social connection.
//!
//! Run the following Cargo command to run this example:
//!
//! ```bash
//! cargo run --example 02_search-for-account
//! ```

use tetr_ch::prelude::*;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Set the social connection to search for.
    // The API document says searching for the other social links will be added in the near future.
    let social_connection = SocialConnection::Discord("724976600873041940".to_string());

    // Search for the account.
    let response = match client.search_user(social_connection).await {
        Ok(res) => res,
        Err(err) => panic!("Response error: {}\n", err),
    };

    if let Some(err) = response.error {
        panic!("Error: {}\n", err.msg.expect("no error message"));
    }

    let data = response.data.unwrap().user.unwrap();
    println!("Name: {}", data.username);
    println!("ID: {}", data.id);

    // For more information about the data structure, see:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/searched_user/struct.UserData.html
}
