//! User-related models.

use crate::{
    model::{cache::CacheData, record::SinglePlayRecord},
    util::{max_f64, to_unix_ts},
};
use serde::Deserialize;

/// The response for the User information.
/// Describes the user in detail.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested user data.
    pub data: Option<UserData>,
}

impl UserResponse {
    /// Returns UNIX timestamp when the user's account created, if one exists.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn account_created_at(&self) -> Option<i64> {
        match &self.get_user().ts {
            Some(ts) => Some(to_unix_ts(ts)),
            None => None,
        }
    }

    /// Returns the level based on the user's xp.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn level(&self) -> u32 {
        let xp = self.get_user().xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        let level =
            ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.)
                .floor() as u32;
        level
    }

    /// Returns the user's avatar URL.
    /// If the user has no avatar, returns anon's.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn face(&self) -> String {
        let default = "https://tetr.io/res/avatar.png".to_string();
        if let Some(ar) = self.get_user().avatar_revision {
            if ar == 0 {
                return default;
            }
            format!(
                "https://tetr.io/user-content/avatars/{}.jpg?rv={}",
                self.get_user()._id,
                ar
            )
        } else {
            default
        }
    }

    /// Returns the user's banner URL.
    /// If the user has no banner, returns `None`.
    ///
    /// ***Even if the user is not currently a supporter,
    /// the URL may be returned if the banner was once set.**
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn banner(&self) -> Option<String> {
        if let Some(br) = self.get_user().banner_revision {
            if br == 0 {
                return None;
            }
            Some(format!(
                "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                self.get_user()._id,
                br
            ))
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's rank.
    /// If the user is unranked, returns ?-rank(z) icon URL.
    /// If the user has no rank, returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn rank_icon_url(&self) -> Option<String> {
        if self.get_user().league.gamesplayed < 10 {
            Some(format!(
                "https://tetr.io/res/league-ranks/{}.png",
                self.get_user().league.rank
            ))
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's percentile rank.
    /// If not applicable, returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn percentile_rank_icon_url(&self) -> Option<String> {
        let pr = &self.get_user().league.percentile_rank;
        if pr != "z" {
            Some(format!("https://tetr.io/res/league-ranks/{}.png", pr))
        } else {
            None
        }
    }

    /// Returns an `Option<String>`.
    ///
    /// If user is displaying the country,
    /// returns `Some(String)` with an image URL of the national flag based on the user's ISO 3166-1 country code.
    /// If the user is not displaying the country, returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn national_flag_url(&self) -> Option<String> {
        if let Some(cc) = &self.get_user().country {
            Some(format!(
                "https://tetr.io/res/flags/{}.png",
                cc.to_lowercase()
            ))
        } else {
            None
        }
    }

    /// Returns the user's progress percentage in the rank.
    /// Returns `None` if there is no user's position in global leaderboards.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn rank_progress(&self) -> Option<f64> {
        let usr = self.get_user();
        let current_standing = usr.league.standing as f64;
        let prev_at = usr.league.prev_at as f64;
        let next_at = usr.league.next_at as f64;

        if prev_at < 0. || next_at < 0. {
            return None;
        }

        Some((current_standing - prev_at) / (next_at - prev_at) * 100.)
    }

    /// Returns the badges count.
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    pub fn badges_count(&self) -> usize {
        self.get_user().badges.len()
    }

    /// Returns a UNIX timestamp when this resource was cached.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_at(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_until(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns the [`&User`].
    ///
    /// [`&User`]: crate::model::user::User
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    fn get_user(&self) -> &User {
        match self.data.as_ref() {
            Some(d) => d.user.as_ref(),
            None => panic!("There is no user object because the request was not successful."),
        }
    }
}

impl AsRef<UserResponse> for UserResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The requested user data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserData {
    pub user: User,
}

impl AsRef<UserData> for UserData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The requested user.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// The user's internal ID.
    pub _id: String,
    /// The user's username.
    pub username: String,
    /// The user's role (one of `"anon"`, `"user"`, `"bot"`, `"mod"`, `"admin"`, *`"banned"`).  
    ///
    /// ***`"banned"` is not specified in TETRA CHANNEL API docs.**
    pub role: String,
    /// When the user account was created.
    /// If not set, this account was created before join dates were recorded.
    pub ts: Option<String>,
    /// If this user is a bot, the bot's operator.
    pub botmaster: Option<String>,
    /// The user's badges
    pub badges: Vec<Badge>,
    /// The user's XP in points.
    pub xp: f64,
    /// The amount of online games played by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    pub gamesplayed: i32,
    /// The amount of online games won by this user.
    /// If the user has chosen to hide this statistic, it will be -1.
    pub gameswon: i32,
    /// The amount of seconds this user spent playing, both on- and offline.
    /// If the user has chosen to hide this statistic, it will be -1.
    pub gametime: f64,
    /// The user's ISO 3166-1 country code, or `None` if hidden/unknown. Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user currently has a bad standing (recently banned).
    pub badstanding: Option<bool>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>, // EXCEPTION
    /// An indicator of their total amount supported, between 0 and 4 inclusive.
    pub supporter_tier: u8,
    /// Whether this user is a verified account.
    pub verified: bool,
    /// This user's current TETRA LEAGUE standing.
    pub league: LeagueData,
    /// This user's avatar ID.
    /// We can get their avatar at  
    /// `https://tetr.io/user-content/avatars/{ USERID }.jpg?rv={ AVATAR_REVISION }`.
    pub avatar_revision: Option<u64>,
    /// his user's banner ID.
    /// We can get their banner at  
    /// `https://tetr.io/user-content/banners/{ USERID }.jpg?rv={ BANNER_REVISION }`.  
    /// Ignore this field if the user is not a supporter.
    pub banner_revision: Option<u64>,
    /// This user's "About Me" section.
    /// Ignore this field if the user is not a supporter.
    ///
    /// ***Even if the user is not currently a supporter,
    /// the bio may be exist if the bio was once set.**
    pub bio: Option<String>,
    /// The amount of players who have added this user to their friends list.
    pub friend_count: Option<u32>, // EXCEPTION
}

impl User {
    /// Returns UNIX timestamp when the user's account created, if one exists.
    pub fn account_created_at(&self) -> Option<i64> {
        match &self.ts {
            Some(ts) => Some(to_unix_ts(ts)),
            None => None,
        }
    }

    /// Returns the level based on the user's xp.
    pub fn level(&self) -> u32 {
        let xp = self.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        let level =
            ((xp / 500.).powf(0.6) + (xp / (5000. + max_f64(0., xp - 4000000.) / 5000.)) + 1.)
                .floor() as u32;
        level
    }

    /// Returns the user's avatar URL.
    /// If the user has no avatar, returns anon's.
    pub fn face(&self) -> String {
        let default = "https://tetr.io/res/avatar.png".to_string();
        if let Some(ar) = self.avatar_revision {
            if ar == 0 {
                return default;
            }
            format!(
                "https://tetr.io/user-content/avatars/{}.jpg?rv={}",
                self._id, ar
            )
        } else {
            default
        }
    }

    /// Returns the user's banner URL.
    /// If the user has no banner, returns `None`.
    ///
    /// ***Even if the user is not currently a supporter,
    /// the URL may be returned if the banner was once set.**
    pub fn banner(&self) -> Option<String> {
        if let Some(br) = self.banner_revision {
            if br == 0 {
                return None;
            }
            Some(format!(
                "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                self._id, br
            ))
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's rank.
    /// If the user is unranked, returns ?-rank(z) icon URL.
    /// If the user has no rank, returns `None`.
    pub fn rank_icon_url(&self) -> Option<String> {
        if self.league.gamesplayed < 10 {
            Some(format!(
                "https://tetr.io/res/league-ranks/{}.png",
                self.league.rank
            ))
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's percentile rank.
    /// If not applicable, returns `None`.
    pub fn percentile_rank_icon_url(&self) -> Option<String> {
        let pr = &self.league.percentile_rank;
        if pr != "z" {
            Some(format!("https://tetr.io/res/league-ranks/{}.png", pr))
        } else {
            None
        }
    }

    /// Returns an `Option<String>`.
    ///
    /// If user is displaying the country,
    /// returns `Some(String)` with an image URL of the national flag based on the user's ISO 3166-1 country code.
    /// If the user is not displaying the country, returns `None`.
    pub fn national_flag_url(&self) -> Option<String> {
        if let Some(cc) = &self.country {
            Some(format!(
                "https://tetr.io/res/flags/{}.png",
                cc.to_lowercase()
            ))
        } else {
            None
        }
    }

    /// Returns the user's progress percentage in the rank.
    /// Returns `None` if there is no user's position in global leaderboards.
    pub fn rank_progress(&self) -> Option<f64> {
        let current_standing = self.league.standing as f64;
        let prev_at = self.league.prev_at as f64;
        let next_at = self.league.next_at as f64;

        if prev_at < 0. || next_at < 0. {
            return None;
        }

        Some((current_standing - prev_at) / (next_at - prev_at) * 100.)
    }

    /// Returns the badges count.
    pub fn badges_count(&self) -> usize {
        self.badges.len()
    }
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's badges.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Badge {
    /// The badge's internal ID,
    /// and the filename of the badge icon (all PNGs within `/res/badges/`)
    pub id: String,
    /// The badge's label, shown when hovered.
    pub label: String,
    /// The badge's timestamp, if shown.
    pub ts: Option<String>,
}

impl Badge {
    /// Returns the formatted badge icon URL.
    pub fn badge_icon_url(&self) -> String {
        format!("https://tetr.io/res/badges/{}.png", self.id)
    }

    /// Returns a UNIX timestamp when this badge was achieved.
    pub fn achieved_at(&self) -> Option<i64> {
        match &self.ts {
            Some(ts) => Some(to_unix_ts(ts)),
            None => None,
        }
    }
}

impl AsRef<Badge> for Badge {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's badges.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct LeagueData {
    /// The amount of TETRA LEAGUE games played by this user.
    pub gamesplayed: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    pub gameswon: u32,
    /// This user's TR (Tetra Rating), or -1 if less than 10 games were played.
    pub rating: f64,
    /// This user's letter rank. Z is unranked.
    pub rank: String,
    /// This user's position in global leaderboards, or -1 if not applicable.
    pub standing: i32,
    /// This user's position in local leaderboards, or -1 if not applicable.
    pub standing_local: i32,
    /// The next rank this user can achieve,
    /// if they win more games, or `None` if unranked (or the best rank).
    pub next_rank: Option<String>,
    /// The previous rank this user can achieve,
    /// if they lose more games, or `None` if unranked (or the worst rank).
    pub prev_rank: Option<String>,
    /// The position of the best player in the user's current rank, surpass them to go up a rank.
    /// -1 if unranked (or the best rank).
    pub next_at: i32,
    /// The position of the worst player in the user's current rank, dip below them to go down a rank.
    /// -1 if unranked (or the worst rank).
    pub prev_at: i32,
    /// This user's percentile position (0 is best, 1 is worst).
    pub percentile: f64,
    /// This user's percentile rank, or Z if not applicable.
    pub percentile_rank: String,
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
    pub decaying: bool,
}

impl LeagueData {
    /// Returns an icon URL of the user's rank.
    /// If the user is unranked, returns ?-rank(z) icon URL.
    /// If the user has no rank, returns `None`.
    pub fn rank_icon_url(&self) -> Option<String> {
        if self.gamesplayed < 10 {
            Some(format!(
                "https://tetr.io/res/league-ranks/{}.png",
                self.rank
            ))
        } else {
            None
        }
    }

    /// Returns an icon URL of the user's percentile rank.
    /// If not applicable, returns `None`.
    pub fn percentile_rank_icon_url(&self) -> Option<String> {
        let pr = &self.percentile_rank;
        if pr != "z" {
            Some(format!("https://tetr.io/res/league-ranks/{}.png", pr))
        } else {
            None
        }
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

/// The response for the user records.
/// Describes the user records.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct UserRecordsResponse {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<CacheData>,
    /// The requested user records data.
    pub data: Option<RecordsData>,
}

impl UserRecordsResponse {
    /// Returns the PPS(Pieces Per Second) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_pps(&self) -> f64 {
        self.get_40l_record().pps()
    }

    /// Returns the PPS(Pieces Per Second) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_pps(&self) -> f64 {
        self.get_blitz_record().pps()
    }

    /// Returns the KPP(Keys Per Piece) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_kpp(&self) -> f64 {
        self.get_40l_record().kpp()
    }

    /// Returns the KPP(Keys Per Piece) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_kpp(&self) -> f64 {
        self.get_blitz_record().kpp()
    }

    /// Returns the KPS(Keys Per Second) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_kps(&self) -> f64 {
        self.get_40l_record().kps()
    }

    /// Returns the KPS(Keys Per Second) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_kps(&self) -> f64 {
        self.get_blitz_record().kps()
    }

    /// Returns the LPM(Lines Per Minute) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_lpm(&self) -> f64 {
        self.get_40l_record().lpm()
    }

    /// Returns the LPM(Lines Per Minute) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_lpm(&self) -> f64 {
        self.get_blitz_record().lpm()
    }

    /// Returns the SPP(Score Per Piece) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_spp(&self) -> f64 {
        self.get_40l_record().spp()
    }

    /// Returns the SPP(Score Per Piece) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_spp(&self) -> f64 {
        self.get_blitz_record().spp()
    }

    /// Returns the finesse rate of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_finesse_rate(&self) -> f64 {
        self.get_40l_record().finesse_rate()
    }

    /// Returns the finesse rate of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_finesse_rate(&self) -> f64 {
        self.get_blitz_record().finesse_rate()
    }

    /// Returns the 40 LINES record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_record_url(&self) -> String {
        self.get_40l_record().record_url()
    }
    /// Returns the BLITZ record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    pub fn blitz_record_url(&self) -> String {
        self.get_blitz_record().record_url()
    }

    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    pub fn forty_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_40l_record().ts)
    }
    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    pub fn blitz_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_blitz_record().ts)
    }

    /// Returns a UNIX timestamp when this resource was cached.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_at(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns a UNIX timestamp when this resource's cache expires.
    ///
    /// # Panics
    ///
    /// Panics if there is no cache data.
    pub fn cached_until(&self) -> i64 {
        match self.cache.as_ref() {
            Some(c) => c.cached_at(),
            None => panic!("There is no cache data."),
        }
    }

    /// Returns the [`&RecordsData`].
    ///
    /// [`&RecordsData`]: crate::model::user::RecordsData
    ///
    /// # Panics
    ///
    /// Panics if the request was not successful.
    fn get_user_records(&self) -> &RecordsData {
        if let Some(d) = self.data.as_ref() {
            d
        } else {
            panic!("There is no user records object because the request was not successful.")
        }
    }

    /// Returns the [`&SinglePlayRecord`] for 40 LINES.
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record,
    /// or the request was not successful.
    fn get_40l_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.get_user_records().records.forty_lines.record.as_ref() {
            d
        } else {
            panic!("There is no 40 LINES record.")
        }
    }

    /// Returns the [`&SinglePlayRecord`] for BLITZ.
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record,
    /// or the request was not successful.
    fn get_blitz_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.get_user_records().records.blitz.record.as_ref() {
            d
        } else {
            panic!("There is no BLITZ record.")
        }
    }
}

impl AsRef<UserRecordsResponse> for UserRecordsResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The requested user records data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct RecordsData {
    /// The requested user's ranked records.
    pub records: Records,
    /// The user's ZEN record.
    pub zen: Zen,
}

impl RecordsData {
    /// Returns the PPS(Pieces Per Second) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_pps(&self) -> f64 {
        self.get_40l_record().pps()
    }

    /// Returns the PPS(Pieces Per Second) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_pps(&self) -> f64 {
        self.get_blitz_record().pps()
    }

    /// Returns the KPP(Keys Per Piece) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_kpp(&self) -> f64 {
        self.get_40l_record().kpp()
    }

    /// Returns the KPP(Keys Per Piece) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_kpp(&self) -> f64 {
        self.get_blitz_record().kpp()
    }

    /// Returns the KPS(Keys Per Second) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_kps(&self) -> f64 {
        self.get_40l_record().kps()
    }

    /// Returns the KPS(Keys Per Second) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_kps(&self) -> f64 {
        self.get_blitz_record().kps()
    }

    /// Returns the LPM(Lines Per Minute) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_lpm(&self) -> f64 {
        self.get_40l_record().lpm()
    }

    /// Returns the LPM(Lines Per Minute) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_lpm(&self) -> f64 {
        self.get_blitz_record().lpm()
    }

    /// Returns the SPP(Score Per Piece) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_spp(&self) -> f64 {
        self.get_40l_record().spp()
    }

    /// Returns the SPP(Score Per Piece) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_spp(&self) -> f64 {
        self.get_blitz_record().spp()
    }

    /// Returns the finesse rate of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_finesse_rate(&self) -> f64 {
        self.get_40l_record().finesse_rate()
    }

    /// Returns the finesse rate of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_finesse_rate(&self) -> f64 {
        self.get_blitz_record().finesse_rate()
    }

    /// Returns the 40 LINES record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_record_url(&self) -> String {
        self.get_40l_record().record_url()
    }

    /// Returns the BLITZ record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_record_url(&self) -> String {
        self.get_blitz_record().record_url()
    }

    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_40l_record().ts)
    }
    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_blitz_record().ts)
    }

    /// Returns the [`&SinglePlayRecord`] for 40 LINES..
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    fn get_40l_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.records.forty_lines.record.as_ref() {
            d
        } else {
            panic!("There is no 40 LINES record.")
        }
    }

    /// Returns the [`&SinglePlayRecord`] for BLITZ.
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    fn get_blitz_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.records.blitz.record.as_ref() {
            d
        } else {
            panic!("There is no BLITZ record.")
        }
    }
}

impl AsRef<RecordsData> for RecordsData {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The requested user's ranked records.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Records {
    /// The user's 40 LINES record.
    #[serde(rename = "40l")]
    pub forty_lines: FortyLines,
    /// The user's BLITZ record.
    pub blitz: Blitz,
}

impl Records {
    /// Returns the PPS(Pieces Per Second) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_pps(&self) -> f64 {
        self.get_40l_record().pps()
    }

    /// Returns the PPS(Pieces Per Second) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_pps(&self) -> f64 {
        self.get_blitz_record().pps()
    }

    /// Returns the KPP(Keys Per Piece) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_kpp(&self) -> f64 {
        self.get_40l_record().kpp()
    }

    /// Returns the KPP(Keys Per Piece) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_kpp(&self) -> f64 {
        self.get_blitz_record().kpp()
    }

    /// Returns the KPS(Keys Per Second) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_kps(&self) -> f64 {
        self.get_40l_record().kps()
    }

    /// Returns the KPS(Keys Per Second) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_kps(&self) -> f64 {
        self.get_blitz_record().kps()
    }

    /// Returns the LPM(Lines Per Minute) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_lpm(&self) -> f64 {
        self.get_40l_record().lpm()
    }

    /// Returns the LPM(Lines Per Minute) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_lpm(&self) -> f64 {
        self.get_blitz_record().lpm()
    }

    /// Returns the SPP(Score Per Piece) of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_spp(&self) -> f64 {
        self.get_40l_record().spp()
    }

    /// Returns the SPP(Score Per Piece) of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_spp(&self) -> f64 {
        self.get_blitz_record().spp()
    }

    /// Returns the finesse rate of 40 LINES.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES.
    pub fn forty_lines_finesse_rate(&self) -> f64 {
        self.get_40l_record().finesse_rate()
    }

    /// Returns the finesse rate of BLITZ.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_finesse_rate(&self) -> f64 {
        self.get_blitz_record().finesse_rate()
    }

    /// Returns the 40 LINES record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_record_url(&self) -> String {
        self.get_40l_record().record_url()
    }

    /// Returns the BLITZ record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_record_url(&self) -> String {
        self.get_blitz_record().record_url()
    }

    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_40l_record().ts)
    }

    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn blitz_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_blitz_record().ts)
    }

    /// Returns the [`&SinglePlayRecord`] for 40 LINES..
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    fn get_40l_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.forty_lines.record.as_ref() {
            d
        } else {
            panic!("There is no 40 LINES record.")
        }
    }

    /// Returns the [`&SinglePlayRecord`] for BLITZ.
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    fn get_blitz_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.blitz.record.as_ref() {
            d
        } else {
            panic!("There is no BLITZ record.")
        }
    }
}

impl AsRef<Records> for Records {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's 40 LINES record.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct FortyLines {
    /// The user's 40 LINES record data, or `None` if never played.
    pub record: Option<SinglePlayRecord>,
    /// The user's rank in global leaderboards,
    /// or `None` if not in global leaderboards.
    pub rank: Option<u32>,
}

impl FortyLines {
    /// Returns the PPS(Pieces Per Second) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn pps(&self) -> f64 {
        self.get_40l_record().pps()
    }

    /// Returns the KPP(Keys Per Piece) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn kpp(&self) -> f64 {
        self.get_40l_record().kpp()
    }

    /// Returns the KPS(Keys Per Second) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn kps(&self) -> f64 {
        self.get_40l_record().kps()
    }

    /// Returns the LPM(Lines Per Minute) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn lpm(&self) -> f64 {
        self.get_40l_record().lpm()
    }

    /// Returns the SPP(Score Per Piece) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn spp(&self) -> f64 {
        self.get_40l_record().spp()
    }

    /// Returns the finesse rate of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn finesse_rate(&self) -> f64 {
        self.get_40l_record().finesse_rate()
    }

    /// Returns the 40 LINES record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_record_url(&self) -> String {
        self.get_40l_record().record_url()
    }

    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    pub fn forty_lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_40l_record().ts)
    }

    /// Returns the [`&SinglePlayRecord`] for 40 LINES..
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no 40 LINES record.
    fn get_40l_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.record.as_ref() {
            d
        } else {
            panic!("There is no 40 LINES record.")
        }
    }
}

impl AsRef<FortyLines> for FortyLines {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's BLITZ record.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Blitz {
    /// The user's BLITZ record data, or `None` if never played.
    pub record: Option<SinglePlayRecord>,
    /// The user's rank in global leaderboards,
    /// or `None` if not in global leaderboards.
    pub rank: Option<u32>,
}

impl Blitz {
    /// Returns the PPS(Pieces Per Second) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn pps(&self) -> f64 {
        self.get_blitz_record().pps()
    }

    /// Returns the KPP(Keys Per Piece) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn kpp(&self) -> f64 {
        self.get_blitz_record().kpp()
    }

    /// Returns the KPS(Keys Per Second) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn kps(&self) -> f64 {
        self.get_blitz_record().kps()
    }

    /// Returns the LPM(Lines Per Minute) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn lpm(&self) -> f64 {
        self.get_blitz_record().lpm()
    }

    /// Returns the SPP(Score Per Piece) of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn spp(&self) -> f64 {
        self.get_blitz_record().spp()
    }

    /// Returns the finesse rate of this replay.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn finesse_rate(&self) -> f64 {
        self.get_blitz_record().finesse_rate()
    }

    /// Returns the BLITZ record URL.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn record_url(&self) -> String {
        self.get_blitz_record().record_url()
    }

    /// Returns a UNIX timestamp when this record was recorded.
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    pub fn lines_recorded_at(&self) -> i64 {
        to_unix_ts(&self.get_blitz_record().ts)
    }

    /// Returns the [`&SinglePlayRecord`] for BLITZ.
    ///
    /// [`&SinglePlayRecord`]: crate::model::record::SinglePlayRecord
    ///
    /// # Panics
    ///
    /// Panics if there is no BLITZ record.
    fn get_blitz_record(&self) -> &SinglePlayRecord {
        if let Some(d) = self.record.as_ref() {
            d
        } else {
            panic!("There is no BLITZ record.")
        }
    }
}

impl AsRef<Blitz> for Blitz {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// The user's ZEN record.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Zen {
    /// The user's level in ZEN mode.
    pub level: u32,
    /// The user's score in ZEN mode.
    pub score: u64,
}
