use crate::types::*;

mod game;

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        game::game()
    ]
}
