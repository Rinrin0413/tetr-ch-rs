//! A model for the user roles.

use serde::Deserialize;
use std::fmt;

/// A user role.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
pub enum Role {
    /// A normal user.
    #[serde(rename = "user")]
    User,
    /// An anonymous user.
    #[serde(rename = "anon")]
    Anon,
    /// A bot.
    #[serde(rename = "bot")]
    Bot,
    /// A SYSOP.
    #[serde(rename = "sysop")]
    Sysop,
    /// An administrator.
    #[serde(rename = "admin")]
    Admin,
    /// A moderator.
    #[serde(rename = "mod")]
    Mod,
    /// A community moderator.
    #[serde(rename = "halfmod")]
    Halfmod,
    /// A banned user.
    #[serde(rename = "banned")]
    Banned,
    /// A hidden user.
    #[serde(rename = "hidden")]
    Hidden,
}

impl Role {
    /// Whether the user is a normal user.
    pub fn is_normal_user(&self) -> bool {
        matches!(self, Role::User)
    }

    /// Whether the user is an anonymous.
    pub fn is_anon(&self) -> bool {
        matches!(self, Role::Anon)
    }

    /// Whether the user is a bot.
    pub fn is_bot(&self) -> bool {
        matches!(self, Role::Bot)
    }

    /// Whether the user is a SYSOP.
    pub fn is_sysop(&self) -> bool {
        matches!(self, Role::Sysop)
    }

    /// Whether the user is an administrator.
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }

    /// Whether the user is a moderator.
    pub fn is_mod(&self) -> bool {
        matches!(self, Role::Mod)
    }

    /// Whether the user is a community moderator.
    pub fn is_halfmod(&self) -> bool {
        matches!(self, Role::Halfmod)
    }

    /// Whether the user is banned.
    pub fn is_banned(&self) -> bool {
        matches!(self, Role::Banned)
    }

    /// Whether the user is hidden.
    pub fn is_hidden(&self) -> bool {
        matches!(self, Role::Hidden)
    }
}

impl AsRef<Role> for Role {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Anon => write!(f, "Anonymous"),
            Role::Bot => write!(f, "Bot"),
            Role::Sysop => write!(f, "SYSOP"),
            Role::Admin => write!(f, "Administrator"),
            Role::Mod => write!(f, "Moderator"),
            Role::Halfmod => write!(f, "Community moderator"),
            Role::Banned => write!(f, "Banned user"),
            Role::Hidden => write!(f, "Hidden user"),
        }
    }
}
