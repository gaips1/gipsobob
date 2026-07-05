use crate::{types::*};
use poise::serenity_prelude as serenity;

mod marry;
mod marriages;
pub mod my_marriage;

pub async fn handle_marriages_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data
) -> Result<(), Error> {
    let custom_id = interaction.data.custom_id.as_str();

    if custom_id.starts_with("marriage:divorce") {
        my_marriage::handle_divorce_button(ctx, interaction, data).await?;
    }
    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        marry::marry(),
        my_marriage::my_marriage(),
        marriages::marriages()
    ]
}