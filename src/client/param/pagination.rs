//! Features for pagination.
//!
//! Want to paginate over some data?
//! Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`](crate::client::Client::with_session_id) to ensure data consistency.  
//! For more details, see the example in
//! [`15_pagination-for-leaderboard.rs`](https://github.com/Rinrin0413/tetr-ch-rs/tree/master/examples/15_pagination-for-leaderboard.rs).

use serde::Deserialize;

/// A prisecter.
///
/// A **prisecter** is consisting of three floats.
/// It allows you to continue paginating.
///
/// Want to paginate over some data?
/// Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`](crate::client::Client::with_session_id) to ensure data consistency.  
/// For more details, see the example in
/// [`15_pagination-for-leaderboard.rs`](https://github.com/Rinrin0413/tetr-ch-rs/tree/master/examples/15_pagination-for-leaderboard.rs).
#[derive(Clone, Debug, Deserialize)]
pub struct Prisecter {
    /// The primary sort key.
    pub pri: f64,
    /// The secondary sort key.
    pub sec: f64,
    /// The tertiary sort key.
    pub ter: f64,
}

impl Prisecter {
    /// Converts to an array.
    ///
    /// This array can be used as a bound for the next search.
    pub fn to_array(&self) -> [f64; 3] {
        [self.pri, self.sec, self.ter]
    }
}

impl AsRef<Prisecter> for Prisecter {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// A bound to paginate.
///
/// Want to paginate over some data?
/// Remember to pass an `X-Session-ID` header using the [`Client::with_session_id`](crate::client::Client::with_session_id) to ensure data consistency.  
/// For more details, see the example in
/// [`15_pagination-for-leaderboard.rs`](https://github.com/Rinrin0413/tetr-ch-rs/tree/master/examples/15_pagination-for-leaderboard.rs).
#[derive(Clone, Debug)]
pub enum Bound {
    /// An upper bound.
    /// Use this to paginate downwards:
    /// take the lowest seen prisecter and pass that back through this field to continue scrolling.
    ///
    /// A **prisecter** is consisting of three floats.
    /// The `prisecter` field in a response data allows you to continue paginating.
    After([f64; 3]),
    /// A lower bound.
    /// Use this to paginate upwards:
    /// take the highest seen prisecter and pass that back through this field to continue scrolling.
    /// If set, the search order is reversed
    /// (returning the lowest items that match the query)
    ///
    /// A **prisecter** is consisting of three floats.
    /// The `prisecter` field in a response data allows you to continue paginating.
    Before([f64; 3]),
}

impl Bound {
    /// Converts into a query parameter.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::user_leaderboard::Bound;
    /// let bound = Bound::After([12345.678, 0.0, 0.0]);
    /// assert_eq!(bound.to_query_param(), ("after".to_string(), "12345.678:0:0".to_string()));
    /// ```
    pub(crate) fn to_query_param(&self) -> (String, String) {
        match self {
            Bound::After(b) => ("after".to_string(), format!("{}:{}:{}", b[0], b[1], b[2])),
            Bound::Before(b) => ("before".to_string(), format!("{}:{}:{}", b[0], b[1], b[2])),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prisecter_to_array_converts_to_array() {
        let prisecter = Prisecter {
            pri: 1.0,
            sec: 2.0,
            ter: 3.0,
        };
        assert_eq!(prisecter.to_array(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn prisecter_as_ref_converts_to_ref() {
        let prisecter = Prisecter {
            pri: 1.0,
            sec: 2.0,
            ter: 3.0,
        };
        let _ref_prisecter: &Prisecter = prisecter.as_ref();
    }

    #[test]
    fn bound_after_to_query_param_converts_into_query_param() {
        let bound = Bound::After([12345.678, 0.0, 0.0]);
        assert_eq!(
            bound.to_query_param(),
            ("after".to_string(), "12345.678:0:0".to_string())
        );
    }

    #[test]
    fn bound_before_to_query_param_converts_into_query_param() {
        let bound = Bound::Before([12345.678, 0.0, 0.0]);
        assert_eq!(
            bound.to_query_param(),
            ("before".to_string(), "12345.678:0:0".to_string())
        );
    }
}
