//! Models for the endpoint "User Search".
//!
//! About the endpoint "User Search",
//! see the [API document](https://tetr.io/about/api/#userssearchquery).

use crate::model::prelude::*;

/// A searched user.
///
/// Only one user is contained.
/// Generally, you won't see two users with the same social linked, though,
/// as it would be against TETR.IO multiaccounting policies.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserData {
    /// The user information (TETRA.IO user account).
    pub user: Option<UserInfo>,
}

impl AsRef<UserData> for UserData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A user information (TETRA.IO user account).
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserInfo {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    pub username: String,
}

impl UserInfo {
    impl_get_user!(id);
    impl_for_username!();
}

impl AsRef<UserInfo> for UserInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}
