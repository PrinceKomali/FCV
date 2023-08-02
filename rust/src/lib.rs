use std::fs;
use std::ffi::*;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::*;
// use serenity::model::id::*;
use serenity::model::gateway::Ready;
struct Handler;
// #[no_mangle]
extern "C" {
    static MOD_ROLE: usize;
    static OWNER_ROLE: usize;
    // static ROLES_CHANNEL: usize;

    fn test(a: *const c_char) -> *const c_char;
    fn is_id(a: *const c_char) -> bool;
}
fn c_to_s(s: *const c_char) -> & 'static str {
    (unsafe { CStr::from_ptr(s) }).to_str().unwrap()
}
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut args: Vec<&str> = msg.content.split(" ").collect();
        if args[0] == "!send" {
            let mut is_mod = false;
            for role in msg.member.clone().unwrap().roles {
                if
                    (role.0 as usize) == (unsafe { MOD_ROLE }) ||
                    (role.0 as usize) == (unsafe { OWNER_ROLE })
                {
                    is_mod = true;
                }
            }
            if !is_mod {
                msg.reply(&ctx, format!("This command can only be used by Mods")).await.unwrap();
            } else {
                // ctx.cache;
                // println!("{}", c_to_s(unsafe{test(CString::new("").unwrap().as_ptr())}));
                // println!("{}", unsafe{is_id(CString::new(args[1]).unwrap().as_ptr())});
                // println!("{}", args.remove(0));
                msg.reply(&ctx, format!("{}",unsafe{is_id(CString::new(args[1]).unwrap().as_ptr())})).await.unwrap();
                // println!("{}", );
                // ChannelId(1136321118349828196).say(&ctx, "hi").await.unwrap();
            }
        }
    }
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("{:?}", reaction);
        let _ = reaction.delete(ctx).await;
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
#[no_mangle]
async fn start_bot() {
    let token = fs::read_to_string("token").expect("Token no read");
    let intents =
        GatewayIntents::non_privileged() |
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler).await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
