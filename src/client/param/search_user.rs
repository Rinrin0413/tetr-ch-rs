//! Features for the [`Client::search_user`](crate::client::Client::search_user) method.

/// A social connection.
///
/// Other social links could be added in the future.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SocialConnection {
    /// A Discord user ID.
    DiscordId(String),
    /// A Discord username.
    DiscordUsername(String),
    /// A Twitch user ID.
    TwitchId(String),
    /// A Twitch username (as used in the URL).
    TwitchUsername(String),
    /// A Twitch display name (may include Unicode).
    TwitchDisplayName(String),
    /// An X user ID.
    TwitterId(String),
    /// An X handle (as used in the URL).
    TwitterUsername(String),
    /// An X display name (may include Unicode).
    TwitterDisplayName(String),
    /// A Reddit user ID.
    RedditId(String),
    /// A Reddit username.
    RedditUsername(String),
    /// A YouTube user ID (as used in the URL).
    YoutubeId(String),
    /// A YouTube display name.
    YoutubeUsername(String),
    /// A Steam ID.
    SteamId(String),
    /// A Steam display name.
    SteamUsername(String),
}

impl SocialConnection {
    /// Converts into a parameter string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use tetr_ch::client::param::search_user::SocialConnection;
    /// let connection = SocialConnection::DiscordId("724976600873041940".to_string());
    /// assert_eq!(connection.to_param(), "discord:id:724976600873041940");
    /// ```
    pub(crate) fn to_param(&self) -> String {
        match self {
            SocialConnection::DiscordId(id) => format!("discord:id:{}", id),
            SocialConnection::DiscordUsername(name) => format!("discord:username:{}", name),
            SocialConnection::TwitchId(id) => format!("twitch:id:{}", id),
            SocialConnection::TwitchUsername(name) => format!("twitch:username:{}", name),
            SocialConnection::TwitchDisplayName(name) => {
                format!("twitch:display_username:{}", name)
            }
            SocialConnection::TwitterId(id) => format!("twitter:id:{}", id),
            SocialConnection::TwitterUsername(name) => format!("twitter:username:{}", name),
            SocialConnection::TwitterDisplayName(name) => {
                format!("twitter:display_username:{}", name)
            }
            SocialConnection::RedditId(id) => format!("reddit:id:{}", id),
            SocialConnection::RedditUsername(name) => format!("reddit:username:{}", name),
            SocialConnection::YoutubeId(id) => format!("youtube:id:{}", id),
            SocialConnection::YoutubeUsername(name) => format!("youtube:username:{}", name),
            SocialConnection::SteamId(id) => format!("steam:id:{}", id),
            SocialConnection::SteamUsername(name) => format!("steam:username:{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn social_connection_to_param_converts_into_param_str() {
        let connection = SocialConnection::DiscordId("724976600873041940".to_string());
        assert_eq!(connection.to_param(), "discord:id:724976600873041940");
    }
}
