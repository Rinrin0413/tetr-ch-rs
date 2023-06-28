use tetr_ch::client::{
    Client,
    stream::{StreamType, StreamContext}
};

#[tokio::main]
async fn main() {
    // Create a new client.
    let client = Client::new();

    // Get a stream.
    let _stream1 = match client
        .get_stream(
            // 40 LINES
            StreamType::FortyLines,
            // User's best
            StreamContext::UserBest,
            // User ID
            Some("621db46d1d638ea850be2aa0"),
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Some examples for other arguments setups:

    // The global 40 LINES stream.
    let _stream2 = match Client::new()
        .get_stream(
            StreamType::FortyLines,
            StreamContext::Global,
            None
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // User `5e32fc85ab319c2ab1beb07c`'s best BLITZ records stream.
    let _stream3 = match Client::new()
        .get_stream(
            StreamType::Blitz,
            StreamContext::UserBest,
            Some("5e32fc85ab319c2ab1beb07c")
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // User `5e32fc85ab319c2ab1beb07c`'s recent mixed records stream.
    let _stream4 = match Client::new()
        .get_stream(
            StreamType::Any,
            StreamContext::UserRecent,
            Some("5e32fc85ab319c2ab1beb07c")
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // User `5e32fc85ab319c2ab1beb07c`'s recent TETRA LEAGUE records stream.
    let _stream5 = match Client::new()
        .get_stream(
            StreamType::League,
            StreamContext::UserRecent,
            Some("5e32fc85ab319c2ab1beb07c")
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            panic!("Error: {}\n", err.to_string());
        }
    };

    // Learn about what we can get from following docs:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/stream/struct.StreamResponse.html
}
