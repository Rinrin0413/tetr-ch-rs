//! Utilities for the tetr-ch-rs.

use chrono::DateTime;

/// Parses an RFC 3339 and ISO 8601 date to UNIX timestamp as `i64`.
pub(crate) fn to_unix_ts(ts: &str) -> i64 {
    match DateTime::parse_from_rfc3339(ts) {
        Ok(dt) => dt.timestamp(),
        Err(e) => panic!("{}", e),
    }
}

/// Compares and returns the maximum of two 64bit floats`.
pub(crate) fn max_f64(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        v2
    } else {
        v1
    }
}
