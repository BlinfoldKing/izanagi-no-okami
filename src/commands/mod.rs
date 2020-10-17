use serenity::framework::standard::macros::group;

pub mod reminder;

use reminder::*;

#[group]
#[commands(ping, remind)]
pub struct General;
