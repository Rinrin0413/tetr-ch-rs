//! A model for the ranks in TETRA LEAGUE.

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

/// A enum for the ranks in TETRA LEAGUE.
#[derive(Clone, Debug, Deserialize)]
pub enum Rank {
    /// D rank.
    #[serde(rename = "d")]
    D,
    /// D+ rank.
    #[serde(rename = "d+")]
    DPlus,
    /// C- rank.
    #[serde(rename = "c-")]
    CMinus,
    /// C rank.
    #[serde(rename = "c")]
    C,
    /// C+ rank.
    #[serde(rename = "c+")]
    CPlus,
    /// B- rank.
    #[serde(rename = "b-")]
    BMinus,
    /// B rank.
    #[serde(rename = "b")]
    B,
    /// B+ rank.
    #[serde(rename = "b+")]
    BPlus,
    /// A- rank.
    #[serde(rename = "a-")]
    AMinus,
    /// A rank.
    #[serde(rename = "a")]
    A,
    /// A+ rank.
    #[serde(rename = "a+")]
    APlus,
    /// S- rank.
    #[serde(rename = "s-")]
    SMinus,
    /// S rank.
    #[serde(rename = "s")]
    S,
    /// S+ rank.
    #[serde(rename = "s+")]
    SPlus,
    /// SS rank.
    #[serde(rename = "ss")]
    SS,
    /// U rank.
    #[serde(rename = "u")]
    U,
    /// X rank.
    #[serde(rename = "x")]
    X,
    /// X+ rank.
    #[serde(rename = "x+")]
    XPlus,
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
    /// # use tetr_ch::model::league_rank::Rank;
    /// assert_eq!(Rank::D.name(), "D");
    /// assert_eq!(Rank::DPlus.name(), "D+");
    /// assert_eq!(Rank::CMinus.name(), "C-");
    /// assert_eq!(Rank::C.name(), "C");
    /// assert_eq!(Rank::CPlus.name(), "C+");
    /// assert_eq!(Rank::BMinus.name(), "B-");
    /// assert_eq!(Rank::B.name(), "B");
    /// assert_eq!(Rank::BPlus.name(), "B+");
    /// assert_eq!(Rank::AMinus.name(), "A-");
    /// assert_eq!(Rank::A.name(), "A");
    /// assert_eq!(Rank::APlus.name(), "A+");
    /// assert_eq!(Rank::SMinus.name(), "S-");
    /// assert_eq!(Rank::S.name(), "S");
    /// assert_eq!(Rank::SPlus.name(), "S+");
    /// assert_eq!(Rank::SS.name(), "SS");
    /// assert_eq!(Rank::U.name(), "U");
    /// assert_eq!(Rank::X.name(), "X");
    /// assert_eq!(Rank::XPlus.name(), "X+");
    /// assert_eq!(Rank::Z.name(), "Unranked");
    /// ```
    pub fn name(&self) -> &str {
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
            Rank::XPlus => "X+",
            Rank::Z => "Unranked",
        }
    }

    /// Whether the rank is unranked (Z rank).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::league_rank::Rank;
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
    /// # use tetr_ch::model::league_rank::Rank;
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
    /// assert_eq!(Rank::XPlus.icon_url(), "https://tetr.io/res/league-ranks/x+.png");
    /// assert_eq!(Rank::Z.icon_url(), "https://tetr.io/res/league-ranks/z.png");
    /// ```
    pub fn icon_url(&self) -> String {
        format!("https://tetr.io/res/league-ranks/{}.png", self)
    }

    /// Returns the rank color (hex color code).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::league_rank::Rank;
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
    /// assert_eq!(Rank::XPlus.color(), 0xa763ea);
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
            Self::XPlus => Self::X_PLUS_COL,
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

    /// The X+ rank color.
    /// <span style="background-color:#a763ea;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#a763ea</span>
    pub const X_PLUS_COL: u32 = 0xa763ea;

    /// The unranked(Z rank) color.
    /// <span style="background-color:#767671;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#767671</span>
    pub const Z_COL: u32 = 0x767671;

    /// The XX rank color.
    /// <span style="background-color:#ff8fff;border-radius:8px;padding:2px;margin:8px;font-size:16px;border:1px solid black;color:black;">#ff8fff</span>
    #[deprecated(since = "0.6.0", note = "this is not official rank")]
    pub const XX_COL: u32 = 0xff8fff;
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
            Rank::XPlus => write!(f, "x+"),
            Rank::Z => write!(f, "z"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let rank_x_plus = Rank::XPlus;
        let rank_z = Rank::Z;
        assert_eq!(rank_d.name(), "D");
        assert_eq!(rank_d_plus.name(), "D+");
        assert_eq!(rank_c_minus.name(), "C-");
        assert_eq!(rank_c.name(), "C");
        assert_eq!(rank_c_plus.name(), "C+");
        assert_eq!(rank_b_minus.name(), "B-");
        assert_eq!(rank_b.name(), "B");
        assert_eq!(rank_b_plus.name(), "B+");
        assert_eq!(rank_a_minus.name(), "A-");
        assert_eq!(rank_a.name(), "A");
        assert_eq!(rank_a_plus.name(), "A+");
        assert_eq!(rank_s_minus.name(), "S-");
        assert_eq!(rank_s.name(), "S");
        assert_eq!(rank_s_plus.name(), "S+");
        assert_eq!(rank_ss.name(), "SS");
        assert_eq!(rank_u.name(), "U");
        assert_eq!(rank_x.name(), "X");
        assert_eq!(rank_x_plus.name(), "X+");
        assert_eq!(rank_z.name(), "Unranked");
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
        let rank_x_plus = Rank::XPlus;
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
        assert_eq!(rank_x_plus.color(), 0xa763ea);
        assert_eq!(rank_z.color(), 0x767671);
    }

    #[test]
    fn rank_as_ref() {
        let rank = Rank::C;
        let _a = rank.as_ref();
        let _b = rank;
    }
}
