use poise::serenity_prelude as serenity;
use crate::{modules::fun::get_kys_list, types::*};
use rand::prelude::*;

pub async fn handle_kys_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let list = get_kys_list();

    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(
                vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("kys_btn")
                        .label("KYS")
                        .emoji('☠')
                        .style(serenity::ButtonStyle::Danger)
                ])]
            )
    )).await?;
    Ok(())
}