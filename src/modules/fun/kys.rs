use crate::{helpers::resolve_data_path, types::*};
use poise::CreateReply;
use rand::prelude::*;
use std::sync::OnceLock;

static KYS_LIST: OnceLock<Vec<String>> = OnceLock::new();
fn get_kys_list() -> &'static [String] {
    KYS_LIST.get_or_init(|| {
        let data = std::fs::read_to_string(resolve_data_path("src/modules/fun/kys.json")).unwrap();
        serde_json::from_str(&data).expect("Failed to parse kys.json")
    })
}
/// KEEP YOURSELF SAFE
#[poise::command(
    slash_command,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn kys(ctx: Context<'_>) -> Result<(), Error> {
    let list = get_kys_list();
    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    ctx.send(
        CreateReply::default()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("kys_btn")
                    .label("KYS")
                    .emoji('☠')
                    .style(serenity::ButtonStyle::Danger),
            ])])
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

pub async fn handle_kys_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let list = get_kys_list();

    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    crate::create_edit_response!(
        ctx,
        interaction,
        serenity::CreateInteractionResponseMessage::default()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("kys_btn")
                    .label("KYS")
                    .emoji('☠')
                    .style(serenity::ButtonStyle::Danger),
            ])])
    );
    Ok(())
}
