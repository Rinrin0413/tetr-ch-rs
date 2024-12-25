//! Models for the endpoint "User Summary: ZEN".
//!
//! About the endpoint "User Summary: ZEN",
//! see the [API document](https://tetr.io/about/api/#usersusersummarieszen).

use crate::model::prelude::*;

/// A struct that describes a summary of a user's ZEN progress.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Zen {
    /// The user's ZEN level.
    pub level: u32,
    /// The user's ZEN score.
    pub score: f64,
}

impl AsRef<Zen> for Zen {
    fn as_ref(&self) -> &Self {
        self
    }
}
