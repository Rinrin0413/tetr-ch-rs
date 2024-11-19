//! Models for the endpoint "Server Activity".
//!
//! About the endpoint "Server Activity",
//! see the [API document](https://tetr.io/about/api/#generalactivity).

use crate::model::cache::CacheData;
use serde::Deserialize;

/// A struct for the response for the endpoint "Server Activity".
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerActivityResponse {
    /// Whether the request was successful.
    #[serde(rename = "success")]
    pub is_success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested data.
    pub data: Option<ServerActivity>,
}

impl AsRef<ServerActivityResponse> for ServerActivityResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// An array of user activity over the last 2 days.
/// A user is seen as active if they logged in or received XP within the last 30 minutes.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct ServerActivity {
    /// The array of plot points, newest points first.
    pub activity: Vec<u32>,
}

impl ServerActivity {
    /// Returns the peak point of the activity.
    ///
    /// If the activity is empty, `None` is returned.
    pub fn peak(&self) -> Option<u32> {
        self.activity.iter().max().copied()
    }

    /// Returns the index of the peak point of the activity.
    ///
    /// If several points are equally maximum, the first one is returned.
    /// If the activity is empty, `None` is returned.
    pub fn peak_index(&self) -> Option<usize> {
        self.activity
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i)
    }

    /// Returns the trough point of the activity.
    ///
    /// If the activity is empty, `None` is returned.
    pub fn trough(&self) -> Option<u32> {
        self.activity.iter().min().copied()
    }

    /// Returns the index of the trough point of the activity.
    ///
    /// If several points are equally minimum, the first one is returned.
    /// If the activity is empty, `None` is returned.
    pub fn trough_index(&self) -> Option<usize> {
        self.activity
            .iter()
            .enumerate()
            .min_by_key(|(_, &v)| v)
            .map(|(i, _)| i)
    }

    /// Returns the average of the activity.
    ///
    /// If the activity is empty, `None` is returned.
    pub fn average(&self) -> Option<f64> {
        let len = self.activity.len() as f64;
        if 0.0 < len {
            Some(self.activity.iter().sum::<u32>() as f64 / len)
        } else {
            None
        }
    }
}

impl AsRef<ServerActivity> for ServerActivity {
    fn as_ref(&self) -> &Self {
        self
    }
}
