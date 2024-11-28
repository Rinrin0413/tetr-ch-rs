//! A model for timestamp.

use crate::util::to_unix_ts;
use serde::Deserialize;
use std::fmt;

/// A timestamp string.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct Timestamp(String);

impl Timestamp {
    /// Returns the UNIX timestamp.
    ///
    /// # Panics
    ///
    /// Panics if failed to parse the given string.
    pub fn unix_ts(&self) -> i64 {
        to_unix_ts(&self.0)
    }
}

impl AsRef<Timestamp> for Timestamp {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
