//! Features for pagination.

/// A bound to paginate.
#[derive(Clone, Debug)]
pub enum Bound {
    /// A upper bound.
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
