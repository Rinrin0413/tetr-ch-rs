//! ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/tetr-ch-rs?color=007722&label=Latest%20release&style=flat-square)
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
//! But TETRA CHANNEL API is alpha version.
//! So this library may not work properly in the future:(
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! tetr_ch = "0.1.0"
//! ```
//!
//! # Examples
//!
//! The following example is a template for getting user details.
//!
//! ```rust
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
//! See [full examples](./examples/).
//!
//! [![GPL-3.0](https://img.shields.io/github/license/Rinrin0413/tetr-ch-rs?color=%23BD0102&style=for-the-badge)](./LICENSE.md)

#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/Rinrin0413/tetr-ch-rs/dev/assets/tetr-ch-rs.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/Rinrin0413/tetr-ch-rs/dev/assets/tetr-ch-rs.png"
)]

pub mod client;
pub mod constants;
pub mod error;
pub mod model;

pub(crate) mod util;
