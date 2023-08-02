use std::fs;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::*;
use serenity::model::gateway::Ready;
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hi" {
            msg.reply(&ctx, "hello").await.unwrap();
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
