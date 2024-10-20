use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::all::ChannelId;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

static mut CTX: Option<Context> = None;
static mut STATUS: String = String::new();
static mut HINT: String = String::new(); 

#[tauri::command]
async fn say() {
    let mut _ctx = None;
    unsafe {_ctx = CTX.clone()}
    let ctx = _ctx.unwrap();


    let channel_id = ChannelId::new(1267120740377825386);
    if let Err(why) = channel_id.say(&ctx.http, "FALEEEEI!").await {
        println!("Error sending message: {why:?}");
    }
}

#[tauri::command]
async fn start(token: String){
    // Starting... // // // // // // // // // // //
    println! ("Bot is now starting...");         //
    unsafe {STATUS = "Starting...".to_string()}  //
    unsafe {HINT = "".to_string()}               //
    // // // // // // // // // // // // // // // //

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    if let Ok(mut client) = Client::builder(&token, intents)
    .event_handler(Handler)
    .await {
        if let Err(_) = client.start().await {
            // Error: Failed to start client / // // // // // // // // // //
            println!("Error: Failed to start client.");                   //
            unsafe {STATUS = "Offline.".to_string()}                      //
            unsafe {HINT = "Error: Failed to start client.".to_string()}  //
        // // // // // // // // // // // // // // // // // // // // // // //

        }

    } else {
        // Error: Failed to create client / // // // // // // // // // //
        println!("Error: Failed to create client.");                   //
        unsafe {STATUS = "Offline.".to_string()}                       //
        unsafe {HINT = "Error: Failed to create client.".to_string()}  //
        // // // // // // // // // // // // // // // // // // // // // //

    }
}

// Bot functionalities
#[async_trait]
impl EventHandler for Handler {
    async fn message (&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready (&self, ctx: Context, ready: Ready) {
        // Online.  // // // // // // // // // // // // // // // //
        println!("Bot is now online as `{}`", ready.user.name);  //
        unsafe {STATUS = format!("Online.")}                     //
        unsafe {HINT = format!("({})", ready.user.name)}         //
        // // // // // // // // // // // // // // // // // // // //

        unsafe {CTX = Some(ctx)}
    }
}

#[tauri::command]
fn check() -> String {
    unsafe {
        if STATUS == "" { STATUS = "Offline.".to_string() }
        if STATUS == "Offline." { CTX = None }
        return STATUS.clone();
    }
}
#[tauri::command]
fn check_hint() -> String {
    unsafe {
        return HINT.clone();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start,
            check,
            check_hint,
            say
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
