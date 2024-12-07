//! A model for the endpoint "Record Search".
//!
//! About the endpoint "Record Search",
//! see the [API document](https://tetr.io/about/api/#recordsreverse).

use crate::model::prelude::*;

/// A struct for the response for the endpoint "Record Search".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct SearchedRecordResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<ErrorResponse>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<Record>,
}
