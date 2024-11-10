//! A model for user roles.

use serde::Deserialize;

/// A user role.
#[derive(Clone, Debug, Deserialize)]
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

impl ToString for Role {
    /// Converts to a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tetr_ch::model::role::Role;
    /// assert_eq!(Role::User.to_string(), "User");
    /// assert_eq!(Role::Anon.to_string(), "Anonymous");
    /// assert_eq!(Role::Bot.to_string(), "Bot");
    /// assert_eq!(Role::Sysop.to_string(), "SYSOP");
    /// assert_eq!(Role::Admin.to_string(), "Administrator");
    /// assert_eq!(Role::Mod.to_string(), "Moderator");
    /// assert_eq!(Role::Halfmod.to_string(), "Community moderator");
    /// assert_eq!(Role::Banned.to_string(), "Banned user");
    /// assert_eq!(Role::Hidden.to_string(), "Hidden user");
    /// ```
    fn to_string(&self) -> String {
        match self {
            Role::User => "User",
            Role::Anon => "Anonymous",
            Role::Bot => "Bot",
            Role::Sysop => "SYSOP",
            Role::Admin => "Administrator",
            Role::Mod => "Moderator",
            Role::Halfmod => "Community moderator",
            Role::Banned => "Banned user",
            Role::Hidden => "Hidden user",
        }
        .to_string()
    }
}
