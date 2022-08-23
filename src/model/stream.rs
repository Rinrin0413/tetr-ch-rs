//! Stream model.

use crate::model::{cache::CacheData, record::SinglePlayRecord};
use serde::Deserialize;

/// The response for the stream.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct StreamResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested stream data.
    pub data: Option<StreamData>,
}
impl StreamResponse {
    /// Returns a UNIX timestamp when this resource was cached.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_at(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_until(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }
}

impl AsRef<StreamResponse> for StreamResponse {
    fn as_ref(&self) -> &StreamResponse {
        self
    }
}

/// The requested stream data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct StreamData {
    pub records: Vec<SinglePlayRecord>,
}

impl AsRef<StreamData> for StreamData {
    fn as_ref(&self) -> &StreamData {
        self
    }
}
