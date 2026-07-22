use crate::types::*;

mod marriages;
mod marry;
mod my_marriage;

pub async fn handle_marriages_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = interaction.data.custom_id.as_str();

    if custom_id.starts_with("marriage:divorce") {
        my_marriage::handle_divorce_button(ctx, interaction, data).await?;
    }
    Ok(())
}

/// Система браков
#[poise::command(
    slash_command,
    rename = "брак",
    subcommands("marry::marry", "my_marriage::my_marriage", "marriages::marriages"),
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn marriages(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![marriages()]
}
