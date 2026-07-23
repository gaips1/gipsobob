use crate::types::*;

mod helpers;
mod types;

pub async fn handle_quests_select(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    values: &Vec<String>,
) -> Result<(), Error> {
    let status = match values.first().unwrap().as_str() {
        "active" => types::Status::Active,
        "completed" => types::Status::Completed,
        "expired" => types::Status::Expired,
        _ => {
            return Ok(());
        }
    };

    let quests = helpers::get_user_quests(&data.pool, press.user.id.get(), status).await?;

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .embed(helpers::create_quests_embed(&quests))
            .components(helpers::quests_select_menu(status))
    );

    Ok(())
}

/// Квесты
#[poise::command(
    slash_command,
    rename = "квесты",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn quests(ctx: Context<'_>) -> Result<(), Error> {
    let quests = helpers::get_user_quests(
        &ctx.data().pool,
        ctx.author().id.get(),
        types::Status::Active,
    )
    .await?;

    ctx.send(
        poise::CreateReply::default()
            .embed(helpers::create_quests_embed(&quests))
            .components(helpers::quests_select_menu(types::Status::Active))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![quests()]
}
