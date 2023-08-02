use std::fs;
use std::ffi::*;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::*;
use serenity::model::id::ChannelId;
use serenity::model::gateway::Ready;
struct Handler;
// #[no_mangle]
extern "C" {
    static MOD_ROLE: usize;
    static OWNER_ROLE: usize;
    // static ROLES_CHANNEL: usize;

    fn trim_id(a: *const c_char) -> usize;
    fn is_id(a: *const c_char) -> bool;
}
// fn c_to_s(s: *const c_char) -> &'static str {
//     (unsafe { CStr::from_ptr(s) }).to_str().unwrap()
// }
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
                msg.reply(&ctx, format!("[!] This command can only be used by Mods")).await.unwrap();
            } else {
                if args.len() < 3 {
                    msg.reply(&ctx, format!("[!] Need at least 3 arguments!")).await.unwrap();
                    return;
                }
                let _ = args.remove(0);
                let channel = CString::new(args.remove(0)).unwrap();//.as_ptr();
                if !(unsafe { is_id(channel.as_ptr()) }) {
                    msg.reply(&ctx, format!("[!] Argument 0 is not a valid channel!")).await.unwrap();
                    return;
                }
                let channel_id = ChannelId(unsafe{trim_id(channel.as_ptr())} as u64);
                match channel_id.say(&ctx, args.join(" ")).await {
                    Ok(_) => {},
                    Err(_) => {
                    msg.reply(&ctx, format!("[!] Unable to send message. Channel may be invalid.")).await.unwrap();
                    }
                }

                
                // println!("{}", c_to_s());
                // msg.reply(&ctx, format!("{}", unsafe{trim_id(channel.as_ptr())})).await.unwrap();
                // println!("{}", );
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
