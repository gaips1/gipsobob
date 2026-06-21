use crate::types::*;

pub mod fun;
pub mod sbp;

pub fn all() -> Vec<poise::Command<Data, Error>> {
    [
        fun::commands(),
        sbp::commands()
    ]
    .into_iter()
    .flatten()
    .collect()
}