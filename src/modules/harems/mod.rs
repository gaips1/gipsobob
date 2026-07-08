use crate::types::*;

mod create_harem;
mod harems;
mod invite_to_harem;
mod my_harem;

pub async fn handle_harems_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = interaction.data.custom_id.as_str();

    if custom_id.starts_with("harem:leave") {
        my_harem::handle_harem_leave_button(ctx, interaction, data).await?;
    }

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        my_harem::my_harem(),
        create_harem::create_harem(),
        harems::harems(),
        invite_to_harem::invite_to_harem(),
    ]
}
