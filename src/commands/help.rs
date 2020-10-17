use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
pub async fn help(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Help");
        m.embed(|e| {
            e.title("Help");
            e.description("Command list of Izanagi");
            e.field("remind <query>", "remind Teknofest Meet June 14th 10:00", false);
            e
        });
        m
    }).await?;
    Ok(())
}