use crate::types::*;
use rand::seq::IndexedRandom as _;

pub mod helpers;
mod notifications;
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

pub async fn handle_quests_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = interaction.data.custom_id.as_str();

    if custom_id.starts_with("quests:notifications:") {
        notifications::handle_notifications_button(
            ctx,
            interaction,
            data,
            custom_id.strip_prefix("quests:notifications:").unwrap(),
        )
        .await?;
    }

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

#[poise::command(prefix_command)]
async fn add_quest(
    ctx: Context<'_>,
    user_id: serenity::UserId,
    quest_id: Option<String>,
) -> Result<(), Error> {
    if ctx.author().id.get() != 449882524697493515 {
        return Ok(());
    }

    let quest = match quest_id {
        Some(q_id) => helpers::get_quests().get(&q_id).expect("!!Квест не найден"),
        None => {
            let mut rng = rand::rng();
            *helpers::get_quests()
                .values()
                .collect::<Vec<_>>()
                .choose(&mut rng)
                .unwrap()
        }
    };

    helpers::add_quest_to_user(&ctx.data().pool, user_id.get(), quest).await?;

    ctx.say("Успешно!").await?;

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![quests(), add_quest()]
}
