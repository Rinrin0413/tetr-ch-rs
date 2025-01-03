//! Macros to implement methods for the models.
//!
//! # Examples
//!
//! ```ignore
//! pub struct User {
//!     pub username: String,
//!     // ...
//! }
//!
//! impl User {
//!     impl_for_username!();
//! }
//! ```

/// Includes a macro to implement the method `get_user`.
#[macro_use]
mod get_user;
/// Includes a macro to implement the methods for `role` field.
#[macro_use]
mod role;
/// Includes macros to implement the methods for `*_at` fields.
#[macro_use]
mod some_at;

/// A macro to implement the method for `username` field.
///
/// # Methods
///
/// ```ignore
/// pub fn profile_url(&self) -> String
/// ```
///
/// # Dependencies
///
/// - `username: String` field
///
/// Go to [String]
macro_rules! impl_for_username {
    () => {
        /// Returns the user's TETRA CHANNEL profile URL.
        pub fn profile_url(&self) -> String {
            format!("https://ch.tetr.io/u/{}", self.username)
        }
    };
}

/// A macro to implement the method for `country` field.
///
/// # Methods
///
/// ```ignore
/// pub fn national_flag_url(&self) -> Option<String>
/// ```
///
/// # Dependencies
///
/// - `country: Option<String>` field
///
/// Go to [Option] | [String]
macro_rules! impl_for_country {
    () => {
        /// Returns the national flag URL of the user's country.
        ///
        /// If the user's country is hidden or unknown, `None` is returned.
        pub fn national_flag_url(&self) -> Option<String> {
            self.country
                .as_ref()
                .map(|cc| format!("https://tetr.io/res/flags/{}.png", cc.to_lowercase()))
        }
    };
}

/// A macro to implement the method for `xp` field.
///
/// # Methods
///
/// ```ignore
/// pub fn level(&self) -> u32
/// ```
///
/// # Dependencies
///
/// - `xp: f64` field
macro_rules! impl_for_xp {
    () => {
        /// Returns the level of the user.
        pub fn level(&self) -> u32 {
            crate::util::xp_to_level(self.xp)
        }
    };
}

/// A macro to implement the method for `avatar_revision` field.
///
/// # Methods
///
/// ```ignore
/// pub fn avatar_url(&self) -> String
/// ```
///
/// # Dependencies
///
/// - `avatar_revision: Option<u64>` field
///
/// Go to [Option]
macro_rules! impl_for_avatar_revision {
    () => {
        /// Returns the user's avatar URL.
        ///
        /// If the user does not have an avatar, the anonymous's avatar URL is returned.
        pub fn avatar_url(&self) -> String {
            let default = "https://tetr.io/res/avatar.png".to_string();
            if let Some(ar) = self.avatar_revision {
                if ar == 0 {
                    return default;
                }
                format!(
                    "https://tetr.io/user-content/avatars/{}.jpg?rv={}",
                    self.id, ar
                )
            } else {
                default
            }
        }
    };
}

/// A macro to implement the method for `banner_revision` field.
///
/// # Methods
///
/// ```ignore
/// pub fn banner_url(&self) -> Option<String>
/// ```
///
/// # Dependencies
///
/// - `banner_revision: Option<u64>` field
macro_rules! impl_for_banner_revision {
    () => {
        /// Returns the user's banner URL.
        ///
        /// If the user does not have a banner, `None` is returned.
        ///
        /// ***Ignore the returned value if the user is not a supporter.
        /// Because even if the user is not currently a supporter,
        /// `Some<String>` may be returned if the banner was once set.**
        pub fn banner_url(&self) -> Option<String> {
            if let Some(br) = self.banner_revision {
                if br == 0 {
                    return None;
                }
                Some(format!(
                    "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                    self.id, br
                ))
            } else {
                None
            }
        }
    };
}

/// A macro to implement the method for `id: BadgeId` field.
///
/// # Methods
///
/// ```ignore
/// pub fn icon_url(&self) -> String
/// ```
///
/// # Dependencies
///
/// - `id: BadgeId` field
///
/// Go to [BadgeId](crate::model::util::badge_id::BadgeId)
macro_rules! impl_for_id_badge_id {
    () => {
        /// Returns the badge icon URL.
        pub fn icon_url(&self) -> String {
            self.id.icon_url()
        }
    };
}

/// A macro to implement the method for `replay_id` field.
///
/// # Methods
///
/// ```ignore
/// pub fn replay_url(&self) -> String
/// ```
///
/// # Dependencies
///
/// - `replay_id: ReplayId` field
///
/// Go to [ReplayId](crate::model::util::replay_id::ReplayId)
macro_rules! impl_for_replay_id {
    () => {
        /// Returns the replay URL.
        pub fn replay_url(&self) -> String {
            self.replay_id.replay_url()
        }
    };
}
