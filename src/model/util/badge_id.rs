//! A model for badge's internal IDs.

use crate::model::prelude::*;

/// A badge's internal ID.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct BadgeId(String);

impl BadgeId {
    /// Returns the badge icon URL.
    pub fn icon_url(&self) -> String {
        format!("https://tetr.io/res/badges/{}.png", self.0)
    }
}

impl AsRef<BadgeId> for BadgeId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for BadgeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
