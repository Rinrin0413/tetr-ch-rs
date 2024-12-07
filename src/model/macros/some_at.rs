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
macro_rules! impl_for_account_created_at {
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

/// A macro to implement the methods for `received_at` field. (badge)
///
/// # Methods
///
/// ```ignore
/// pub fn received_at(&self) -> Option<i64>
/// ```
///
/// # Dependencies
///
/// - `received_at: Option<Timestamp>` field
macro_rules! impl_for_received_at {
    () => {
        /// Returns a UNIX timestamp when the badge was achieved.
        ///
        /// If the badge was shown, `None` is returned.
        ///
        /// # Panics
        ///
        /// Panics if failed to parse the timestamp.
        pub fn received_at(&self) -> Option<i64> {
            self.received_at.as_ref().map(|ts| ts.unix_ts())
        }
    };
}

/// A macro to implement the methods for `created_at` field. (news item)
///
/// # Methods
///
/// ```ignore
/// pub fn created_at(&self) -> i64
/// ```
///
/// # Dependencies
///
/// - `created_at: Timestamp` field
macro_rules! impl_for_news_created_at {
    () => {
        /// Returns a UNIX timestamp when the news item was created.
        ///
        /// # Panics
        ///
        /// Panics if failed to parse the timestamp.
        pub fn created_at(&self) -> i64 {
            self.created_at.unix_ts()
        }
    };
}

/// A macro to implement the methods for `submitted_at` field. (record)
///
/// # Methods
///
/// ```ignore
/// pub fn submitted_at(&self) -> i64
/// ```
///
/// # Dependencies
///
/// - `submitted_at: Timestamp` field
macro_rules! impl_for_submitted_at {
    () => {
        /// Returns a UNIX timestamp when the record was submitted.
        ///
        /// # Panics
        ///
        /// Panics if failed to parse the timestamp.
        pub fn submitted_at(&self) -> i64 {
            self.submitted_at.unix_ts()
        }
    };
}
