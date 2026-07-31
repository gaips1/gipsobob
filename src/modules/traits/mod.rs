use crate::types::*;

mod types;
mod main_menu;



pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![main_menu::traits()]
}