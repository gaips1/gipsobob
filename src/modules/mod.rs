use crate::types::*;

pub mod fun;

pub fn all() -> Vec<poise::Command<Data, Error>> {
    [
        fun::commands()
    ]
    .into_iter()
    .flatten()
    .collect()
}