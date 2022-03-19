mod bot;

use std::env::var;
use serenity::prelude::*;
use bot::event_handler::Handler;

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").expect("Add token to .env");
    let mut client = Client::builder(token)
    .event_handler(Handler)
    .await
    .expect("Error");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
