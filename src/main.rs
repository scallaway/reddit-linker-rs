use dotenv;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http,
    model::{
        gateway::Ready,
        guild::{Guild, GuildStatus, Member},
        id::UserId,
    },
    Client,
};
use std::collections::HashMap;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected to the Discord Network.", ready.user.name);
        println!(
            "{} is part of {} servers.",
            ready.user.name,
            get_num_guilds(&ready.guilds)
        );
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: bool) {
        println!("Connected to the {} server.", guild.name);
        println!("total members: {}", guild.member_count);
        println!(
            "Server contains {} human members.",
            get_guild_members(
                &guild
                    .members(&self.http)
                    .expect("Couldn't get members from guild")
            )
        );
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the `.env` file
    let token = dotenv::var("DISCORD_TOKEN").unwrap();
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Could not create new client.");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn get_num_guilds(guilds: &Vec<GuildStatus>) -> String {
    if guilds.len() == 1 {
        String::from("1 guild")
    } else {
        format!("{:?} guilds", guilds.len())
    }
}

fn get_guild_members(members: &HashMap<UserId, Member>) -> usize {
    println!("Size: {}", members.len());
    let mut human_members: Vec<Member> = Vec::new();
    for user in members.values() {
        println!("User: {:?}", user);
        if !user.user.bot {
            human_members.push(user.clone());
        }
    }

    human_members.len()
}
