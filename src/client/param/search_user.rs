//! Features for the [`Client::search_user`](crate::client::Client::search_user) method.

/// A social connection.
///
/// [API document](https://tetr.io/about/api/#userssearchquery) says searching for the other social links will be added in the near future.
pub enum SocialConnection {
    /// A Discord ID.
    Discord(String),
}

impl SocialConnection {
    /// Converts into a parameter string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::search_user::SocialConnection;
    /// let connection = SocialConnection::Discord("724976600873041940".to_string());
    /// assert_eq!(connection.to_param(), "discord:724976600873041940");
    /// ```
    pub(crate) fn to_param(&self) -> String {
        match self {
            SocialConnection::Discord(id) => format!("discord:{}", id),
        }
    }
}
