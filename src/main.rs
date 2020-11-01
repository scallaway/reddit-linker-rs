mod utils;
use dotenv;
use regex::Regex;
use serenity::{
    async_trait,
    client::{bridge::gateway::GatewayIntents, Context, EventHandler},
    model::{channel::Message, gateway::Ready, guild::Guild},
    Client,
};
use utils::{
    get_guild_human_members, get_guild_online_members, get_num_guilds, get_text_channels,
    get_voice_channels,
};

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

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let members = &guild
            .members(ctx.http.clone(), Some(900), None)
            .await
            .expect("Could not get members.");
        println!("Connected to the {} server.", guild.name);
        println!(
            "{} contains {} human members, of which {} are online.",
            guild.name,
            get_guild_human_members(members),
            get_guild_online_members(&guild.members, &guild.presences)
        );
        println!(
            "{} contains {} text channels and {} voice channels",
            guild.name,
            get_text_channels(&guild.channels),
            get_voice_channels(&guild.channels)
        );
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return;
        }

        let re: Regex = Regex::new(r"^/?(r/).+").expect("Failed to initialise regex pattern");

        if !re.is_match(&message.content) {
            return;
        }

        message
            .delete(ctx.http.clone())
            .await
            .expect("Couldn't delete message");

        let split_message: Vec<&str> = message.content.split('/').collect();

        let msg = message
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(format!(
                    "{} https://reddit.com/r/{}",
                    message.author, split_message[1]
                ));
                m
            })
            .await;

        if let Err(_why) = msg {
            println!("Error sending message");
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the `.env` file
    let token = dotenv::var("DISCORD_TOKEN").expect("Could not get Discord Token.");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Could not create new client.");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
