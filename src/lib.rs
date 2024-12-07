//! ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/tetr-ch-rs?color=007722&label=Latest%20release&style=flat-square) [![Codecov](https://img.shields.io/codecov/c/github/Rinrin0413/tetr-ch-rs?color=%23ff0077&logo=Codecov&style=flat-square)](https://app.codecov.io/gh/Rinrin0413/tetr-ch-rs)
//!
//! A Rust wrapper for the [TETRA CHANNEL API](https://tetr.io/about/api).
//!
//! You can get the following data by using this library:
//!
//! - Detailed user information
//! - User's summaries
//!    - 40 LINES
//!    - BLITZ
//!    - QUICK PLAY
//!    - EXPERT QUICK PLAY
//!    - TETRA LEAGUE
//!    - ZEN
//!    - Achievements
//! - User leaderboards
//! - User records
//! - Record leaderboards
//! - Rank metadata
//! - and more...
//!
//! Also you can:
//!
//! - Search for TETR.IO account by social connections.
//! - Search for record by user ID and timestamp.
//!
//! # Warning
//!
//! This library is not an officially provided wrapper.
//!
//! TETR.IO is an ongoing project in continuous development.
//! The TETRA CHANNEL API may change with or without notice between updates.
//! So this wrapper may be outdated in the future.
//!
//! <details>
//! <summary>And read the TETRA CHANNEL API rules before using this library:</summary>
//! <div>
//!
//! > Usage of the TETRA CHANNEL API does not require an account or bot account.
//! > Please do note that requests are logged. Some simple rules:
//! >
//! > - **Do not flood the API with requests.** This should be obvious, but just to be sure.
//! >   Please keep the amount of requests at a moderate rate - once a second should be fine for most cases, short bursts are OK.
//! >   Please consider other users!
//! > - **Honor caching data.** If a response indicates its cache will expire after 10 minutes,
//! >   please do not rerequest the data during that time, as the data should not change in that time,
//! >   assuming you are sending an `X-Session-ID` header.
//! > - **Send an `X-Session-ID` header** if you are often rerequesting the same datasets.
//! >   This not only assures the data you receive is consistent, it also helps reduce database calls on our side.
//! > - **Don't use a `X-Session-ID` header for requests that are not related.** That way, load balancing can function as expected.
//! > - **Do not use the API in ways that break the TETR.IO [Terms of Service](https://tetr.io/about/terms/).** Should be obvious.
//! >
//! > ― [https://tetr.io/about/api](https://tetr.io/about/api)
//!
//! </div>
//! </details>
//!
//! # Installation
//!
//! Run the following Cargo command in your project directory:
//!
//! ```bash
//! cargo add tetr_ch
//! ```
//!
//! # Examples
//!
//! The following example is a template for getting user details.
//!
//! ```ignore
//! use tetr_ch::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a new client.
//!     let client = Client::new();
//!
//!     // Set the username or user ID to get the information.
//!     let user = "rinrin-rs";
//!
//!     // Get the data.
//!     let response = match client.get_user(user).await {
//!         Ok(res) => res,
//!         Err(err) => panic!("Response error: {}\n", err),
//!     };
//!
//!     // Check if there is an error.
//!     // An error "No such user!" will be returned here if the user does not exist.
//!     if let Some(err) = response.error {
//!         panic!("Error: {}\n", err.msg.expect("no error message"));
//!     }
//!
//!     let data = response.data.unwrap();
//!     println!("Name: {}", data.username);
//!     println!("ID: {}", data.id);
//!     println!("XP: {}", data.xp);
//!     println!("Level: {}", data.level());
//!     println!("Avatar URL: {}", data.avatar_url());
//! }
//! ```
//!
//! All the examples can be found in the [`examples`](https://github.com/Rinrin0413/tetr-ch-rs/tree/master/examples) directory.
//!
//! [![MIT](https://img.shields.io/github/license/Rinrin0413/tetr-ch-rs?color=%23A11D32&style=for-the-badge)](https://docs.rs/crate/tetr_ch/latest/source/LICENSE)

pub mod client;
pub mod constants;
pub mod model;
pub mod util;

/// A prelude for the tetr-ch-rs.
///
/// # Example
///
/// ```
/// use tetr_ch::prelude::*;
/// ```
pub mod prelude {
    pub use crate::client::{
        param::{
            news_stream::NewsStream as NewsStreamParam,
            record::Gamemode as RecordGamemode,
            record_leaderboard::{RecordsLeaderboardId, Scope},
            search_user::SocialConnection,
            user_leaderboard::LeaderboardType as UserLeaderboardType,
            *,
        },
        Client,
    };
}
