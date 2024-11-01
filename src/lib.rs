//! ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/tetr-ch-rs?color=007722&label=Latest%20release&style=flat-square) [![Codecov](https://img.shields.io/codecov/c/github/Rinrin0413/tetr-ch-rs?color=%23ff0077&logo=Codecov&style=flat-square)](https://app.codecov.io/gh/Rinrin0413/tetr-ch-rs)
//!
//! tetr-ch-rs is a library for the [TETRA CHANNEL API](https://tetr.io/about/api/).
//!
//! You can get the following from the TETRA CHANNEL API with this library:
//!
//! - Public details for each user.
//! - Some single player records.
//! - Some statistics about the [TETR.IO](https://tetr.io).
//! - Graph of user activity.
//! - Some streams.
//! - TETRA LEAGUE Leaderboard.
//! - XP Leaderboard.
//! - The latest news.
//!
//! Also you can search for [TETR.IO](https://tetr.io) accounts by Discord account.
//!
//! But the TETRA CHANNEL API is in beta
//! so this library may not work properly in the future :(
//!
//! **\* This library is NOT official.**
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
//! use tetr_ch::client::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Set the user (name or id).
//!     let user = "rinrin-rs";
//!
//!     // Create a new client.
//!     let client = Client::new();
//!
//!     // Get the user details.
//!     // And send the requested data or error message.
//!     match client.get_user(user).await {
//!         Ok(u) => {
//!             println!("{:?}\n", u);
//!         }
//!         Err(err) => {
//!             eprintln!("Error: {}\n", err.to_string());
//!         }
//!     }
//! }
//! ```
//!
//! See [full examples](https://github.com/Rinrin0413/tetr-ch-rs/tree/master/examples/).
//!
//! [![MIT](https://img.shields.io/github/license/Rinrin0413/tetr-ch-rs?color=%23A11D32&style=for-the-badge)](https://docs.rs/crate/tetr_ch/latest/source/LICENSE)

pub mod client;
pub mod constants;
pub mod error;
pub mod model;

pub(crate) mod util;
