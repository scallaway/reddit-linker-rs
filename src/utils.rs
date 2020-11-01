use serenity::model::{
    channel::{ChannelType, GuildChannel},
    gateway::Presence,
    guild::{GuildStatus, Member},
    id::{ChannelId, UserId},
    user::OnlineStatus,
};
use std::collections::HashMap;

pub fn get_num_guilds(guilds: &Vec<GuildStatus>) -> String {
    if guilds.len() == 1 {
        String::from("1 guild")
    } else {
        format!("{:?} guilds", guilds.len())
    }
}

pub fn get_guild_human_members(members: &Vec<Member>) -> usize {
    let human_members: Vec<Member> = members
        .iter()
        .filter(|member| !member.user.bot)
        .map(|x| x.clone())
        .collect();

    human_members.len()
}

pub fn get_guild_online_members(
    members: &HashMap<UserId, Member>,
    presences: &HashMap<UserId, Presence>,
) -> usize {
    let mut online_members: usize = 0;
    for (user_id, presence) in presences.iter() {
        if !members.get(user_id).expect("User not found").user.bot
            && presence.status == OnlineStatus::Online
        {
            online_members += 1;
        }
    }

    online_members
}

pub fn get_text_channels(channels: &HashMap<ChannelId, GuildChannel>) -> usize {
    let mut text_channels: Vec<GuildChannel> = Vec::new();
    for channel in channels.values() {
        if channel.kind == ChannelType::Text {
            text_channels.push(channel.clone());
        }
    }

    text_channels.len()
}

pub fn get_voice_channels(channels: &HashMap<ChannelId, GuildChannel>) -> usize {
    let mut voice_channels: Vec<GuildChannel> = Vec::new();
    for channel in channels.values() {
        if channel.kind == ChannelType::Voice {
            voice_channels.push(channel.clone());
        }
    }

    voice_channels.len()
}
