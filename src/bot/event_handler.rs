use crate::bot::command_handler::CommandHandler;

use serenity::{
    async_trait,
    model::{gateway::Ready, channel::Message},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, r: Ready) {
        println!("Ready @ Session {}", r.session_id);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        CommandHandler::new().process_message(ctx, msg, "!").await;
    }
}