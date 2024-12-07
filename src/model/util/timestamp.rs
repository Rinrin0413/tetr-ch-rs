//! A model for timestamp.

use crate::{model::prelude::*, util::to_unix_ts};

/// A timestamp string.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct Timestamp(String);

impl Timestamp {
    /// Creates a new `Timestamp`.
    pub(crate) fn new(ts: String) -> Self {
        Self(ts)
    }

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
