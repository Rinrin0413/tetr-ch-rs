//! Features for user leaderboards.

/// A user leaderboard type.
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
    /// # use tetr_ch::client::leaderboard::LeaderboardType;
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
/// use tetr_ch::client::param::user_leaderboard::SearchCriteria;
///
/// // Default search criteria.
/// let c1 = SearchCriteria::new();
///
/// // Upper bound is `[15200, 0, 0]`, three entries, filter by Japan.
/// let c2 = SearchCriteria::new()
///     .after([15200., 0., 0.])
///     .limit(3)
///     .country("jp");
///
/// // Lower bound is `[15200, 0, 0]`.
/// // Also the search order is reversed.
/// let c3 = SearchCriteria::new()
///     .before([15200., 0., 0.]);
///
/// // You can initialize the search criteria to default as follows:
/// let mut c4 = SearchCriteria::new().country("us");
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
    pub fn init(self) -> Self {
        Self::default()
    }

    /// Sets the upper bound.
    ///
    /// # Arguments
    ///
    /// - `bound`: The upper bound to paginate downwards:
    /// take the lowest seen prisecter and pass that back through this field to continue scrolling.
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
    /// let mut criteria = SearchCriteria::new();
    /// criteria.after([10000.0, 0.0, 0.0]);
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
    /// - `bound`: The lower bound to paginate upwards:
    /// take the highest seen prisecter and pass that back through this field to continue scrolling.
    /// If use this, the search order is reversed
    /// (returning the lowest items that match the query)
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
    /// let mut criteria = SearchCriteria::new();
    /// criteria.before([10000.0, 0.0, 0.0]);
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
    /// - `limit`: The amount of entries to return.
    /// Between 1 and 100. 25 by default.
    ///
    /// # Examples
    ///
    /// Limits the amount of entries to return to `10`.
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.limit(10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the argument `limit` is not between `1` and `100`.
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.limit(0);
    /// ```
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.limit(101);
    /// ```
    pub fn limit(self, limit: u8) -> Self {
        if (1..=100).contains(&limit) {
            Self {
                limit: Some(limit),
                ..self
            }
        } else {
            panic!(
                "The argument `limit` must be between 1 and 100.\n\
                Received: {}",
                limit
            );
        }
    }

    /// Sets the ISO 3166-1 country code to filter to.
    ///
    /// # Arguments
    ///
    /// - `country`: The ISO 3166-1 country code to filter to.
    ///
    /// # Examples
    ///
    /// Sets the country code to `jp`.
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.country("jp");
    /// ```
    pub fn country(self, country: &str) -> Self {
        Self {
            country: Some(country.to_owned().to_uppercase()),
            ..self
        }
    }

    /// Whether the search criteria `limit` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let invalid_criteria = SearchCriteria {
    ///     limit: Some(0),
    ///     ..SearchCriteria::new()
    /// };
    /// assert!(invalid_criteria.is_invalid_limit_range());
    /// ```
    ///
    /// ```
    /// # use tetr_ch::client::param::user_leaderboard::SearchCriteria;
    /// let invalid_criteria = SearchCriteria {
    ///     limit: Some(101),
    ///     ..SearchCriteria::new()
    /// };
    /// assert!(invalid_criteria.is_invalid_limit_range());
    /// ```
    pub fn is_invalid_limit_range(&self) -> bool {
        if let Some(l) = self.limit {
            !(1..=100).contains(&l)
        } else {
            false
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
            result.push(("country".to_string(), c));
        }
        result
    }
}

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
