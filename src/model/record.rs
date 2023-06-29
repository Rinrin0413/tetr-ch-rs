//! The record data.

use crate::{model::user::UserId, util::to_unix_ts};
use serde::Deserialize;

/// The record data.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Record {
    /// The Record's ID.
    /// This is NOT the replay ID.
    #[serde(rename = "_id")]
    pub record_id: String,
    /// The Stream this Record belongs to.
    pub stream: String,
    /// The ID of the associated replay.
    /// This is NOT the Record's ID.
    #[serde(rename = "replayid")]
    pub replay_id: String,
    /// The user who set this Record.
    pub user: User,
    /// The time this record was set.
    #[serde(rename = "ts")]
    pub recorded_at: String,
    /// Whether this is a multiplayer replay.
    #[serde(rename = "ismulti")]
    pub is_multi: Option<bool>,
    /// The state this replay finished with.
    pub endcontext: EndContext,
}

// impl Record {
//     //! # Warning
//     //!
//     //! Calling these methods from a [`Record`] retrieved from other than [`.get_user_records()`] is deprecated.  
//     //! ***Except for two methods:** [`.record_url()`], [`.recorded_at()`]
//     //!
//     //! [`.record_url()`]: Self::record_url
//     //! [`.recorded_at()`]: Self::recorded_at
//     //!
//     //! [`.get_user_records()`]: crate::client::Client::get_user_records
//     //!
//     //! These are because the docs for the [TETRA CHANNEL API](https://tetr.io/about/api/) are incomplete,
//     //! so we cannot guarantee which values are passed.

//     /// Returns the PPS(Pieces Per Second) of this replay.
//     ///
//     /// Read the [warning](#warning) before using this method.
//     ///
//     /// # Panics
//     ///
//     /// Panics if necessary things is missing.
//     /// I can't predict when what will be missing.
//     pub fn pps(&self) -> f64 {
//         let ec = &self.endcontext;
//         ec.pieces_placed.unwrap() as f64 / (ec.final_time.unwrap() / 1000.)
//     }

//     /// Returns the KPP(Keys Per Piece) of this replay.
//     ///
//     /// Read the [warning](#warning) before using this method.
//     ///
//     /// # Panics
//     ///
//     /// Panics if necessary things is missing.
//     /// I can't predict when what will be missing.
//     pub fn kpp(&self) -> f64 {
//         let ec = &self.endcontext;
//         ec.inputs.unwrap() as f64 / ec.pieces_placed.unwrap() as f64
//     }

//     /// Returns the KPS(Keys Per Second) of this replay.
//     ///
//     /// Read the [warning](#warning) before using this method.
//     ///
//     /// # Panics
//     ///
//     /// Panics if necessary things is missing.
//     /// I can't predict when what will be missing.
//     pub fn kps(&self) -> f64 {
//         let ec = &self.endcontext;
//         ec.inputs.unwrap() as f64 / (ec.final_time.unwrap() / 1000.)
//     }

//     /// Returns the LPM(Lines Per Minute) of this replay.
//     ///
//     /// Read the [warning](#warning) before using this method.
//     ///
//     /// # Panics
//     ///
//     /// Panics if necessary things is missing.
//     /// I can't predict when what will be missing.
//     pub fn lpm(&self) -> f64 {
//         let ec = &self.endcontext;
//         ec.cleared_lines.unwrap() as f64 / (ec.final_time.unwrap() / 60000.)
//     }

//     /// Returns the SPP(Score Per Piece) of this replay.
//     ///
//     /// Read the [warning](#warning) before using this method.
//     ///
//     /// # Panics
//     ///
//     /// Panics if necessary things is missing.
//     /// I can't predict when what will be missing.
//     pub fn spp(&self) -> f64 {
//         let ec = &self.endcontext;
//         ec.score.unwrap() as f64 / ec.pieces_placed.unwrap() as f64
//     }

//     /// Returns the finesse rate of this replay.
//     ///
//     /// Read the [warning](#warning) before using this method.
//     ///
//     /// # Panics
//     ///
//     /// Panics if necessary things is missing.
//     /// I can't predict when what will be missing.
//     pub fn finesse_rate(&self) -> f64 {
//         let ec = &self.endcontext;
//         ec.clone().finesse.unwrap().perfect_pieces.unwrap() as f64
//             / ec.pieces_placed.unwrap() as f64
//             * 100.
//     }

//     /// Returns the record URL.
//     pub fn record_url(&self) -> String {
//         format!("https://tetr.io/#r:{}", self.replay_id)
//     }

//     /// Returns a UNIX timestamp when this record was recorded.
//     pub fn recorded_at(&self) -> i64 {
//         to_unix_ts(&self.recorded_at)
//     }
// }

impl AsRef<Record> for Record {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// If [`is_multi`] is true, this is the multiplayer end contexts.
/// Otherwise, this is the singleplayer end context.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum EndContext {
    SinglePlay(single_play_end_ctx::SinglePlayEndCtx),
    MultiPlay(Vec<multi_play_end_ctx::MultiPlayEndCtx>),
}

pub mod single_play_end_ctx {
    use super::*;

    /// The state this singleplayer replay finished with.
    ///
    /// ***No information about the endcontext field is given in the TETRA CHANNEL API docs,
    /// so the explanation of each content is a guess.**
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct SinglePlayEndCtx {
        /// A seed for RNG.
        pub seed: Option<f64>,
        /// The number of cleared lines.
        #[serde(rename = "lines")]
        pub cleared_lines: Option<u32>,
        ///
        pub level_lines: Option<u32>,
        ///
        pub level_lines_needed: Option<u32>,
        /// The number of keys presses.
        pub inputs: Option<u32>,
        /// The number of holds.
        pub holds: Option<u32>,
        ///
        pub time: Option<EndCtxTime>,
        /// The record's score.
        pub score: Option<u32>,
        ///
        #[serde(rename = "zenlevel")]
        pub zen_level: Option<u32>,
        ///
        #[serde(rename = "zenprogress")]
        pub zen_progress: Option<u32>,
        /// The level of the record.
        pub level: Option<u32>,
        ///
        pub combo: Option<u32>,
        ///
        #[serde(rename = "currentcombopower")]
        pub current_combo_power: Option<u32>,
        /// The number of maximum combo (zero indexed).
        #[serde(rename = "topcombo")]
        pub top_combo: Option<u32>,
        ///
        pub btb: Option<u32>,
        /// The number of maximum Back To Back chain (zero indexed).
        #[serde(rename = "topbtb")]
        pub top_btb: Option<u32>,
        /// The number of T-Spins.
        #[serde(rename = "tspins")]
        pub t_spins: Option<u32>,
        /// The number of pieces places.
        #[serde(rename = "piecesplaced")]
        pub pieces_placed: Option<u32>,
        /// How the lines was cleared.
        pub clears: Option<EndCtxClears>,
        /// Garbage-related data.
        pub garbage: Option<EndCtxGarbage>,
        /// The number of kills.
        pub kills: Option<u32>,
        /// The finesse data.
        pub finesse: Option<EndCtxFinesse>,
        /// The time at the finished.
        #[serde(rename = "finalTime")]
        pub final_time: Option<f64>,
        /// The game type.
        #[serde(rename = "gametype")]
        pub game_type: Option<String>,
    }
    
    impl AsRef<SinglePlayEndCtx> for SinglePlayEndCtx {
        fn as_ref(&self) -> &Self {
            self
        }
    }
    
    ///
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct EndCtxTime {
        ///
        pub start: Option<u32>,
        ///
        pub zero: Option<bool>,
        ///
        pub locked: Option<bool>,
        ///
        pub prev: Option<u32>,
        ///
        #[serde(rename = "frameoffset")]
        pub frame_offset: Option<i32>,
    }
    
    impl AsRef<EndCtxTime> for EndCtxTime {
        fn as_ref(&self) -> &Self {
            self
        }
    }
    
    /// How the lines was cleared.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct EndCtxClears {
        /// The number of cleared with Singles.
        pub singles: Option<u32>,
        /// The number of cleared with Doubles
        pub doubles: Option<u32>,
        /// The number of cleared with Triples
        pub triples: Option<u32>,
        /// The number of cleared with Quads
        pub quads: Option<u32>,
        /// The number of cleared with Realt T-Spins
        #[serde(rename = "realtspins")]
        pub realt_spins: Option<u32>,
        /// The number of cleared with Mini T-Spins
        #[serde(rename = "minitspins")]
        pub mini_t_spins: Option<u32>,
        /// The number of cleared with Mini T-Spin Singles
        #[serde(rename = "minitspinsingles")]
        pub mini_tss: Option<u32>,
        /// The number of cleared with Mini T-Spin Doubles
        #[serde(rename = "minitspindoubles")]
        pub mini_tsd: Option<u32>,
        /// The number of cleared with T-Spin Singles
        #[serde(rename = "tspinsingles")]
        pub tss: Option<u32>,
        /// The number of cleared with T-Spin Doubles
        #[serde(rename = "tspindoubles")]
        pub tsd: Option<u32>,
        /// The number of cleared with T-Spin Triples
        #[serde(rename = "tspintriples")]
        pub tst: Option<u32>,
        /// The number of cleared with T-Spin Quads
        #[serde(rename = "tspinquads")]
        pub t_spin_quads: Option<u32>,
        /// The number of cleared with All Clears
        pub all_clears: Option<u32>,
    }
    
    impl AsRef<EndCtxClears> for EndCtxClears {
        fn as_ref(&self) -> &Self {
            self
        }
    }
    
    /// Garbage-related data.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct EndCtxGarbage {
        /// The number of garbage sent.
        pub sent: Option<u32>,
        /// The number of garbage received.
        pub received: Option<u32>,
        /// The number of garbage attacks.
        pub attack: Option<u32>,
        /// The number of garbage cleared.
        pub cleared: Option<u32>,
    }
    
    impl AsRef<EndCtxGarbage> for EndCtxGarbage {
        fn as_ref(&self) -> &Self {
            self
        }
    }
    
    /// About the finesse data.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct EndCtxFinesse {
        /// The number of maximum finesse chain (?)
        pub combo: Option<u32>,
        /// The num of finesse faults.
        pub faults: Option<u32>,
        /// The number of perfect finesses.
        #[serde(rename = "perfectpieces")]
        pub perfect_pieces: Option<u32>,
    }
    
    impl AsRef<EndCtxFinesse> for EndCtxFinesse {
        fn as_ref(&self) -> &Self {
            self
        }
    }

}

pub mod multi_play_end_ctx {
    use super::*;

    /// The state this multiplayer replay finished with.
    ///
    /// ***No information about the endcontext field is given in the TETRA CHANNEL API docs,
    /// so the explanation of each content is a guess.**
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct MultiPlayEndCtx {
        /// Who is finished with this state.
        pub user: Option<User>,
        /// This user's handling settings.
        pub handling: Option<Handling>,
        /// 
        #[serde(rename = "active")]
        pub is_active: Option<bool>,
        /// Whether this user is winner.
        #[serde(rename = "success")]
        pub is_success: Option<bool>,
        /// The number of keys presses.
        pub inputs: Option<u32>,
        /// The number of pieces placed.
        #[serde(rename = "piecesplaced")]
        pub pieces_placed: Option<u32>,
        /// This user's natural order in this record.
        #[serde(rename = "naturalorder")]
        pub natural_order: Option<u32>,
        /// 
        pub score: Option<u32>,
        /// The number of wins.
        pub wins: Option<u32>,
        /// The points data.
        pub points: Option<Points>,
    }
    
    /// This user's handling settings.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct Handling {
        /// ARR(Automatic Repeat Rate).
        pub arr: Option<f64>,
        /// DAS(Delayed Auto Shift).
        pub das: Option<f64>,
        /// DCD(DAS Cut Delay).
        pub dcd: Option<f64>,
        /// SDF(Soft Drop Factor).
        pub sdf: Option<u32>,
        /// Whether "accidental hard drops prevention" is enabled.
        #[serde(rename = "safelock")]
        pub enable_safe_lock: Option<bool>,
        /// Whether "DAS cancellation when changing directions" is enabled.
        #[serde(rename = "cancel")]
        pub enable_das_cancel: Option<bool>,
    }
    
    /// The points data.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct Points {
        /// The number of wins.
        pub primary: Option<u32>,
        /// APM(Attacks Per Minute).
        pub secondary: Option<f64>,
        /// PPS(Pieces Per Second).
        pub tertiary: Option<f64>,
        /// Extra data.
        pub extra: Extra,
        /// APM for each game.
        #[serde(rename = "secondaryAvgTracking")]
        pub secondary_avg_tracking: Option<Vec<f64>>,
        /// PPS for each game.
        #[serde(rename = "tertiaryAvgTracking")]
        pub tertiary_avg_tracking: Option<Vec<f64>>,
        /// Extra data for each game.
        #[serde(rename = "extraAvgTracking")]
        pub extra_avg_tracking: Option<ExtraAvgTracking>,
    }
    
    /// Extra data.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct Extra {
        /// VS score.
        pub vs: Option<f64>,
    }
    
    /// Extra data for each game.
    #[derive(Clone, Debug, Deserialize)]
    #[non_exhaustive]
    pub struct ExtraAvgTracking {
        /// VS score for each game.
        #[serde(rename = "aggregatestats___vsscore")]
        pub aggregate_stats_vs_score: Option<Vec<f64>>,
    }
    
}

/// The user who set this Record,
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct User {
    /// The user's internal ID.
    #[serde(rename = "_id")]
    pub id: UserId,
    /// The user's username.
    #[serde(rename = "username")]
    pub name: String,
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &Self {
        self
    }
}