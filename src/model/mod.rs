//! Easy-to-use models of the various objects received from the API.
//!
//! You can import the models that are enums and tuple structs from the [`model::prelude`](prelude) module.
//!
//! ```
//! use tetr_ch::model::prelude::*;
//! ```

#[macro_use]
mod macros;

pub mod achievement_info;
pub mod cache;
pub mod error_response;
pub mod labs;
pub mod leaderboard;
pub mod news;
pub mod records_leaderboard;
pub mod response;
pub mod searched_record;
pub mod searched_user;
pub mod server_activity;
pub mod server_stats;
pub mod summary;
pub mod user;
pub mod user_records;
pub mod util;

/// A prelude for the models.
///
/// # Example
///
/// ```
/// use tetr_ch::model::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        cache::Status as CacheStatus,
        news::NewsData,
        util::{
            Achievement, BadgeId, Gamemode, NewsStream as NewsStreamModel, Rank,
            RecordLeaderboard as RecordLeaderboardModel, ReplayId, Role, Timestamp, UserId,
        },
    };

    pub(crate) use super::{
        cache::CacheData, error_response::ErrorResponse, summary::record::Record,
    };
    pub(crate) use crate::client::param::pagination::Prisecter;
    pub(crate) use serde::Deserialize;
    pub(crate) use std::fmt;
}
