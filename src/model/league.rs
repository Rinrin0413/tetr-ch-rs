//! TETRA LEAGUE related objects.

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

/// The user's TETRA LEAGUE data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueData {
    /// The amount of TETRA LEAGUE games played by this user.
    #[serde(rename = "gamesplayed")]
    pub play_count: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    #[serde(rename = "gameswon")]
    pub win_count: u32,
    /// This user's TR (Tetra Rating), or -1 if less than 10 games were played.
    pub rating: f64,
    /// This user's letter rank. Z is unranked.
    pub rank: Rank,
    /// This user's highest achieved rank this season.
    #[serde(rename = "bestrank")]
    pub best_rank: Option<Rank>,
    /// This user's position in global leaderboards, or -1 if not applicable.
    pub standing: i32,
    /// This user's position in local leaderboards, or -1 if not applicable.
    pub standing_local: i32,
    /// The next rank this user can achieve,
    /// if they win more games, or `None` if unranked (or the best rank).
    pub next_rank: Option<Rank>,
    /// The previous rank this user can achieve,
    /// if they lose more games, or `None` if unranked (or the worst rank).
    pub prev_rank: Option<Rank>,
    /// The position of the best player in the user's current rank, surpass them to go up a rank.
    /// -1 if unranked (or the best rank).
    pub next_at: i32,
    /// The position of the worst player in the user's current rank, dip below them to go down a rank.
    /// -1 if unranked (or the worst rank).
    pub prev_at: i32,
    /// This user's percentile position (0 is best, 1 is worst).
    pub percentile: f64,
    /// This user's percentile rank, or Z if not applicable.
    pub percentile_rank: Rank,
    /// This user's Glicko-2 rating.
    pub glicko: Option<f64>,
    /// This user's Glicko-2 Rating Deviation.
    /// If over 100, this user is unranked.
    pub rd: Option<f64>,
    /// This user's average APM (attack per minute) over the last 10 games.
    pub apm: Option<f64>,
    /// This user's average PPS (pieces per second) over the last 10 games.
    pub pps: Option<f64>,
    /// This user's average VS (versus score) over the last 10 games.
    pub vs: Option<f64>,
    /// Whether this user's RD is rising (has not played in the last week).
    #[serde(rename = "decaying")]
    pub is_decaying: bool,
}

impl LeagueData {
    /// Returns an icon URL of the user's rank.
    /// If the user is unranked, returns ?-rank(z) icon URL.
    /// If the user has no rank, returns `None`.
    pub fn rank_icon_url(&self) -> Option<String> {
        if 10 <= self.play_count {
            Some(self.rank.icon_url())
        } else {
            None
        }
    }

    /// Returns a rank color. (Hex color codes)
    /// If the user has no rank, returns `None`.
    pub fn rank_color(&self) -> Option<u32> {
        if 10 <= self.play_count {
            Some(self.rank.color())
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's percentile rank.
    /// If not applicable, returns `None`.
    pub fn percentile_rank_icon_url(&self) -> Option<String> {
        let pr = &self.percentile_rank;
        if !pr.is_unranked() {
            Some(pr.icon_url())
        } else {
            None
        }
    }

    /// Returns a percentile rank color. (Hex color codes)
    /// If not applicable, returns `None`.
    pub fn percentile_rank_color(&self) -> Option<u32> {
        let pr = &self.percentile_rank;
        if !pr.is_unranked() {
            Some(pr.color())
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's highest achieved rank.
    /// If the user has no highest achieved rank, returns `None`.
    pub fn best_rank_icon_url(&self) -> Option<String> {
        self.best_rank.as_ref().map(|br| br.icon_url())
    }

    /// Returns a highest achieved rank color. (Hex color codes)
    /// If the user has no highest achieved rank, returns `None`.
    pub fn best_rank_color(&self) -> Option<u32> {
        self.best_rank.as_ref().map(|br| br.color())
    }

    /// Returns the user's progress percentage in the rank.
    /// Returns `None` if there is no user's position in global leaderboards.
    pub fn rank_progress(&self) -> Option<f64> {
        let current_standing = self.standing as f64;
        let prev_at = self.prev_at as f64;
        let next_at = self.next_at as f64;

        if prev_at < 0. || next_at < 0. {
            return None;
        }

        Some((current_standing - prev_at) / (next_at - prev_at) * 100.)
    }
}

impl AsRef<LeagueData> for LeagueData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The TETRA LEAGUE rank.
#[derive(Clone, Debug, Deserialize)]
pub enum Rank {
    /// The D rank.
    #[serde(rename = "d")]
    D,
    /// The D+ rank.
    #[serde(rename = "d+")]
    DPlus,
    /// The C- rank.
    #[serde(rename = "c-")]
    CMinus,
    /// The C rank.
    #[serde(rename = "c")]
    C,
    /// The C+ rank.
    #[serde(rename = "c+")]
    CPlus,
    /// The B- rank.
    #[serde(rename = "b-")]
    BMinus,
    /// The B rank.
    #[serde(rename = "b")]
    B,
    /// The B+ rank.
    #[serde(rename = "b+")]
    BPlus,
    /// The A- rank.
    #[serde(rename = "a-")]
    AMinus,
    /// The A rank.
    #[serde(rename = "a")]
    A,
    /// The A+ rank.
    #[serde(rename = "a+")]
    APlus,
    /// The S- rank.
    #[serde(rename = "s-")]
    SMinus,
    /// The S rank.
    #[serde(rename = "s")]
    S,
    /// The S+ rank.
    #[serde(rename = "s+")]
    SPlus,
    /// The SS rank.
    #[serde(rename = "ss")]
    SS,
    /// The U rank.
    #[serde(rename = "u")]
    U,
    /// The X rank.
    #[serde(rename = "x")]
    X,
    /// Unranked.
    #[serde(rename = "z")]
    Z,
}

impl Rank {
    /// Returns the rank's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::league::Rank;
    /// assert_eq!(Rank::D.as_str(), "D");
    /// assert_eq!(Rank::DPlus.as_str(), "D+");
    /// assert_eq!(Rank::CMinus.as_str(), "C-");
    /// assert_eq!(Rank::C.as_str(), "C");
    /// assert_eq!(Rank::CPlus.as_str(), "C+");
    /// assert_eq!(Rank::BMinus.as_str(), "B-");
    /// assert_eq!(Rank::B.as_str(), "B");
    /// assert_eq!(Rank::BPlus.as_str(), "B+");
    /// assert_eq!(Rank::AMinus.as_str(), "A-");
    /// assert_eq!(Rank::A.as_str(), "A");
    /// assert_eq!(Rank::APlus.as_str(), "A+");
    /// assert_eq!(Rank::SMinus.as_str(), "S-");
    /// assert_eq!(Rank::S.as_str(), "S");
    /// assert_eq!(Rank::SPlus.as_str(), "S+");
    /// assert_eq!(Rank::SS.as_str(), "SS");
    /// assert_eq!(Rank::U.as_str(), "U");
    /// assert_eq!(Rank::X.as_str(), "X");
    /// assert_eq!(Rank::Z.as_str(), "Unranked");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Rank::D => "D",
            Rank::DPlus => "D+",
            Rank::CMinus => "C-",
            Rank::C => "C",
            Rank::CPlus => "C+",
            Rank::BMinus => "B-",
            Rank::B => "B",
            Rank::BPlus => "B+",
            Rank::AMinus => "A-",
            Rank::A => "A",
            Rank::APlus => "A+",
            Rank::SMinus => "S-",
            Rank::S => "S",
            Rank::SPlus => "S+",
            Rank::SS => "SS",
            Rank::U => "U",
            Rank::X => "X",
            Rank::Z => "Unranked",
        }
    }

    /// Whether the rank is unranked(Z).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::league::Rank;
    /// assert!(!Rank::D.is_unranked());
    /// assert!(!Rank::A.is_unranked());
    /// assert!(!Rank::X.is_unranked());
    /// assert!(Rank::Z.is_unranked());
    /// ```
    pub fn is_unranked(&self) -> bool {
        matches!(self, Rank::Z)
    }

    /// Returns the URL of the rank icon.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::league::Rank;
    /// assert_eq!(Rank::D.icon_url(), "https://tetr.io/res/league-ranks/d.png");
    /// assert_eq!(Rank::DPlus.icon_url(), "https://tetr.io/res/league-ranks/d+.png");
    /// assert_eq!(Rank::CMinus.icon_url(), "https://tetr.io/res/league-ranks/c-.png");
    /// assert_eq!(Rank::C.icon_url(), "https://tetr.io/res/league-ranks/c.png");
    /// assert_eq!(Rank::CPlus.icon_url(), "https://tetr.io/res/league-ranks/c+.png");
    /// assert_eq!(Rank::BMinus.icon_url(), "https://tetr.io/res/league-ranks/b-.png");
    /// assert_eq!(Rank::B.icon_url(), "https://tetr.io/res/league-ranks/b.png");
    /// assert_eq!(Rank::BPlus.icon_url(), "https://tetr.io/res/league-ranks/b+.png");
    /// assert_eq!(Rank::AMinus.icon_url(), "https://tetr.io/res/league-ranks/a-.png");
    /// assert_eq!(Rank::A.icon_url(), "https://tetr.io/res/league-ranks/a.png");
    /// assert_eq!(Rank::APlus.icon_url(), "https://tetr.io/res/league-ranks/a+.png");
    /// assert_eq!(Rank::SMinus.icon_url(), "https://tetr.io/res/league-ranks/s-.png");
    /// assert_eq!(Rank::S.icon_url(), "https://tetr.io/res/league-ranks/s.png");
    /// assert_eq!(Rank::SPlus.icon_url(), "https://tetr.io/res/league-ranks/s+.png");
    /// assert_eq!(Rank::SS.icon_url(), "https://tetr.io/res/league-ranks/ss.png");
    /// assert_eq!(Rank::U.icon_url(), "https://tetr.io/res/league-ranks/u.png");
    /// assert_eq!(Rank::X.icon_url(), "https://tetr.io/res/league-ranks/x.png");
    /// assert_eq!(Rank::Z.icon_url(), "https://tetr.io/res/league-ranks/z.png");
    /// ```
    pub fn icon_url(&self) -> String {
        format!("https://tetr.io/res/league-ranks/{}.png", self)
    }

    /// Returns the rank color. (Hex color codes)
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::league::Rank;
    /// assert_eq!(Rank::D.color(), 0x907591);
    /// assert_eq!(Rank::DPlus.color(), 0x8e6091);
    /// assert_eq!(Rank::CMinus.color(), 0x79558c);
    /// assert_eq!(Rank::C.color(), 0x733e8f);
    /// assert_eq!(Rank::CPlus.color(), 0x552883);
    /// assert_eq!(Rank::BMinus.color(), 0x5650c7);
    /// assert_eq!(Rank::B.color(), 0x4f64c9);
    /// assert_eq!(Rank::BPlus.color(), 0x4f99c0);
    /// assert_eq!(Rank::AMinus.color(), 0x3bb687);
    /// assert_eq!(Rank::A.color(), 0x46ad51);
    /// assert_eq!(Rank::APlus.color(), 0x46ad51);
    /// assert_eq!(Rank::SMinus.color(), 0xB2972B);
    /// assert_eq!(Rank::S.color(), 0xE0A71B);
    /// assert_eq!(Rank::SPlus.color(), 0xD8AF0E);
    /// assert_eq!(Rank::SS.color(), 0xDB8B1F);
    /// assert_eq!(Rank::U.color(), 0xFF3813);
    /// assert_eq!(Rank::X.color(), 0xff45ff);
    /// assert_eq!(Rank::Z.color(), 0x767671);
    /// ```
    pub fn color(&self) -> u32 {
        match self {
            Self::D => Self::D_COL,
            Self::DPlus => Self::D_PLUS_COL,
            Self::CMinus => Self::C_MINUS_COL,
            Self::C => Self::C_COL,
            Self::CPlus => Self::C_PLUS_COL,
            Self::BMinus => Self::B_MINUS_COL,
            Self::B => Self::B_COL,
            Self::BPlus => Self::B_PLUS_COL,
            Self::AMinus => Self::A_MINUS_COL,
            Self::A => Self::A_COL,
            Self::APlus => Self::A_PLUS_COL,
            Self::SMinus => Self::S_MINUS_COL,
            Self::S => Self::S_COL,
            Self::SPlus => Self::S_PLUS_COL,
            Self::SS => Self::SS_COL,
            Self::U => Self::U_COL,
            Self::X => Self::X_COL,
            Self::Z => Self::Z_COL,
        }
    }

    /// The D rank color.
    /// <span style="background-color:#907591;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#907591</span>
    pub const D_COL: u32 = 0x907591;

    /// The D+ rank color.
    /// <span style="background-color:#8e6091;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#8e6091</span>
    pub const D_PLUS_COL: u32 = 0x8e6091;

    /// The C- rank color.
    /// <span style="background-color:#79558c;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#79558c</span>
    pub const C_MINUS_COL: u32 = 0x79558c;

    /// The C rank color.
    /// <span style="background-color:#733e8f;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#733e8f</span>
    pub const C_COL: u32 = 0x733e8f;

    /// The C+ rank color.
    /// <span style="background-color:#552883;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#552883</span>
    pub const C_PLUS_COL: u32 = 0x552883;

    /// The B- rank color.
    /// <span style="background-color:#5650c7;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#5650c7</span>
    pub const B_MINUS_COL: u32 = 0x5650c7;

    /// The B rank color.
    /// <span style="background-color:#4f64c9;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#4f64c9</span>
    pub const B_COL: u32 = 0x4f64c9;

    /// The B+ rank color.
    /// <span style="background-color:#4f99c0;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#4f99c0</span>
    pub const B_PLUS_COL: u32 = 0x4f99c0;

    /// The A- rank color.
    /// <span style="background-color:#3bb687;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#3bb687</span>
    pub const A_MINUS_COL: u32 = 0x3bb687;

    /// The A rank color.
    /// <span style="background-color:#46ad51;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#46ad51</span>
    pub const A_COL: u32 = 0x46ad51;

    /// The A+ rank color.
    /// <span style="background-color:#1fa834;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#1fa834</span>
    pub const A_PLUS_COL: u32 = 0x46ad51;

    /// The S- rank color.
    /// <span style="background-color:#b2972b;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#b2972b</span>
    pub const S_MINUS_COL: u32 = 0xb2972b;

    /// The S rank color.
    /// <span style="background-color:#e0a71b;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#e0a71b</span>
    pub const S_COL: u32 = 0xe0a71b;

    /// The S+ rank color.
    /// <span style="background-color:#d8af0e;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#d8af0e</span>
    pub const S_PLUS_COL: u32 = 0xd8af0e;

    /// The SS rank color.
    /// <span style="background-color:#db8b1f;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#db8b1f</span>
    pub const SS_COL: u32 = 0xdb8b1f;

    /// The U rank color.
    /// <span style="background-color:#ff3813;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#ff3813</span>
    pub const U_COL: u32 = 0xff3813;

    /// The X rank color.
    /// <span style="background-color:#ff45ff;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#ff45ff</span>
    pub const X_COL: u32 = 0xff45ff;

    /// The XX rank color.
    /// <span style="background-color:#ff8fff;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#ff8fff</span>
    pub const XX_COL: u32 = 0xff8fff;

    /// The unranked(Z rank) color.
    /// <span style="background-color:#767671;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#767671</span>
    pub const Z_COL: u32 = 0x767671;
}

impl AsRef<Rank> for Rank {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Rank::D => write!(f, "d"),
            Rank::DPlus => write!(f, "d+"),
            Rank::CMinus => write!(f, "c-"),
            Rank::C => write!(f, "c"),
            Rank::CPlus => write!(f, "c+"),
            Rank::BMinus => write!(f, "b-"),
            Rank::B => write!(f, "b"),
            Rank::BPlus => write!(f, "b+"),
            Rank::AMinus => write!(f, "a-"),
            Rank::A => write!(f, "a"),
            Rank::APlus => write!(f, "a+"),
            Rank::SMinus => write!(f, "s-"),
            Rank::S => write!(f, "s"),
            Rank::SPlus => write!(f, "s+"),
            Rank::SS => write!(f, "ss"),
            Rank::U => write!(f, "u"),
            Rank::X => write!(f, "x"),
            Rank::Z => write!(f, "z"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rank_icon_url_from_league_data() {
        let league_data_ranked = LeagueData {
            rank: Rank::D,
            play_count: 10,
            ..default_league_data()
        };
        let league_data_unranked = LeagueData {
            play_count: 9,
            ..default_league_data()
        };
        assert_eq!(
            league_data_ranked.rank_icon_url(),
            Some("https://tetr.io/res/league-ranks/d.png".to_string())
        );
        assert_eq!(league_data_unranked.rank_icon_url(), None);
    }

    #[test]
    fn get_rank_color_from_league_data() {
        let league_data = LeagueData {
            rank: Rank::A,
            play_count: 10,
            ..default_league_data()
        };
        assert_eq!(league_data.rank_color(), Some(0x46ad51));
    }

    #[test]
    fn get_percentile_rank_icon_url_from_league_data() {
        let league_data_ranked = LeagueData {
            percentile_rank: Rank::X,
            ..default_league_data()
        };
        let league_data_unranked = LeagueData {
            percentile_rank: Rank::Z,
            ..default_league_data()
        };
        assert_eq!(
            league_data_ranked.percentile_rank_icon_url(),
            Some("https://tetr.io/res/league-ranks/x.png".to_string())
        );
        assert_eq!(league_data_unranked.percentile_rank_icon_url(), None);
    }

    #[test]
    fn get_percentile_rank_color_from_league_data() {
        let league_data = LeagueData {
            percentile_rank: Rank::DPlus,
            ..default_league_data()
        };
        assert_eq!(league_data.percentile_rank_color(), Option::Some(0x8e6091));
    }

    #[test]
    fn get_rank_progress() {
        let league_data_unranked = LeagueData {
            prev_at: -1,
            next_at: -1,
            ..default_league_data()
        };
        let league_data_ranked = LeagueData {
            standing: 600,
            prev_at: 2000,
            next_at: 400,
            ..default_league_data()
        };
        assert_eq!(league_data_unranked.rank_progress(), None);
        assert_eq!(league_data_ranked.rank_progress(), Some(87.5));
    }

    #[test]
    fn league_data_as_ref() {
        let league_data = default_league_data();
        let _a = league_data.as_ref();
        let _b = league_data;
    }

    #[test]
    fn ranks_as_str() {
        let rank_d = Rank::D;
        let rank_d_plus = Rank::DPlus;
        let rank_c_minus = Rank::CMinus;
        let rank_c = Rank::C;
        let rank_c_plus = Rank::CPlus;
        let rank_b_minus = Rank::BMinus;
        let rank_b = Rank::B;
        let rank_b_plus = Rank::BPlus;
        let rank_a_minus = Rank::AMinus;
        let rank_a = Rank::A;
        let rank_a_plus = Rank::APlus;
        let rank_s_minus = Rank::SMinus;
        let rank_s = Rank::S;
        let rank_s_plus = Rank::SPlus;
        let rank_ss = Rank::SS;
        let rank_u = Rank::U;
        let rank_x = Rank::X;
        let rank_z = Rank::Z;
        assert_eq!(rank_d.as_str(), "D");
        assert_eq!(rank_d_plus.as_str(), "D+");
        assert_eq!(rank_c_minus.as_str(), "C-");
        assert_eq!(rank_c.as_str(), "C");
        assert_eq!(rank_c_plus.as_str(), "C+");
        assert_eq!(rank_b_minus.as_str(), "B-");
        assert_eq!(rank_b.as_str(), "B");
        assert_eq!(rank_b_plus.as_str(), "B+");
        assert_eq!(rank_a_minus.as_str(), "A-");
        assert_eq!(rank_a.as_str(), "A");
        assert_eq!(rank_a_plus.as_str(), "A+");
        assert_eq!(rank_s_minus.as_str(), "S-");
        assert_eq!(rank_s.as_str(), "S");
        assert_eq!(rank_s_plus.as_str(), "S+");
        assert_eq!(rank_ss.as_str(), "SS");
        assert_eq!(rank_u.as_str(), "U");
        assert_eq!(rank_x.as_str(), "X");
        assert_eq!(rank_z.as_str(), "Unranked");
    }

    #[test]
    fn whether_rank_is_unranked() {
        let ranked_rank = Rank::CMinus;
        let unranked_rank = Rank::Z;
        assert!(!ranked_rank.is_unranked());
        assert!(unranked_rank.is_unranked());
    }

    #[test]
    fn get_rank_icon_url() {
        let rank = Rank::SS;
        assert_eq!(
            rank.icon_url(),
            "https://tetr.io/res/league-ranks/ss.png".to_string()
        );
    }

    #[test]
    fn get_ranks_color() {
        let rank_d = Rank::D;
        let rank_d_plus = Rank::DPlus;
        let rank_c_minus = Rank::CMinus;
        let rank_c = Rank::C;
        let rank_c_plus = Rank::CPlus;
        let rank_b_minus = Rank::BMinus;
        let rank_b = Rank::B;
        let rank_b_plus = Rank::BPlus;
        let rank_a_minus = Rank::AMinus;
        let rank_a = Rank::A;
        let rank_a_plus = Rank::APlus;
        let rank_s_minus = Rank::SMinus;
        let rank_s = Rank::S;
        let rank_s_plus = Rank::SPlus;
        let rank_ss = Rank::SS;
        let rank_u = Rank::U;
        let rank_x = Rank::X;
        let rank_z = Rank::Z;
        assert_eq!(rank_d.color(), 0x907591);
        assert_eq!(rank_d_plus.color(), 0x8e6091);
        assert_eq!(rank_c_minus.color(), 0x79558c);
        assert_eq!(rank_c.color(), 0x733e8f);
        assert_eq!(rank_c_plus.color(), 0x552883);
        assert_eq!(rank_b_minus.color(), 0x5650c7);
        assert_eq!(rank_b.color(), 0x4f64c9);
        assert_eq!(rank_b_plus.color(), 0x4f99c0);
        assert_eq!(rank_a_minus.color(), 0x3bb687);
        assert_eq!(rank_a.color(), 0x46ad51);
        assert_eq!(rank_a_plus.color(), 0x46ad51);
        assert_eq!(rank_s_minus.color(), 0xb2972b);
        assert_eq!(rank_s.color(), 0xe0a71b);
        assert_eq!(rank_s_plus.color(), 0xd8af0e);
        assert_eq!(rank_ss.color(), 0xdb8b1f);
        assert_eq!(rank_u.color(), 0xff3813);
        assert_eq!(rank_x.color(), 0xff45ff);
        assert_eq!(rank_z.color(), 0x767671);
    }

    #[test]
    fn rank_as_ref() {
        let rank = Rank::C;
        let _a = rank.as_ref();
        let _b = rank;
    }

    fn default_league_data() -> LeagueData {
        LeagueData {
            play_count: 0,
            win_count: 0,
            rating: 0.,
            rank: Rank::Z,
            best_rank: None,
            standing: 0,
            standing_local: 0,
            next_rank: Some(Rank::Z),
            prev_rank: Some(Rank::Z),
            next_at: 0,
            prev_at: 0,
            percentile: 0.,
            percentile_rank: Rank::Z,
            glicko: Some(0.),
            rd: Some(0.),
            apm: Some(0.),
            pps: Some(0.),
            vs: Some(0.),
            is_decaying: false,
        }
    }
}
