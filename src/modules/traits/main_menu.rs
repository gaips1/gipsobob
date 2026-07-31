use crate::{helpers::{check_user_flag, set_user_flag}, modules::dialogues::get_dialogue, types::*};

fn get_main_menu(traits: &Vec<String>) -> (Vec<serenity::CreateActionRow>, serenity::CreateEmbed) {
    let dialogue = get_dialogue("traits:main_menu").unwrap();
    let all_traits = super::get_traits();

    let traits: Vec<(char, &str)> = traits
        .iter()
        .map(|id| {
            let t = all_traits
                .get(id)
                .map(|s| s.as_str())
                .unwrap();

            let mut chars = t.chars();
            (chars.next().unwrap_or('🟢'), chars.as_str())
        })
        .collect();

    let embed = serenity::CreateEmbed::new()
        .title("Мутации")
        .description(
            format!(
                "{}\n\n\
                🧬 Твои мутации:\n\
                **[ {} Слот 1: {} ]**\n\
                **[ {} Слот 2: {} ]**\n\
                **[ {} Слот 3: {} ]**\n\
                ",
                dialogue.content,
                traits[0].0, traits[0].1,
                traits[1].0, traits[1].1,
                traits[2].0, traits[2].1
            )
        )
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("traits:spin")
            .label("💉 Вколоть жижу (500 бебр)")
            .style(serenity::ButtonStyle::Primary),

        serenity::CreateButton::new("traits:upgrade")
            .label("🔪 Раскроить еще один слот")
            .style(serenity::ButtonStyle::Primary)
    ])];

    (buttons, embed)
}

pub async fn handle_traits_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    if press.data.custom_id.starts_with("traits:mm") {
        let user_traits: Vec<String> = sqlx::query_scalar(
            "SELECT traits FROM users WHERE id = $1"
        )
        .bind(press.user.id.get() as i64)
        .fetch_one(&data.pool)
        .await?;

        let mm = get_main_menu(&user_traits);
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("")
                .components(mm.0)
                .embed(mm.1)
        )
    }

    Ok(())
}

/// Что же доктор вколет в этот раз?
#[poise::command(
    slash_command,
    rename = "мутации",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn traits(ctx: Context<'_>) -> Result<(), Error> {
    if !check_user_flag(&ctx.data().pool, ctx.author().id.get(), "has_traits_opened").await? {
        let dialogue = get_dialogue("traits:first_hi").unwrap();
        ctx.send(
            poise::CreateReply::default()
                .content(dialogue.content)
                .components(dialogue.buttons)
                .ephemeral(true)
        ).await?;

        set_user_flag(&ctx.data().pool, ctx.author().id.get(), "has_traits_opened").await?;

        return Ok(());
    }

    let user_traits: Vec<String> = sqlx::query_scalar(
        "SELECT traits FROM users WHERE id = $1"
    )
    .bind(ctx.author().id.get() as i64)
    .fetch_one(&ctx.data().pool)
    .await?;

    let mm = get_main_menu(&user_traits);
    ctx.send(
        poise::CreateReply::default()
            .components(mm.0)
            .embed(mm.1)
            .ephemeral(true)
    ).await?;

    Ok(())
}