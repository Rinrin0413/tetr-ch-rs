//! A model for replay IDs.

use crate::model::prelude::*;

/// A replay's shortID.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct ReplayId(String);

impl ReplayId {
    /// Returns the replay URL.
    pub fn replay_url(&self) -> String {
        format!("https://tetr.io/#R:{}", self)
    }
}

impl AsRef<ReplayId> for ReplayId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for ReplayId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
