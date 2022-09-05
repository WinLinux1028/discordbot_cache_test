use std::env;

use serenity::model::gateway::Ready;
use serenity::model::{channel::Message, prelude::ChannelId};
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::MessageId};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut message = msg.content.split(' ');
        let command = match message.next() {
            Some(s) => s,
            None => return,
        };

        if command == "!getmsg" {
            let channel_id = match message.next() {
                Some(s) => s,
                None => return,
            };
            let channel_id = match channel_id.parse() {
                Ok(o) => ChannelId(o),
                Err(_) => return,
            };

            let getmsg_id = match message.next() {
                Some(s) => s,
                None => return,
            };
            let getmsg_id = match getmsg_id.parse() {
                Ok(o) => MessageId(o),
                Err(_) => return,
            };

            let getmsg = match channel_id.message(&ctx, getmsg_id).await {
                Ok(o) => o,
                Err(_) => return,
            };
            let _ = msg.channel_id.say(&ctx, getmsg.content).await;
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.cache.set_max_messages(512);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
