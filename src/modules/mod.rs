use crate::types::*;

pub mod dromland;
pub mod fun;
pub mod giveaways;
pub mod harems;
pub mod marriages;
mod other;
pub mod quests;
pub mod sbp;
pub mod counter;

pub fn all() -> Vec<poise::Command<Data, Error>> {
    [
        fun::commands(),
        sbp::commands(),
        marriages::commands(),
        harems::commands(),
        dromland::commands(),
        other::commands(),
        giveaways::commands(),
        quests::commands(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
