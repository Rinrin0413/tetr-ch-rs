//! Utilities for tetr-ch-rs.

use crate::model::util::timestamp::Timestamp;
use chrono::DateTime;
use serde::Deserialize;
use serde_json::Value;

/// Parses an RFC 3339 and ISO 8601 date and time string into a UNIX timestamp.
///
/// # Panics
///
/// Panics if failed to parse the given string.
pub(crate) fn to_unix_ts(ts: &str) -> i64 {
    DateTime::parse_from_rfc3339(ts)
        .expect("Failed to parse the given string.")
        .timestamp()
}

/// Compares and returns the maximum of two 64bit floats`.
pub(crate) fn max_f64(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        v2
    } else {
        v1
    }
}

/// Deserializes from the given value to `Option<Timestamp>`.
///
/// If the given value is string, returns `Some(Timestamp)`.
/// Otherwise, returns `None`.
pub(crate) fn deserialize_from_non_str_to_none<'de, D>(
    deserializer: D,
) -> Result<Option<Timestamp>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    if let Some(received_at) = value.as_str() {
        Ok(Some(Timestamp::new(received_at.to_owned())))
    } else {
        Ok(None)
    }
}

/// # Panics
///
/// Panics with a message "The limit must be between 1 and 100, but got X."
/// if the given value is not between 1 and 100.
pub(crate) fn validate_limit(value: u8) {
    assert!(
        (1..=100).contains(&value),
        "The limit must be between 1 and 100, but got {}.",
        value
    );
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
