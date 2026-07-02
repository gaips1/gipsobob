use crate::{types::*};

mod marry;
pub mod my_marriage;

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        marry::marry(),
        my_marriage::my_marriage()
    ]
}