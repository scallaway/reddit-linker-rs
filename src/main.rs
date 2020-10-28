use dotenv;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, guild::GuildStatus},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("{}", get_num_guilds(&ready.guilds));
        println!("Members: {}", get_guild_members(&ready.guilds));
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

fn get_guild_members(guilds: &Vec<GuildStatus>) -> i64 {
    if guilds.len() == 0 {
        -1
    } else {
        let guild = guilds.first().unwrap();

        match *guild {
            GuildStatus::OnlineGuild(ref guild) => guild.member_count as i64,
            GuildStatus::OnlinePartialGuild(_) => -1,
            GuildStatus::Offline(_) => -1,
            _ => -1,
        }
    }
}
