//! A model for the cache data.
//!
//! For more details, see the [API document](https://tetr.io/about/api/#cachedata).

use serde::Deserialize;
use std::fmt;

/// Data about how a request was cached.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct CacheData {
    /// Whether the cache was hit.
    /// Either `"hit"`, `"miss"`, or `"awaited"`.
    /// `"awaited"` means resource was already being requested by another client.
    pub status: Status,
    /// When this resource was cached.
    pub cached_at: u64,
    /// When this resource's cache expires.
    pub cached_until: u64,
}

impl CacheData {
    /// Returns a UNIX timestamp when this resource was cached.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // An example.
    /// assert_eq!(cache_data.cached_at(), 1596317823);
    /// ```
    pub fn cached_at(&self) -> i64 {
        self.cached_at as i64 / 1000
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // An example.
    /// assert_eq!(cache_data.cached_until(), 1661710844);
    /// ```
    pub fn cached_until(&self) -> i64 {
        self.cached_until as i64 / 1000
    }
}

impl AsRef<CacheData> for CacheData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A status of the cache.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Status {
    Hit,
    Miss,
    /// Resource was already being requested by another client.
    Awaited,
}

impl AsRef<Status> for Status {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Hit => write!(f, "hit"),
            Status::Miss => write!(f, "miss"),
            Status::Awaited => write!(f, "awaited"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cached_at_and_cached_until() {
        let cache_data = CacheData {
            status: Status::Hit,
            cached_at: 1661710769000,
            cached_until: 1661710844000,
        };
        assert_eq!(cache_data.cached_at(), 1661710769);
        assert_eq!(cache_data.cached_until(), 1661710844);
    }

    #[test]
    fn cache_data_as_ref() {
        let cache_data = CacheData {
            status: Status::Hit,
            cached_at: 1661710769000,
            cached_until: 1661710844000,
        };
        assert_eq!(cache_data.as_ref().status, Status::Hit);
        assert_eq!(cache_data.as_ref().cached_at(), 1661710769);
        assert_eq!(cache_data.as_ref().cached_until(), 1661710844);
    }
}
