//! Models for the endpoint "User Personal Records".
//!
//! About the endpoint "User Personal Records",
//! see the [API document](https://tetr.io/about/api/#usersuserrecordsgamemodeleaderboard).

use crate::model::prelude::*;

/// An array of user personal records.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserRecords {
    /// The matched records.
    pub entries: Vec<Record>,
}

impl AsRef<UserRecords> for UserRecords {
    fn as_ref(&self) -> &Self {
        self
    }
}
