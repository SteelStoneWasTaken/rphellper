use serenity::all::ChannelId;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::model::gateway::Ready;

use std::sync::Arc;

static mut CTX: Option<Arc<Context>> = None;

struct Handler;

#[tauri::command]
async fn start(token: String) -> String {
    unsafe {
        if CTX.is_none() {
            println! ("Bot is now starting...");

            let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

            if let Ok(mut client) = Client::builder(&token, intents).event_handler(Handler).await {
                if let Err(_) = client.start().await {
                    // SET_STATUS: Error: Failed to start client
                    println!("Error: Failed to start client.");
                    return "Error: Failed to start client.".to_string();
                }
                return "".to_string();
            } else {
                // SET_STATUS: Error: Failed to create client
                println!("Error: Failed to create client.");
                return "Error: Failed to create client.".to_string();
            }
        } else {
            if let Some(ref ctx) = CTX {
                ctx.shard.shutdown_clean();
                CTX = None;
                // SET_STATUS: Offline
                println!("Bot has been stopped.");
                return "".to_string();

            } else {
                println!("Error: Context is not set.");
                return "Error: Context is not set.".to_string();
            }
        }
    }
}

// Bot functionalities
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        unsafe {
            CTX = Some(Arc::new(ctx));

            // SET_STATUS: Online
            println!("Bot is now online as `{}`", ready.user.name);
        }
    }
}

// Say function
#[tauri::command]
async fn say() {
    unsafe {
        if let Some(ref ctx) = CTX {
            let channel_id = ChannelId::new(1267120740377825386);
            if let Err(why) = channel_id.say(&ctx.http, "FALEEEEI!").await {
                println!("Error sending message: {why:?}");
            }
        } else {
            println!("Context is not set.");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start,
            say
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
