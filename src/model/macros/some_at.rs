/// A macro to implement the methods for `created_at` field. (user account)
///
/// # Methods
///
/// ```ignore
/// pub fn created_at(&self) -> Option<i64>
/// ```
///
/// # Dependencies
///
/// - `created_at: Option<Timestamp>` field
///
/// Go to [Option] | [Timestamp](crate::model::util::timestamp::Timestamp)
macro_rules! impl_for_created_at {
    () => {
        /// Returns a UNIX timestamp when the user's account created.
        ///
        /// If the account was created before join dates were recorded, `None` is returned.
        ///
        /// # Panics
        ///
        /// Panics if failed to parse the timestamp.
        pub fn created_at(&self) -> Option<i64> {
            self.created_at.as_ref().map(|ts| ts.unix_ts())
        }
    };
}
