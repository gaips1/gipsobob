use crate::types::*;

pub mod counter;
pub mod dialogues;
pub mod dromland;
pub mod fun;
pub mod giveaways;
pub mod harems;
pub mod marriages;
mod other;
pub mod quests;
pub mod sbp;
pub mod traits;

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
        traits::commands()
    ]
    .into_iter()
    .flatten()
    .collect()
}
