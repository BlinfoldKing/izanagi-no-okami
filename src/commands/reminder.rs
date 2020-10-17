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
    let wib = chrono::FixedOffset::east(7 * 3600);

    let default_datetime = Utc::now().with_timezone(&wib);
    let date =     if let Some(d) = DateParser::parse(&query) {
        wib.ymd(d.year(), d.month(), d.day())
    } else {
        default_datetime.date()
    };

    let time = TimeParser::parse(&query).unwrap_or(default_datetime.time());

    let date_time = date.and_time(time).unwrap_or(default_datetime);

    let mut data = ctx.data.write().await;
    let reminder = data.get_mut::<Reminder>().unwrap();
    msg.reply(ctx, format!("reminder set to: {}-{}-{} {:02}:{:02}",
      date_time.date().year(),
      date_time.date().month(),
      date_time.date().day(),
      date_time.time().hour(), 
      date_time.time().minute()))
        .await?;

    reminder.schedules.push(Schedule {
        message: msg.clone(),
        query: query,
        date_time: date_time,
    });

    Ok(())
}
