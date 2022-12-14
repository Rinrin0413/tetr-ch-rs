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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cached_at_and_cached_until() {
        let cache_data = CacheData {
            status: "hit".to_string(),
            cached_at: 1661710769000,
            cached_until: 1661710844000,
        };
        assert_eq!(cache_data.cached_at(), 1661710769);
        assert_eq!(cache_data.cached_until(), 1661710844);
    }

    #[test]
    fn cache_data_as_ref() {
        let cache_data = CacheData {
            status: "hit".to_string(),
            cached_at: 1661710769000,
            cached_until: 1661710844000,
        };
        assert_eq!(cache_data.as_ref().status, "hit");
        assert_eq!(cache_data.as_ref().cached_at(), 1661710769);
        assert_eq!(cache_data.as_ref().cached_until(), 1661710844);
    }
}
