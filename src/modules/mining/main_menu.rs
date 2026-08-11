use crate::types::*;

/// Ваша майнинг ферма
#[poise::command(
    slash_command,
    rename = "майнинг",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn mining(ctx: Context<'_>) -> Result<(), Error> {

    Ok(())
}
