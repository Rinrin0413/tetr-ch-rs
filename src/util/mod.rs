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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_to_unix_ts() {
        let ts = "2022-07-26T17:35:23.988Z";
        assert_eq!(to_unix_ts(ts), 1658856923);
    }

    #[test]
    fn compare_and_return_maximum_of_two_f64() {
        let v1 = -2.;
        let v2 = 2.;
        assert_eq!(max_f64(v1, v2), 2.0);
    }
}
