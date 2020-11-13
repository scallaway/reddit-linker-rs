use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::{
    channel::{ChannelType, GuildChannel},
    gateway::Presence,
    guild::{GuildStatus, Member},
    id::{ChannelId, UserId},
    user::OnlineStatus,
};
use std::collections::HashMap;

pub fn get_num_guilds(guilds: &Vec<GuildStatus>) -> String {
    format!(
        "{} guild{}",
        guilds.len(),
        if guilds.len() > 1 { "" } else { "s" }
    )
}

pub fn get_guild_human_members(members: &Vec<Member>) -> usize {
    members.iter().filter(|member| !member.user.bot).count()
}

pub fn get_guild_online_members(
    members: &HashMap<UserId, Member>,
    presences: &HashMap<UserId, Presence>,
) -> usize {
    presences
        .iter()
        // Get all online user ids
        .filter_map(|(user_id, presence)| match presence.status {
            OnlineStatus::Online => Some(user_id),
            _ => None,
        })
        // Remove members that are bots
        .filter(|user_id| !members.get(user_id).expect("User not found").user.bot)
        .count()
}

pub fn get_text_channels(channels: &HashMap<ChannelId, GuildChannel>) -> usize {
    get_channels(channels, ChannelType::Text)
}

pub fn get_voice_channels(channels: &HashMap<ChannelId, GuildChannel>) -> usize {
    get_channels(channels, ChannelType::Voice)
}

pub fn get_channels(channels: &HashMap<ChannelId, GuildChannel>, kind: ChannelType) -> usize {
    channels
        .values()
        .filter(|channel| channel.kind == kind)
        .count()
}

static SUBREDDIT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^/?(r/.+)").expect("Failed to initialise regex pattern"));

pub fn content_to_path(content: &str) -> Option<&str> {
    SUBREDDIT_REGEX.captures(content).map(|captures| {
        captures
            .get(1)
            .expect("subreddit regex to contain a group")
            .as_str()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn content_to_path_basic() {
        assert_eq!(
            content_to_path("r/ContagiousLaughter").unwrap(),
            "r/ContagiousLaughter"
        )
    }

    #[test]
    fn content_to_path_leading_slash() {
        assert_eq!(
            content_to_path("/r/ContagiousLaughter").unwrap(),
            "r/ContagiousLaughter"
        )
    }

    #[test]
    fn content_to_path_post() {
        assert_eq!(
            content_to_path(
                "/r/ContagiousLaughter/comments/ayiack/this_2_year_old_mocking_his_dads_accent/"
            )
            .unwrap(),
            "r/ContagiousLaughter/comments/ayiack/this_2_year_old_mocking_his_dads_accent/"
        )
    }
}
