//! A model for user IDs,

use serde::Deserialize;
use std::fmt;

/// A user's internal ID.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct UserId(String);

impl UserId {
    impl_get_user!();

    /// Returns the user's internal ID.
    #[deprecated(since = "0.6.0", note = "please use the `.to_string()` method instead")]
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl AsRef<UserId> for UserId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
