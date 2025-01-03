//! Features for news streams.

/// A news stream.
#[derive(Clone, Debug)]
pub enum NewsStream {
    /// A global news stream.
    Global,
    /// A news stream of the user.
    /// Contains a user ID.
    User(String),
}

impl ToNewsStreamParam for NewsStream {
    /// Converts into a parameter string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::news_stream::NewsStream;
    /// let global = NewsStream::Global;
    /// let user = NewsStream::User("621db46d1d638ea850be2aa0".to_string());
    /// assert_eq!(global.to_param(), "global");
    /// assert_eq!(user.to_param(), "user_621db46d1d638ea850be2aa0");
    /// ```
    fn to_param(&self) -> String {
        match self {
            NewsStream::Global => "global".to_string(),
            NewsStream::User(id) => format!("user_{}", id),
        }
    }
}

pub trait ToNewsStreamParam {
    /// Converts into a parameter string.
    fn to_param(&self) -> String;
}
