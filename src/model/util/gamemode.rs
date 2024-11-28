//! A model for the game modes.

use crate::client::param::record::{self, Gamemode as RecordGm};
use serde::Deserialize;
use std::fmt;

/// A game mode.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct Gamemode(String);

impl Gamemode {
    /// Converts into a [`crate::client::param::record::Gamemode`].
    /// If failed, returns the game mode as is as `Err<String>`.
    pub fn into_record_gamemode(&self) -> Result<record::Gamemode, String> {
        match self.0.as_str() {
            "40l" => Ok(RecordGm::FortyLines),
            "blitz" => Ok(RecordGm::Blitz),
            "zenith" => Ok(RecordGm::Zenith),
            "zenithex" => Ok(RecordGm::ZenithEx),
            "league" => Ok(RecordGm::League),
            _ => Err(self.0.clone()),
        }
    }
}

impl AsRef<Gamemode> for Gamemode {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for Gamemode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
