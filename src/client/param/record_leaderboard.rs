//! Features for record leaderboards.

use super::pagination::Bound;
use crate::util::validate_limit;

/// A record leaderboard ID.
#[derive(Clone, Debug)]
pub struct RecordsLeaderboardId {
    /// The game mode. e.g. `40l`.
    pub gamemode: String,
    /// The scope.
    pub scope: Scope,
    /// The optional Revolution ID. e.g. `@2024w31`.
    pub revolution_id: Option<String>,
}

impl RecordsLeaderboardId {
    /// Creates a new [`RecordsLeaderboardId`].
    ///
    /// # Arguments
    ///
    /// - `gamemode` - The game mode. e.g. `40l`.
    /// - `scope` - The scope. ether [`Scope::Global`] or [`Scope::Country`].
    /// - `revolution_id` - The optional Revolution ID. e.g. `@2024w31`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::client::param::record_leaderboard::{RecordsLeaderboardId, Scope};
    /// let id = RecordsLeaderboardId::new("40l", Scope::Global, None);
    /// ```
    pub fn new(gamemode: &str, scope: Scope, revolution_id: Option<&str>) -> Self {
        Self {
            gamemode: gamemode.to_owned(),
            scope,
            revolution_id: revolution_id.map(|s| s.to_owned()),
        }
    }

    /// Converts into a parameter.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::record_leaderboard::{RecordsLeaderboardId, Scope};
    /// let id1 = RecordsLeaderboardId::new("40l", Scope::Global, None);
    /// let id2 = RecordsLeaderboardId::new("blitz", Scope::Country("JP".to_string()), None);
    /// let id3 = RecordsLeaderboardId::new("zenith", Scope::Global, Some("@2024w31"));
    /// assert_eq!(id1.to_param(), "40l_global");
    /// assert_eq!(id2.to_param(), "blitz_country_JP");
    /// assert_eq!(id3.to_param(), "zenith_global@2024w31");
    /// ```
    pub(crate) fn to_param(&self) -> String {
        let scope = match &self.scope {
            Scope::Global => "global".to_string(),
            Scope::Country(c) => format!("country_{}", c.to_uppercase()),
        };
        let revolution_id = self.revolution_id.as_deref().unwrap_or("");
        format!("{}_{}{}", self.gamemode, scope, revolution_id)
    }
}

/// A scope of record leaderboards.
#[derive(Clone, Debug)]
pub enum Scope {
    /// Global scope.
    Global,
    /// Country scope.
    /// Contains a country code.
    /// e.g. `JP`.
    Country(String),
}

/// A search criteria for the records leaderboard.
///
/// # Examples
///
/// ```
/// use tetr_ch::prelude::*;
///
/// // Default search criteria.
/// let c1 = record_leaderboard::SearchCriteria::new();
///
/// // Upper bound is `[500000, 0, 0]`, three entries.
/// let c2 = record_leaderboard::SearchCriteria::new()
///     .after([500000., 0., 0.])
///     .limit(3);
///
/// // Lower bound is `[500000, 0, 0]`.
/// // The leaderboard order is reversed.
/// let c3 = record_leaderboard::SearchCriteria::new()
///     .before([500000., 0., 0.]);
///
/// // You can initialize the search criteria to default as follows:
/// let mut c4 = record_leaderboard::SearchCriteria::new().limit(10);
/// c4.init();
/// ```
#[derive(Clone, Debug, Default)]
pub struct SearchCriteria {
    /// The bound to paginate.
    pub bound: Option<Bound>,
    /// The amount of entries to return,
    /// between 1 and 100. 25 by default.
    pub limit: Option<u8>,
}

impl SearchCriteria {
    /// Creates a new [`SearchCriteria`].
    /// The values are set to default.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
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
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.init();
    /// ```
    pub fn init(&mut self) {
        self.bound = None;
        self.limit = None;
    }

    /// Sets the upper bound.
    ///
    /// # Arguments
    ///
    /// - `bound` - The upper bound to paginate downwards:
    ///   take the lowest seen prisecter and pass that back through this field to continue scrolling.
    ///
    /// A **prisecter** is consisting of three floats.
    /// The `prisecter` field in a response data allows you to continue paginating.
    ///
    /// # Examples
    ///
    /// Sets the upper bound to `[500000.0, 0.0, 0.0]`.
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let criteria = SearchCriteria::new().after([500000.0, 0.0, 0.0]);
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
    /// Sets the lower bound to `[500000.0, 0.0, 0.0]`.
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let criteria = SearchCriteria::new().before([500000.0, 0.0, 0.0]);
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
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let criteria = SearchCriteria::new().limit(10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the argument `limit` is not between `1` and `100`.
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let criteria = SearchCriteria::new().limit(0);
    /// ```
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let criteria = SearchCriteria::new().limit(101);
    /// ```
    pub fn limit(self, limit: u8) -> Self {
        validate_limit(limit);
        Self {
            limit: Some(limit),
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
    /// # use tetr_ch::client::param::record::SearchCriteria;
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
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_leaderboard_id_new_creates_default() {
        let id = RecordsLeaderboardId::new("40l", Scope::Global, None);
        assert_eq!(id.gamemode, "40l");
        assert!(matches!(id.scope, Scope::Global));
        assert!(id.revolution_id.is_none());
    }

    #[test]
    fn records_leaderboard_id_to_param_converts_to_param() {
        let id1 = RecordsLeaderboardId::new("40l", Scope::Global, None);
        let id2 = RecordsLeaderboardId::new("blitz", Scope::Country("jp".to_string()), None);
        let id3 = RecordsLeaderboardId::new("zenith", Scope::Global, Some("@2024w31"));
        assert_eq!(id1.to_param(), "40l_global");
        assert_eq!(id2.to_param(), "blitz_country_JP");
        assert_eq!(id3.to_param(), "zenith_global@2024w31");
    }

    #[test]
    fn search_criteria_new_creates_default() {
        let criteria: SearchCriteria = SearchCriteria::new();
        assert!(criteria.bound.is_none());
        assert!(criteria.limit.is_none());
    }

    #[test]
    fn search_criteria_init_initializes() {
        let mut criteria = SearchCriteria::new().after([500000., 0., 0.]).limit(3);
        criteria.init();
        assert!(criteria.bound.is_none());
        assert!(criteria.limit.is_none());
    }

    #[test]
    fn search_criteria_after_sets_upper_bound() {
        let criteria = SearchCriteria::new().after([500000.0, 0.0, 0.0]);
        assert!(matches!(
            criteria.bound,
            Some(Bound::After([500000.0, 0.0, 0.0]))
        ));
    }

    #[test]
    fn search_criteria_before_sets_lower_bound() {
        let criteria = SearchCriteria::new().before([500000.0, 0.0, 0.0]);
        assert!(matches!(
            criteria.bound,
            Some(Bound::Before([500000.0, 0.0, 0.0]))
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
    fn search_criteria_build_returns_query_params() {
        let criteria = SearchCriteria::new().after([500000., 0., 0.]).limit(3);
        let query_params: Vec<(String, String)> = criteria.build();
        assert_eq!(
            query_params,
            vec![
                ("after".to_string(), "500000:0:0".to_string()),
                ("limit".to_string(), "3".to_string())
            ]
        );
    }

    #[test]
    fn search_criteria_build_returns_empty_vec_if_no_params() {
        let criteria = SearchCriteria::new();
        let query_params: Vec<(String, String)> = criteria.build();
        assert!(query_params.is_empty());
    }
}
