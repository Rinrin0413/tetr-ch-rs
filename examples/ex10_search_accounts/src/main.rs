use tetr_ch::client::Client;

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Search a TETRA.IO account by Discord ID.
    let usr = match client.search_user("724976600873041940").await {
        Ok(u) => u,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Print the requested data.
    println!("{:?}\n", usr.data.as_ref().unwrap().user);

    // Get the user details.
    let user_details = match usr.data.as_ref().unwrap().get_user().await {
        Ok(u) => u,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };
    println!("Details: {:?}\n", user_details);

    // And get the user's records.
    let user_records = match usr.data.as_ref().unwrap().get_records().await {
        Ok(u) => u,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };
    println!("Records: {:?}\n", user_records);

    // Learn more about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/searched_user/struct.SearchedUserResponse.html
}
