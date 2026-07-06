use crate::types::*;

pub mod dromland;
pub mod fun;
pub mod harems;
pub mod marriages;
pub mod sbp;

pub fn all() -> Vec<poise::Command<Data, Error>> {
    [
        fun::commands(),
        sbp::commands(),
        marriages::commands(),
        harems::commands(),
        dromland::commands(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
