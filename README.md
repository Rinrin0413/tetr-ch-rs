# tetr-ch-rs ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/tetr-ch-rs?color=007722&label=Latest%20release&style=flat-square) [![Codecov](https://img.shields.io/codecov/c/github/Rinrin0413/tetr-ch-rs?color=%23ff0077&logo=Codecov&style=flat-square)](https://app.codecov.io/gh/Rinrin0413/tetr-ch-rs)

tetr-ch-rs is a Rust library for the [TETRA CHANNEL API](https://tetr.io/about/api/).

You can get the following from the TETRA CHANNEL API with this library:

- Public details for each user.
- Some single player records.
- Some statistics about the [TETR.IO](https://tetr.io).
- Graph of user activity.
- Some streams.
- TETRA LEAGUE Leaderboard.
- XP Leaderboard.
- The latest news.

Also you can search for [TETR.IO](https://tetr.io) accounts by Discord account.

But TETRA CHANNEL API is alpha version.
So this library may not work properly in the future:(

**\* This library is NOT official.**

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
tetr_ch = "0.3.3"
```

# Examples

The following example is a template for getting user details.

```rust
use tetr_ch::client::Client;

#[tokio::main]
async fn main() {
    // Set the user (name or id).
    let user = "rinrin-rs";

    // Create a new client.
    let client = Client::new();

    // Get the user details.
    // And send the requested data or error message.
    match client.get_user(user).await {
        Ok(u) => {
            println!("{:?}\n", u);
        }
        Err(err) => {
            eprintln!("Error: {}\n", err.to_string());
        }
    }
}
```

See [full examples](./examples/).

And see the [docs](https://docs.rs/tetr_ch).

[![GPL-3.0](https://img.shields.io/github/license/Rinrin0413/tetr-ch-rs?color=%23BD0102&style=for-the-badge)](./LICENSE.md)
