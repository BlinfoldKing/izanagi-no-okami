use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready, id::GuildId};
use serenity::prelude::*;
use std::env;

extern crate chrono;
use chrono::prelude::*;

extern crate date_time_parser;
use date_time_parser::{DateParser, TimeParser};

use std::boxed::Box;
use std::time::Duration;

use std::sync::atomic::{AtomicBool, Ordering};

struct Schedule {
    message: Message,
    query: String,
    date_time: NaiveDateTime,
}

struct ReminderController {
    schedules: Vec<Schedule>,
}

struct Reminder;
impl TypeMapKey for Reminder {
    type Value = ReminderController;
}

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, _msg: Message) {
        // if msg.content == "\\ping" {
        //     if let Err(err) = msg.channel_id.say(&ctx.http, "Pong!").await {
        //         println!("Error sending message: {:?}", err)
        //     }
        // }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");
        // let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {
            // let mut ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    // set_status_to_current_time(Arc::clone(&ctx2)).await;

                    let mut data = ctx.data.write().await;
                    let reminder = data.get_mut::<Reminder>().unwrap();

                    let schedule = reminder.schedules.pop();
                    if let Some(s) = schedule {
                        let date = s.date_time;
                        let now = Local::now().naive_local();

                        if now.timestamp() > date.timestamp() {
                            let _ = s
                                .message
                                .reply(ctx.clone(), format!("REMINDER: {}", s.query))
                                .await;
                        } else {
                            reminder.schedules.push(s)
                        }

                        tokio::time::delay_for(Duration::from_millis(10)).await;
                    }
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

#[group]
#[commands(ping, remind)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn remind(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
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

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected token in environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("\\"))
        .group(&GENERAL_GROUP);

    let mut client = Client::new(token)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        data.insert::<Reminder>(ReminderController {
            schedules: Vec::new(),
        });
    }

    if let Err(err) = client.start().await {
        println!("{:?}", err)
    }
}
