//! A model for achievements.
//!
//! For more details, see the [API document](https://tetr.io/about/api/#achievementdata).

use crate::model::prelude::*;

/// An achievement.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Achievement {
    /// The Achievement ID, for every type of achievement.
    #[serde(rename = "k")]
    pub id: u32,
    /// The category of the achievement.
    pub category: String,
    /// The primary name of the achievement.
    pub name: String,
    /// The objective of the achievement.
    pub object: String,
    /// The flavor text of the achievement.
    pub desc: String,
    /// The order of this achievement in its category.
    ///
    /// ***The API document does not say this field is optional.**
    #[serde(rename = "o")]
    pub order: Option<u32>,
    /// The rank type of this achievement.
    ///
    /// - 1 = PERCENTILE — ranked by percentile cutoffs (5% Diamond, 10% Platinum, 30% Gold, 50% Silver, 70% Bronze)
    /// - 2 = ISSUE — always has the ISSUED rank
    /// - 3 = ZENITH — ranked by QUICK PLAY floors
    /// - 4 = PERCENTILELAX — ranked by percentile cutoffs (5% Diamond, 20% Platinum, 60% Gold, 100% Silver)
    /// - 5 = PERCENTILEVLAX — ranked by percentile cutoffs (20% Diamond, 50% Platinum, 100% Gold)
    /// - 6 = PERCENTILEMLAX — ranked by percentile cutoffs (10% Diamond, 20% Platinum, 50% Gold, 100% Silver)
    #[serde(rename = "rt")]
    pub rank_type: u32,
    /// The value type of this achievement:
    ///
    /// - 0 = NONE — [`Achievement::value`] is `None`
    /// - 1 = NUMBER — [`Achievement::value`] is a positive number
    /// - 2 = TIME — [`Achievement::value`] is a positive amount of milliseconds
    /// - 3 = TIME_INV — [`Achievement::value`] is a negative amount of milliseconds; negate it before displaying
    /// - 4 = FLOOR — [`Achievement::value`] is an altitude, [`Achievement::additional`] is a floor number
    /// - 5 = ISSUE — [`Achievement::value`] is the negative time of issue
    /// - 6 = NUMBER_INV — [`Achievement::value`] is a negative number; negate it before displaying
    #[serde(rename = "vt")]
    pub value_type: u32,
    /// The AR type of this achievement:
    ///
    /// - 0 = UNRANKED — no AR is given
    /// - 1 = RANKED — AR is given for medal ranks
    /// - 2 = COMPETITIVE — AR is given for medal ranks and leaderboard positions
    #[serde(rename = "art")]
    pub ar_type: u32,
    /// The minimum score required to obtain the achievement.
    pub min: i64,
    /// The amount of decimal placed to show.
    pub deci: u32,
    /// Whether this achievement is usually not shown.
    #[serde(rename = "hidden")]
    pub is_hidden: bool,
    /// The achieved score.
    #[serde(rename = "v")]
    pub value: Option<f64>,
    /// Additional data (see [`Achievement::value_type`]).
    #[serde(rename = "a")]
    pub additional: Option<f64>,
    /// The time the achievement was updated.
    #[serde(rename = "t")]
    pub time: Option<String>,
    /// The zero-indexed position in the achievement's leaderboards.
    #[serde(rename = "pos")]
    pub position: Option<i32>,
    /// The total amount of players who have this achievement
    /// (with a value of min or higher).
    pub total: Option<i32>,
    /// The rank of the achievement.
    ///
    /// - 0 = NONE,
    /// - 1 = BRONZE,
    /// - 2 = SILVER,
    /// - 3 = GOLD,
    /// - 4 = PLATINUM,
    /// - 5 = DIAMOND,
    /// - 100 = ISSUED
    pub rank: Option<u32>,
}

impl AsRef<Achievement> for Achievement {
    fn as_ref(&self) -> &Self {
        self
    }
}
