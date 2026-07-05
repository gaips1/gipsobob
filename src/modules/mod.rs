use crate::types::*;

pub mod fun;
pub mod sbp;
pub mod marriages;
pub mod harems;

pub fn all() -> Vec<poise::Command<Data, Error>> {
    [
        fun::commands(),
        sbp::commands(),
        marriages::commands(),
        harems::commands()
    ]
    .into_iter()
    .flatten()
    .collect()
}