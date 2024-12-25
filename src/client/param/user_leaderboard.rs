//! Features for user leaderboards.

use super::pagination::Bound;
use crate::util::{encode, validate_limit};

/// A user leaderboard type.
#[derive(Clone, Debug)]
pub enum LeaderboardType {
    /// A TETRA LEAGUE leaderboard.
    League,
    /// An XP leaderboard.
    Xp,
    /// An Achievement Rating leaderboard.
    Ar,
}

impl LeaderboardType {
    /// Converts into a parameter string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::user_leaderboard::LeaderboardType;
    /// assert_eq!(LeaderboardType::League.to_param(), "league");
    /// assert_eq!(LeaderboardType::Xp.to_param(), "xp");
    /// assert_eq!(LeaderboardType::Ar.to_param(), "ar");
    /// ```
    pub(crate) fn to_param(&self) -> String {
        match self {
            LeaderboardType::League => "league".to_string(),
            LeaderboardType::Xp => "xp".to_string(),
            LeaderboardType::Ar => "ar".to_string(),
        }
    }
}

/// A search criteria for user leaderboards.
///
/// # Examples
///
/// ```
/// use tetr_ch::prelude::*;
///
/// // Default search criteria.
/// let c1 = user_leaderboard::SearchCriteria::new();
///
/// // Upper bound is `[15200, 0, 0]`, three entries, filter by Japan.
/// let c2 = user_leaderboard::SearchCriteria::new()
///     .after([15200., 0., 0.])
///     .limit(3)
///     .country("jp");
///
/// // Lower bound is `[15200, 0, 0]`.
/// // The leaderboard order is reversed.
/// let c3 = user_leaderboard::SearchCriteria::new()
///     .before([15200., 0., 0.]);
///
/// // You can initialize the search criteria to default as follows:
/// let mut c4 = user_leaderboard::SearchCriteria::new().country("us");
/// c4.init();
/// ```
#[derive(Clone, Debug, Default)]
pub struct SearchCriteria {
    /// The bound to paginate.
    pub bound: Option<Bound>,
    /// The amount of entries to return,
    /// between 1 and 100. 25 by default.
    pub limit: Option<u8>,
    /// The ISO 3166-1 country code to filter to.
    /// Leave unset to not filter by country.
    pub country: Option<String>,
}

impl SearchCriteria {
    /// Creates a new [`SearchCriteria`].
    /// The values are set to default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the search criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let mut criteria = SearchCriteria::new().country("us");
    /// criteria.init();
    /// ```
    pub fn init(&mut self) {
        self.bound = None;
        self.limit = None;
        self.country = None;
    }

    /// Sets the upper bound.
    ///
    /// # Arguments
    ///
    /// - `bound` -: The upper bound to paginate downwards:
    ///   take the lowest seen prisecter and pass that back through this field to continue scrolling.
    ///
    /// A **prisecter** is consisting of three floats.
    /// The `prisecter` field in a response data allows you to continue paginating.
    ///
    /// # Examples
    ///
    /// Sets the upper bound to `[10000.0, 0.0, 0.0]`.
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new().after([10000.0, 0.0, 0.0]);
    /// ```
    pub fn after(self, bound: [f64; 3]) -> Self {
        Self {
            bound: Some(Bound::After(bound)),
            ..self
        }
    }

    /// Sets the lower bound.
    ///
    /// # Arguments
    ///
    /// - `bound` - The lower bound to paginate upwards:
    ///   take the highest seen prisecter and pass that back through this field to continue scrolling.
    ///   If use this, the search order is reversed
    ///   (returning the lowest items that match the query)
    ///
    /// A **prisecter** is consisting of three floats.
    /// The `prisecter` field in a response data allows you to continue paginating.
    ///
    /// # Examples
    ///
    /// Sets the lower bound to `[10000.0, 0.0, 0.0]`.
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new().before([10000.0, 0.0, 0.0]);
    /// ```
    pub fn before(self, bound: [f64; 3]) -> Self {
        Self {
            bound: Some(Bound::Before(bound)),
            ..self
        }
    }

    /// Limits the amount of entries to return.
    ///
    /// # Arguments
    ///
    /// - `limit` - The amount of entries to return.
    ///   Between 1 and 100. 25 by default.
    ///
    /// # Examples
    ///
    /// Limits the amount of entries to return to `10`.
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new().limit(10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the argument `limit` is not between `1` and `100`.
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new().limit(0);
    /// ```
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new().limit(101);
    /// ```
    pub fn limit(self, limit: u8) -> Self {
        validate_limit(limit);
        Self {
            limit: Some(limit),
            ..self
        }
    }

    /// Sets the ISO 3166-1 country code to filter to.
    ///
    /// # Arguments
    ///
    /// - `country` - The ISO 3166-1 country code to filter to.
    ///
    /// # Examples
    ///
    /// Sets the country code to `jp`.
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let mut criteria = SearchCriteria::new().country("jp");
    /// ```
    pub fn country(self, country: &str) -> Self {
        Self {
            country: Some(country.to_owned()),
            ..self
        }
    }

    /// # Panics
    ///
    /// Panics if the limit is not between 1 and 100.
    pub(crate) fn validate_limit(&self) {
        if let Some(self_limit) = self.limit {
            validate_limit(self_limit)
        }
    }

    /// Builds the search criteria to `Vec<(String, String)>`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let criteria = SearchCriteria::new();
    /// let query_params = criteria.build();
    /// ```
    pub(crate) fn build(self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        if let Some(b) = self.bound {
            result.push(b.to_query_param());
        }
        if let Some(l) = self.limit {
            result.push(("limit".to_string(), l.to_string()));
        }
        if let Some(c) = self.country {
            result.push(("country".to_string(), encode(c.to_uppercase())));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaderboard_type_to_param_converts_into_param_str() {
        assert_eq!(LeaderboardType::League.to_param(), "league");
        assert_eq!(LeaderboardType::Xp.to_param(), "xp");
        assert_eq!(LeaderboardType::Ar.to_param(), "ar");
    }

    #[test]
    fn search_criteria_new_creates_default() {
        let criteria = SearchCriteria::new();
        assert!(criteria.bound.is_none());
        assert!(criteria.limit.is_none());
        assert!(criteria.country.is_none());
    }

    #[test]
    fn search_criteria_init_initializes() {
        let mut criteria = SearchCriteria::new()
            .after([15200.0, 0.0, 0.0])
            .limit(3)
            .country("jp");
        criteria.init();
        assert!(criteria.bound.is_none());
        assert!(criteria.limit.is_none());
        assert!(criteria.country.is_none());
    }

    #[test]
    fn search_criteria_after_sets_upper_bound() {
        let criteria = SearchCriteria::new().after([15200.0, 0.0, 0.0]);
        assert!(matches!(
            criteria.bound,
            Some(Bound::After([15200.0, 0.0, 0.0]))
        ));
    }

    #[test]
    fn search_criteria_before_sets_lower_bound() {
        let criteria = SearchCriteria::new().before([15200.0, 0.0, 0.0]);
        assert!(matches!(
            criteria.bound,
            Some(Bound::Before([15200.0, 0.0, 0.0]))
        ));
    }

    #[test]
    fn search_criteria_limit_sets_valid_limit() {
        for i in 1..=100 {
            let criteria = SearchCriteria::new().limit(i);
            assert_eq!(criteria.limit, Some(i));
        }
    }

    #[test]
    #[should_panic]
    fn search_criteria_limit_panics_if_out_of_range() {
        SearchCriteria::new().limit(0);
        SearchCriteria::new().limit(101);
    }

    #[test]
    fn search_criteria_country_sets_country() {
        let criteria = SearchCriteria::new().country("jp");
        assert_eq!(criteria.country, Some("jp".to_string()));
    }

    #[test]
    #[should_panic]
    fn search_criteria_validate_limit_panics_if_out_of_range() {
        SearchCriteria {
            limit: Some(0),
            ..SearchCriteria::default()
        }
        .validate_limit();
        SearchCriteria {
            limit: Some(101),
            ..SearchCriteria::default()
        }
        .validate_limit();
    }

    #[test]
    fn search_criteria_build_creates_query_params() {
        let criteria = SearchCriteria::new()
            .after([15200.0, 0.0, 0.0])
            .limit(3)
            .country("JP");
        let query_params = criteria.build();
        assert_eq!(
            query_params,
            vec![
                ("after".to_string(), "15200:0:0".to_string()),
                ("limit".to_string(), "3".to_string()),
                ("country".to_string(), "JP".to_string())
            ]
        );
    }

    #[test]
    fn search_criteria_build_returns_empty_vec_if_no_params() {
        let criteria = SearchCriteria::new();
        let query_params = criteria.build();
        assert!(query_params.is_empty());
    }
}
