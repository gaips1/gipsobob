use std::collections::HashMap;
use crate::{helpers::{check_user_flag, set_user_flag}, modules::{dialogues::get_dialogue, traits::UserTrait}, types::*};

fn format_user_trait(all_traits: &'static HashMap<String, String>, user_trait: &UserTrait) -> String {
    let trait_text = all_traits
        .get(&user_trait.trait_id)
        .unwrap();

    let mut chars = trait_text.chars();

    format!(
        "{} Слот {}: {}",
        chars.next().unwrap_or('🟢'),
        user_trait.slot_index + 1,
        chars.as_str()
    )
}

async fn get_main_menu(pool: &sqlx::PgPool, user_id: u64) -> Result<(Vec<serenity::CreateActionRow>, serenity::CreateEmbed), Error> {
    let dialogue = get_dialogue("traits:main_menu").unwrap();
    let all_traits = super::get_traits();

    let (user_traits, user_unlocked_slots) = tokio::try_join!(
        sqlx::query_as::<_, UserTrait>(
            "SELECT trait_id, slot_index FROM user_traits WHERE user_id = $1 ORDER BY slot_index ASC"
        )
        .bind(user_id as i64)
        .fetch_all(pool),

        sqlx::query_scalar::<_, i16>(
            "SELECT unlocked_traits_slots FROM users WHERE id = $1"
        )
        .bind(user_id as i64)
        .fetch_one(pool)
    )?;

    let [first_trait, second_trait, third_trait] = [0, 1, 2].map(|index|
        user_traits.get(index).cloned().unwrap_or_else(|| UserTrait {
            trait_id: if user_unlocked_slots > index as i16 { "empty" } else { "locked" }.to_string(),
            slot_index: index as i16,
        })
    );

    let embed = serenity::CreateEmbed::new()
        .title("Мутации")
        .description(
            format!(
                "{}\n\n\
                🧬 Твои мутации:\n\
                **[ {} ]**\n\
                **[ {} ]**\n\
                **[ {} ]**\n\
                ",
                dialogue.content,
                format_user_trait(all_traits, &first_trait),
                format_user_trait(all_traits, &second_trait),
                format_user_trait(all_traits, &third_trait)
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

    Ok((buttons, embed))
}

pub async fn handle_traits_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    if press.data.custom_id.starts_with("traits:mm") {
        let mm = get_main_menu(&data.pool, press.user.id.get()).await?;
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

    let mm = get_main_menu(&ctx.data().pool, ctx.author().id.get()).await?;
    ctx.send(
        poise::CreateReply::default()
            .components(mm.0)
            .embed(mm.1)
            .ephemeral(true)
    ).await?;

    Ok(())
}