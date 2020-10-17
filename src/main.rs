extern crate chrono;
extern crate chrono_tz;
extern crate date_time_parser;

use chrono::prelude::*;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::{channel::Message, gateway::{Ready, Activity}, id::GuildId};
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub mod commands;
pub mod entity;

use commands::*;
use entity::schedule::*;

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
                        let now = Utc::now().with_timezone(&chrono::FixedOffset::east(7 * 3600));
                        
                        if now.timestamp() > date.timestamp() {
                            let _ = s
                                .message
                                .reply(ctx.clone(), format!("REMINDER: {}", s.query))
                                .await;
                        } else {
                            reminder.schedules.push(s)
                        }
                    }
                    
                    set_status_to_current_time(&ctx).await;
                    tokio::time::delay_for(Duration::from_millis(1000)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn set_status_to_current_time(ctx: &Context) {
    let current_time = Utc::now();
    let indo_time = FixedOffset::east(7 * 3600).from_utc_datetime(&Utc::now().naive_utc());

    ctx.set_activity(Activity::listening(&format!("{:02}:{:02}:{:02}", 
        indo_time.hour(), indo_time.minute(), indo_time.second()))).await;
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
