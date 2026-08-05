use crate::{
    modules::{dialogues::get_dialogue, traits::UserTrait},
    types::*,
};
use std::collections::HashMap;

pub fn format_user_trait(
    all_traits: &'static HashMap<String, String>,
    user_trait: &UserTrait,
    with_text: bool,
) -> String {
    let trait_text = all_traits.get(&user_trait.trait_id).unwrap();

    let mut chars = trait_text.chars();
    let emoji = chars.next().unwrap_or('🟢');
    let slot = user_trait.slot_index + 1;

    let result = if with_text {
        format!("{emoji} Слот {slot}: {}", chars.as_str())
    } else {
        format!("{emoji} Слот {slot}")
    };

    result
}

async fn get_main_menu(
    pool: &sqlx::PgPool,
    user_id: u64,
) -> Result<(Vec<serenity::CreateActionRow>, serenity::CreateEmbed), Error> {
    let dialogue = get_dialogue("traits:main_menu").unwrap();
    let all_traits = super::get_traits();

    let rows: Vec<(i16, Option<String>, Option<i16>)> = sqlx::query_as(
        "SELECT \
            u.unlocked_traits_slots, \
            ut.trait_id, \
            ut.slot_index \
        FROM traits_users u \
        LEFT JOIN user_traits ut ON u.id = ut.user_id \
        WHERE u.id = $1 \
        ORDER BY ut.slot_index ASC",
    )
    .bind(user_id as i64)
    .fetch_all(pool)
    .await?;

    let user_unlocked_slots = rows.first().unwrap().0;
    let user_traits: Vec<UserTrait> = rows
        .into_iter()
        .filter_map(|r| {
            Some(UserTrait {
                trait_id: r.1?,
                slot_index: r.2?,
            })
        })
        .collect();

    let [first_trait, second_trait, third_trait] = [0, 1, 2].map(|index| {
        user_traits
            .iter()
            .find(|t| t.slot_index == index as i16)
            .cloned()
            .unwrap_or_else(|| UserTrait {
                trait_id: if user_unlocked_slots > index as i16 {
                    "!empty"
                } else {
                    "!locked"
                }
                .to_string(),
                slot_index: index as i16,
            })
    });

    let embed = serenity::CreateEmbed::new()
        .title("Мутации")
        .description(format!(
            "{}\n\n\
                🧬 Твои мутации:\n\
                **[ {} ]**\n\
                **[ {} ]**\n\
                **[ {} ]**\n\
                ",
            dialogue.content,
            format_user_trait(all_traits, &first_trait, true),
            format_user_trait(all_traits, &second_trait, true),
            format_user_trait(all_traits, &third_trait, true)
        ))
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("traits:spin")
                .label("💉 Вколоть жижу (500 бебр)")
                .style(serenity::ButtonStyle::Primary),
            serenity::CreateButton::new("traits:upgrade")
                .label("🔪 Раскроить еще один слот (3000 бебр)")
                .style(serenity::ButtonStyle::Primary)
        ]),
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("traits:collection")
                .label("🏆 Ваша коллекция мутаций")
                .style(serenity::ButtonStyle::Success)
        ])
    ];

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
    } else if press.data.custom_id.starts_with("traits:spin") {
        super::spin::handle_traits_spin_button(ctx, press, data).await?
    } else if press.data.custom_id == "traits:upgrade" {
        super::upgrade::handle_traits_upgrade_button(ctx, press, data).await?
    } else if press.data.custom_id == "traits:collection" {
        super::collection::handle_traits_collection_button(ctx, press, data).await?
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
    let is_traits_user_exists: bool =
        sqlx::query_scalar("SELECT EXISTS ( SELECT 1 FROM traits_users WHERE id = $1 )")
            .bind(ctx.author().id.get() as i64)
            .fetch_one(&ctx.data().pool)
            .await?;

    if !is_traits_user_exists {
        sqlx::query("INSERT INTO traits_users (id) VALUES ($1)")
            .bind(ctx.author().id.get() as i64)
            .execute(&ctx.data().pool)
            .await?;

        let dialogue = get_dialogue("traits:first_hi").unwrap();
        ctx.send(
            poise::CreateReply::default()
                .content(dialogue.content)
                .components(dialogue.buttons)
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let mm = get_main_menu(&ctx.data().pool, ctx.author().id.get()).await?;
    ctx.send(
        poise::CreateReply::default()
            .components(mm.0)
            .embed(mm.1)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
