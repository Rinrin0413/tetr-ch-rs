//! The User Personal Records models.

use crate::model::{cache::CacheData, summary::record::Record};
use serde::Deserialize;

/// The response for the User Personal Records data.
///
/// A list of Records fulfilling the search criteria.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserRecordsResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<UserRecords>,
}

impl AsRef<UserRecordsResponse> for UserRecordsResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The User Personal Records data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserRecords {
    /// The matched records.
    pub entries: Vec<Record>,
}
