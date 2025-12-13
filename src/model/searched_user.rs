//! Models for the endpoint "User Search".
//!
//! About the endpoint "User Search",
//! see the [API document](https://tetr.io/about/api/#userssearchquery).

use crate::model::prelude::*;

/// A searched user.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserData {
    /// The user information (TETRA.IO user accounts).
    pub users: Vec<UserInfo>,
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
