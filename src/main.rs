use dotenv;
use serenity::{
    async_trait,
    client::{bridge::gateway::GatewayIntents, Context, EventHandler},
    model::{
        gateway::Ready,
        guild::{Guild, GuildStatus, Member},
    },
    Client,
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
        println!("Connected to the {} server.", guild.name);
        println!(
            "Server contains {} human members.",
            get_guild_members(
                &guild
                    .members(ctx.http, Some(900), None)
                    .await
                    .expect("Could not get members")
            )
        );
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

fn get_num_guilds(guilds: &Vec<GuildStatus>) -> String {
    if guilds.len() == 1 {
        String::from("1 guild")
    } else {
        format!("{:?} guilds", guilds.len())
    }
}

fn get_guild_members(members: &Vec<Member>) -> usize {
    let mut human_members: Vec<Member> = Vec::new();

    for member in members {
        if !member.user.bot {
            human_members.push(member.clone());
        }
    }

    human_members.len()
}
