//! Utilities for tetr-ch-rs.

use crate::model::util::Timestamp;
use chrono::DateTime;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use serde_json::Value;

/// Converts the given XP to the level.
pub fn xp_to_level(xp: f64) -> u32 {
    // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
    ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.).floor()
        as u32
}

/// Compares and returns the maximum of two 64bit floats`.
fn max_f64(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        v2
    } else {
        v1
    }
}

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

/// Encode the given string for URLs.
pub(crate) fn encode(input: impl ToString) -> String {
    utf8_percent_encode(&input.to_string().replace('.', " "), NON_ALPHANUMERIC).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn xp_to_level_converts_xp_to_level() {
        assert_eq!(xp_to_level(0.), 1);
        assert_eq!(xp_to_level(4096.), 5);
        assert_eq!(xp_to_level(32768.), 19);
        assert_eq!(xp_to_level(262144.), 96);
        assert_eq!(xp_to_level(8388608.), 1770);
    }

    #[test]
    fn max_f64_returns_v1_if_v1_is_max() {
        let v1 = -2.;
        let v2 = 2.;
        assert_eq!(max_f64(v1, v2), 2.0);
    }

    #[test]
    fn max_f64_returns_v2_if_v2_is_max() {
        let v1 = 16.2;
        let v2 = 8.;
        assert_eq!(max_f64(v1, v2), 16.2);
    }

    #[test]
    fn to_unix_ts_parses_string_into_unix_ts() {
        let ts = "2022-07-26T17:35:23.988Z";
        assert_eq!(to_unix_ts(ts), 1658856923);
    }

    #[test]
    #[should_panic(expected = "Failed to parse the given string.")]
    fn to_unix_ts_panics_if_invalid_ts() {
        let invalid_ts = "qawsedrftgyhujikolp";
        to_unix_ts(invalid_ts);
    }

    #[test]
    fn deserialize_from_non_str_to_none_deserializes_str_to_timestamp() {
        let value: Value = json!("2022-07-26T17:35:23.988Z");
        let result = deserialize_from_non_str_to_none(value).unwrap();
        assert_eq!(
            result,
            Some(Timestamp::new("2022-07-26T17:35:23.988Z".to_string()))
        );
    }

    #[test]
    fn deserialize_from_non_str_to_none_deserializes_false_to_none() {
        let value: Value = json!(false);
        let result = deserialize_from_non_str_to_none(value).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn validate_limit_allows_valid_values() {
        for i in 1..=100 {
            validate_limit(i);
        }
    }

    #[test]
    #[should_panic]
    fn validate_limit_panics_if_out_of_range() {
        validate_limit(0);
        validate_limit(101);
    }

    #[test]
    fn encode_encodes_str() {
        assert_eq!(encode("Hello, world!"), "Hello%2C%20world%21");
        assert_eq!(encode("."), "%20");
    }
}
