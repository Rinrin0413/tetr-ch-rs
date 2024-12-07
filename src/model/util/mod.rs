//! Utility models for the models.

pub mod achievement;
pub mod badge_id;
pub mod gamemode;
pub mod league_rank;
pub mod news_stream;
pub mod record_leaderboard;
pub mod replay_id;
pub mod role;
pub mod timestamp;
pub mod user_id;

pub use self::{
    achievement::Achievement, badge_id::BadgeId, gamemode::Gamemode, league_rank::Rank,
    news_stream::NewsStream, record_leaderboard::RecordLeaderboard, replay_id::ReplayId,
    role::Role, timestamp::Timestamp, user_id::UserId,
};
