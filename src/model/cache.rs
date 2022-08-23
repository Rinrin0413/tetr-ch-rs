//! The cache-related data.

use serde::Deserialize;

/// Data about how this request was cached.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct CacheData {
    /// Whether the cache was hit.
    /// Either `"hit"`, `"miss"`, or `"awaited"`.
    /// `"awaited"` means resource was already being requested by another client.
    pub status: String,
    /// When this resource was cached.
    pub cached_at: u64,
    /// When this resource's cache expires.
    pub cached_until: u64,
}

impl CacheData {
    /// Returns a UNIX timestamp when this resource was cached.
    pub fn cached_at(&self) -> i64 {
        self.cached_at as i64 / 1000
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    pub fn cached_until(&self) -> i64 {
        self.cached_until as i64 / 1000
    }
}

impl AsRef<CacheData> for CacheData {
    fn as_ref(&self) -> &Self {
        self
    }
}
