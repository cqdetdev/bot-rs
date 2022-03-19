use serenity::{
    prelude::*,
    model::{
        channel::Message,
    },
};

pub async fn ping(ctx: &Context, msg: &Message) {
    let mut m = msg.channel_id.say(&ctx.http, "Pong!").await.unwrap();
    println!("{} {}", &m.timestamp.timestamp(), msg.timestamp.timestamp());
    let ping = &m.timestamp.timestamp() - msg.timestamp.timestamp();
    m.edit(&ctx.http, |x| {
        x.content(format!("Ping: `{}ms`", ping))
    }).await.unwrap();
}