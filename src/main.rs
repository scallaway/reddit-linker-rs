mod utils;
use dotenv;
use serenity::{
    async_trait,
    client::{bridge::gateway::GatewayIntents, Context, EventHandler},
    model::{channel::Message, gateway::Ready, guild::Guild},
    Client,
};
use utils::{
    content_to_path, get_guild_human_members, get_guild_online_members, get_num_guilds,
    get_text_channels, get_voice_channels,
};

const MEMBER_LIMIT: u64 = 900;

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
            .members(ctx.http.clone(), Some(MEMBER_LIMIT), None)
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

        let path = match content_to_path(&message.content) {
            Some(path) => path,
            None => return,
        };

        message
            .delete(ctx.http.clone())
            .await
            .expect("Couldn't delete message");

        let msg = message
            .channel_id
            .send_message(&ctx.http, |m| {
                m.content(format!("{} https://reddit.com/{}", message.author, path));
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
