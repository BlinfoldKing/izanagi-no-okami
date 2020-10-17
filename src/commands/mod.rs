use serenity::framework::standard::macros::group;

pub mod reminder;
pub mod help;

use reminder::*;
use help::*;

#[group]
#[commands(ping, remind, help)]
pub struct General;
