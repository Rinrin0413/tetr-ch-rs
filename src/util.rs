//! Utilities for tetr-ch-rs.

use chrono::DateTime;
use serde::Deserialize;
use serde_json::Value;

/// Parses an RFC 3339 and ISO 8601 date and time string into a UNIX timestamp.
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

/// Deserialize from the given value to `Option<String>`.
///
/// If the given value is string, returns `Some(String)`.
/// Otherwise, returns `None`.
pub(crate) fn deserialize_from_non_str_to_none<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    if let Some(received_at) = value.as_str() {
        Ok(Some(received_at.to_owned()))
    } else {
        Ok(None)
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
    #[should_panic]
    fn panics_invalid_rfc3339() {
        let invalid_ts = "qawsedrftgyhujikolp";
        to_unix_ts(invalid_ts);
    }

    #[test]
    fn compare_and_return_maximum_of_two_f64_v1() {
        let v1 = -2.;
        let v2 = 2.;
        assert_eq!(max_f64(v1, v2), 2.0);
    }

    #[test]
    fn compare_and_return_maximum_of_two_f64_v2() {
        let v1 = 16.2;
        let v2 = 8.;
        assert_eq!(max_f64(v1, v2), 16.2);
    }
}
