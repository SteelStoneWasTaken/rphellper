use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

mod split;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Variables
        let user = msg.author.id;
        // Ignore own menssage.
        if user == ctx.http.get_current_user().await.unwrap().id {
            return;
        }
        // Separe and execute command lines by ';' or \n
        for i in split::functions(msg.content.clone()) {
            // Split command lines into components.
            let a = split::components(i.clone());
            println!("{a:?}");
            
            reply(&ctx, &msg, i.as_str()).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

async fn reply(ctx: &Context, msg: &Message, output: &str) {
    if !output.is_empty(){
        if let Err(why) = msg.reply_ping(&ctx.http, output).await {
            println!("Error sending message: {why:?}");
        }
    }
}