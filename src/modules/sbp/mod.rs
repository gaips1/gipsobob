pub mod register;
pub mod account;
mod transfer;
mod invite;
mod captcha;
pub mod casino;
mod rob;

use crate::types::*;

pub const USER_UNAUTHORIZED_ERROR: &str = "Пользователь не зарегистрирован в Системе Быстрых Платежей! Скажите ему, чтобы он сделал это, написав `/reg`.\n||Или же пригласите его, используя команду `/invite`||";

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        account::account(),
        register::reg(),
        transfer::transfer_slash_command(),
        transfer::transfer_context_menu_command(),
        invite::invite(),
        captcha::captcha(),
        casino::casino(),
        rob::rob()
    ]
}