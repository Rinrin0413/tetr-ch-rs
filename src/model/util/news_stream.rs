//! A model for news streams.

use crate::{
    client::{error::RspErr, param::news_stream::ToNewsStreamParam, Client},
    model::{news::NewsLatestResponse, prelude::*},
};

/// A news stream.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct NewsStream(String);

impl NewsStream {
    /// Gets the latest news items in the stream.
    /// Calls the [`Client::get_news_latest`](crate::client::Client::get_news_latest) method.
    ///
    /// # Arguments
    ///
    /// - `limit` - The amount of entries to return, between 1 and 100.
    ///
    /// # Panics
    ///
    /// Panics if the argument `limit` is not between 1 and 100.
    ///
    /// # Errors
    ///
    /// - A [`ResponseError::RequestErr`](crate::client::error::ResponseError::RequestErr) is returned,
    ///   if the request failed.
    /// - A [`ResponseError::DeserializeErr`](crate::client::error::ResponseError::DeserializeErr) is returned,
    ///   if the response did not match the expected format but the HTTP request succeeded.
    ///   There may be defectives in this wrapper or the TETRA CHANNEL API document.
    /// - A [`ResponseError::HttpErr`](crate::client::error::ResponseError::HttpErr) is returned,
    ///   if the HTTP request failed and the response did not match the expected format.
    ///   Even if the HTTP request failed,
    ///   it may be possible to deserialize the response containing an error message,
    ///   so the deserialization will be tried before returning this error.
    pub async fn get_news_items(self, limit: u8) -> RspErr<NewsLatestResponse> {
        Client::new().get_news_latest(self, limit).await
    }

    /// Whether the stream is the global news stream.
    pub fn is_global_steam(&self) -> bool {
        self.0 == "global"
    }

    /// Whether the stream is a news stream of a user.
    pub fn is_user_steam(&self) -> bool {
        self.0.starts_with("user_")
    }
}

impl AsRef<NewsStream> for NewsStream {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for NewsStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToNewsStreamParam for NewsStream {
    fn to_param(&self) -> String {
        self.0.clone()
    }
}
