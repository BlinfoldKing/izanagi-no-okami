use chrono::prelude::*;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct Schedule {
    pub message: Message,
    pub query: String,
    pub date_time: NaiveDateTime,
}

pub struct ReminderController {
    pub schedules: Vec<Schedule>,
}

pub struct Reminder;
impl TypeMapKey for Reminder {
    type Value = ReminderController;
}
