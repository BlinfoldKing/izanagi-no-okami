use serenity::framework::standard::macros::group;

pub mod help;
pub mod reminder;

use help::*;
use reminder::*;

#[group]
#[commands(ping, remind, help)]
pub struct General;
