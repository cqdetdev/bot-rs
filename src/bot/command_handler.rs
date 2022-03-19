use std::future::Future;
use std::collections::HashMap;
use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
};
use crate::bot::commands::ping::ping;

pub type Command = fn(ctx: &Context, msg: &Message) -> dyn Future<Output=()>;

pub struct CommandHandler {
    commands: HashMap<&'static str, Command>
}

impl CommandHandler {
    pub fn new() -> Self {
        return CommandHandler {
            commands: HashMap::new()
        }
    }

    pub async fn process_message(&self, ctx: Context, msg: Message, prefix: &'static str) {
        if msg.content.starts_with(prefix) {
            let args = msg.content.split(" ").collect::<Vec<&str>>();
            let cmd = &args[0][1..];
            // let command: Command = *self.commands.get(cmd).unwrap();
            // // return command(&ctx, &msg).await;
            // the size for values of type `dyn std::future::Future<Output = ()>` cannot be known at compilation time
            match cmd {
                "ping" => {
                    ping(&ctx, &msg).await;
                },
                _ => {}
            }
        }
    }
}