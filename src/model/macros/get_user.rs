/// A macro to implement the method `get_user`.
///
/// # Methods
///
/// ```ignore
/// pub async fn get_user(&self) -> RspErr<UserResponse>
/// ```
///
/// # Dependencies
///
/// - `fn to_string(&self) -> String` method or `{specified field}: impl ToString` field
///
/// # Examples
///
/// By implementing the [`ToString`] trait:
///
/// ```ignore
/// use std::fmt;
///
/// pub struct UserId(String);
///
/// impl UserId {
///     impl_get_user!();
/// }
///
/// impl fmt::Display for UserId {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "{}", self.0)
///     }
/// }
/// ```
///
/// By specifying a field:
///
/// ```ignore
/// pub struct PartialUser {
///     id: UserId,
///     username: String,
///     // ...
/// }
///
/// impl PartialUser {
///     impl_get_user!(id);
///     // or:
///     // impl_get_user!(username);
/// }
/// ```
///
/// Go to [String] | [ToString]
macro_rules! impl_get_user {
    () => {
        /// Gets the detailed information about the user.
        ///
        /// # Errors
        ///
        /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
        /// if the request failed.
        /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
        /// if the response did not match the expected format but the HTTP request succeeded.
        /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
        /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
        /// if the HTTP request failed and the response did not match the expected format.
        /// Even if the HTTP request failed,
        /// it may be possible to deserialize the response containing an error message,
        /// so the deserialization will be tried before returning this error.
        pub async fn get_user(
            &self,
        ) -> crate::client::error::RspErr<crate::model::user::UserResponse> {
            crate::client::Client::new()
                .get_user(&self.to_string())
                .await
        }
    };
    ($field:ident) => {
        /// Gets the detailed information about the user.
        ///
        /// # Errors
        ///
        /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
        /// if the request failed.
        /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
        /// if the response did not match the expected format but the HTTP request succeeded.
        /// There may be defectives in this wrapper or the TETRA CHANNEL API document.
        /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
        /// if the HTTP request failed and the response did not match the expected format.
        /// Even if the HTTP request failed,
        /// it may be possible to deserialize the response containing an error message,
        /// so the deserialization will be tried before returning this error.
        pub async fn get_user(
            &self,
        ) -> crate::client::error::RspErr<crate::model::user::UserResponse> {
            crate::client::Client::new()
                .get_user(&self.$field.to_string())
                .await
        }
    };
}
