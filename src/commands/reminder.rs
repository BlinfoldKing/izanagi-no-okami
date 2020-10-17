use chrono::prelude::*;
use date_time_parser::{DateParser, TimeParser};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::entity::schedule::*;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!").await?;

    Ok(())
}

#[command]
pub async fn remind(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.raw().collect::<Vec<&str>>().join(" ");
    let date = DateParser::parse(&query).unwrap_or(Local::now().naive_local().date());
    let time = TimeParser::parse(&query).unwrap_or(Local::now().time());
    let date_time = date.and_time(time);

    let mut data = ctx.data.write().await;
    let reminder = data.get_mut::<Reminder>().unwrap();
    msg.reply(ctx, format!("reminder set to: {:?}", date_time))
        .await?;

    reminder.schedules.push(Schedule {
        message: msg.clone(),
        query: query,
        date_time: date_time,
    });

    Ok(())
}
