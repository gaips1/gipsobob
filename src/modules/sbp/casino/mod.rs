use crate::checks::sbp_check;
use crate::types::*;
use poise::CreateReply;

mod guess;
pub mod mines;
mod slots;

/// Казино "У Снюсоеда"
#[poise::command(
    slash_command,
    rename = "казино",
    check = "sbp_check",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn casino(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::default()
        .title("Добро пожаловать в казино!")
        .description("**Выбирайте игру:**");

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("casino:slots").label("Слоты"),
        serenity::CreateButton::new("casino:guess").label("Угадай число"),
        serenity::CreateButton::new("casino:mines").label("Сапёр"),
    ])];

    ctx.send(
        CreateReply::default()
            .embed(embed)
            .ephemeral(true)
            .components(buttons),
    )
    .await?;
    Ok(())
}

pub async fn handle_casino_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    match interaction.data.custom_id.as_str() {
        "casino:slots" => {
            slots::handle_slots_button(ctx, interaction, data).await?;
        }

        "casino:guess" => {
            guess::handle_guess_button(ctx, interaction, data).await?;
        }

        _ => {
            if interaction.data.custom_id.starts_with("casino:mines") {
                mines::handle_mines_buttons(ctx, interaction, data).await?;
            }
        }
    }
    Ok(())
}
