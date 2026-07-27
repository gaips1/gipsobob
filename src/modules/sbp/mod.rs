pub mod account;
mod captcha;
pub mod casino;
mod invite;
pub mod register;
mod rob;
mod transfer;
mod top;

use crate::types::*;

pub const USER_UNAUTHORIZED_ERROR: &str = "Пользователь не зарегистрирован в Системе Быстрых Платежей! Скажите ему, чтобы он сделал это, написав `/сбп регистрация`.\n||Или же пригласите его, используя команду `/invite`||";

pub async fn handle_sbp_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    match interaction.data.custom_id.as_str() {
        "sbp:notifications_change" => {
            account::handle_change_notifications_button(ctx, interaction, data).await?;
        }

        "sbp:register" => {
            register::sbp_register(ctx, interaction, data).await?;
        }

        _ => {}
    }
    Ok(())
}

/// Система Быстрых Платежей
#[poise::command(
    slash_command,
    rename = "сбп",
    subcommands(
        "account::account",
        "register::reg",
        "transfer::transfer_slash_command",
        "invite::invite",
        "top::top"
    ),
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn sbp(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        sbp(),
        transfer::transfer_context_menu_command(),
        captcha::captcha(),
        casino::casino(),
        rob::rob(),
    ]
}
