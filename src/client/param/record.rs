//! Features for records.

use super::pagination::Bound;

/// A game mode of a record.
pub enum Gamemode {
    /// 40 LINES.
    FortyLines,
    /// BLITZ.
    Blitz,
    /// QUICK PLAY.
    Zenith,
    /// EXPERT QUICK PLAY.
    ZenithEx,
    /// TETRA LEAGUE history.
    League,
}

impl Gamemode {
    /// Converts into a parameter string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::record::Gamemode;
    /// let forty_lines = Gamemode::FortyLines;
    /// let blitz = Gamemode::Blitz;
    /// let zenith = Gamemode::Zenith;
    /// let zenith_ex = Gamemode::ZenithEx;
    /// let league = Gamemode::League;
    /// assert_eq!(forty_lines.to_param(), "40l");
    /// assert_eq!(blitz.to_param(), "blitz");
    /// assert_eq!(zenith.to_param(), "zenith");
    /// assert_eq!(zenith_ex.to_param(), "zenithex");
    /// assert_eq!(league.to_param(), "league");
    /// ```
    pub(crate) fn to_param(&self) -> String {
        match self {
            Gamemode::FortyLines => "40l",
            Gamemode::Blitz => "blitz",
            Gamemode::Zenith => "zenith",
            Gamemode::ZenithEx => "zenithex",
            Gamemode::League => "league",
        }
        .to_string()
    }
}

/// A record leaderboard type.
pub enum LeaderboardType {
    /// Top scores.
    Top,
    /// Most recently placed records.
    Recent,
    /// Top scores (Personal Bests only).
    Progression,
}

impl LeaderboardType {
    /// Converts into a parameter string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::record::LeaderboardType;
    /// let top = LeaderboardType::Top;
    /// let recent = LeaderboardType::Recent;
    /// let progression = LeaderboardType::Progression;
    /// assert_eq!(top.to_param(), "top");
    /// assert_eq!(recent.to_param(), "recent");
    /// assert_eq!(progression.to_param(), "progression");
    /// ```
    pub(crate) fn to_param(&self) -> String {
        match self {
            LeaderboardType::Top => "top",
            LeaderboardType::Recent => "recent",
            LeaderboardType::Progression => "progression",
        }
        .to_string()
    }
}

/// A search criteria for user records.
///
/// # Examples
///
/// ```
/// use tetr_ch::client::param::record::SearchCriteria;
///
/// // Default search criteria.
/// let c1 = SearchCriteria::new();
///
/// // Upper bound is `[500000, 0, 0]`, three entries.
/// let c2 = SearchCriteria::new()
///     .after([500000., 0., 0.])
///     .limit(3);
///
/// // Lower bound is `[500000, 0, 0]`.
/// // Also the search order is reversed.
/// let c3 = SearchCriteria::new()
///     .before([500000., 0., 0.]);
///
/// // You can initialize the search criteria to default as follows:
/// let mut c4 = SearchCriteria::new().limit(10);
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
    /// Sets the upper bound to `[500000.0, 0.0, 0.0]`.
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.after([500000.0, 0.0, 0.0]);
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
    /// Sets the lower bound to `[500000.0, 0.0, 0.0]`.
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.before([500000.0, 0.0, 0.0]);
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
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.limit(10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the argument `limit` is not between `1` and `100`.
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let mut criteria = SearchCriteria::new();
    /// criteria.limit(0);
    /// ```
    ///
    /// ```should_panic
    /// # use tetr_ch::client::param::record::SearchCriteria;
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

    /// Whether the search criteria `limit` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
    /// let invalid_criteria = SearchCriteria {
    ///     limit: Some(0),
    ///     ..SearchCriteria::new()
    /// };
    /// assert!(invalid_criteria.is_invalid_limit_range());
    /// ```
    ///
    /// ```
    /// # use tetr_ch::client::param::record::SearchCriteria;
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
