//! Server Activity model.

use crate::model::cache::CacheData;
use serde::Deserialize;

/// The response for the server activity information.
///
/// A graph of count of active players over the last 2 days.
/// A user is seen as active if they logged in or received XP within the last 30 minutes.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerActivityResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested user data.
    pub data: Option<ServerActivity>,
}

impl ServerActivityResponse {
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

    /// Returns the `&ServerActivity`.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    fn _get_server_activity(&self) -> &ServerActivity {
        if let Some(d) = self.data.as_ref() {
            d
        } else {
            panic!("There is no server activity object because the request was not successful.")
        }
    }
}

impl AsRef<ServerActivityResponse> for ServerActivityResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The requested user data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerActivity {
    /// An array of plot points,
    /// newest points first.
    pub activity: Vec<u32>,
}

impl AsRef<ServerActivity> for ServerActivity {
    fn as_ref(&self) -> &Self {
        self
    }
}
