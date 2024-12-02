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

/// Includes a macro to implement the methods for `role` field.
#[macro_use]
mod role;

/// A macro to implement the methods for `username` field.
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

/// A macro to implement the methods for `xp` field.
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
/// - [`crate::util::max_f64`] function
macro_rules! impl_for_xp {
    () => {
        /// Returns the level of the user.
        pub fn level(&self) -> u32 {
            let xp = self.xp;
            // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
            ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.)
                .floor() as u32
        }
    };
}
