use rand::seq::IndexedRandom as _;
use rust_decimal::Decimal;

use crate::{
    modules::{
        dialogues,
        traits::{UserTrait, main_menu::format_user_trait},
    },
    types::*,
};

fn get_trait_rarity_weight(trait_text: &str) -> u32 {
    let first_char = trait_text.chars().next().unwrap_or('⚪');
    match first_char {
        '🟡' => 3,
        '🔵' => 12,
        '🟢' => 30,
        '⚪' => 55,
        _ => 50,
    }
}

pub async fn handle_traits_spin_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let all_traits = super::get_traits();
    let user_id = press.user.id.get() as i64;

    let Some(slot) = press.data.custom_id.strip_prefix("traits:spin:") else {
        let rows: Vec<(i16, i16, Option<String>, Option<i16>)> = sqlx::query_as(
            "SELECT \
                u.unlocked_traits_slots, \
                u.spins_today, \
                ut.trait_id, \
                ut.slot_index \
            FROM traits_users u \
            LEFT JOIN user_traits ut ON u.id = ut.user_id \
            WHERE u.id = $1 \
            ORDER BY ut.slot_index ASC",
        )
        .bind(user_id)
        .fetch_all(&data.pool)
        .await?;

        if rows.first().unwrap().1 > 5 {
            let dialogue = dialogues::get_dialogue("traits:spin:max_spins").unwrap();

            let embed = serenity::CreateEmbed::new()
                .title("Мутации")
                .description(dialogue.content)
                .colour(serenity::colours::branding::RED);

            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(dialogue.buttons)
            );

            return Ok(());
        }

        let user_unlocked_slots = rows.first().unwrap().0;
        let user_traits: Vec<UserTrait> = rows
            .into_iter()
            .filter_map(|r| {
                Some(UserTrait {
                    trait_id: r.2?,
                    slot_index: r.3?,
                })
            })
            .collect();

        let buttons: Vec<_> = (0..user_unlocked_slots)
            .map(|slot| {
                let empty = UserTrait {
                    trait_id: "!empty".into(),
                    slot_index: slot,
                };
                let t = user_traits
                    .iter()
                    .find(|t| t.slot_index == slot)
                    .unwrap_or(&empty);

                serenity::CreateButton::new(format!("traits:spin:{slot}"))
                    .label(format_user_trait(all_traits, t, false))
                    .style(serenity::ButtonStyle::Primary)
            })
            .collect();

        let embed = serenity::CreateEmbed::new()
            .title("Мутации")
            .description("Выберите слот, в который хотите вколоть мутацию.\n**Выбранный слот будет перезаписан**")
            .colour(serenity::colours::branding::BLURPLE);

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![
                    serenity::CreateActionRow::Buttons(buttons),
                    serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("traits:mm")
                            .label("Назад")
                            .style(serenity::ButtonStyle::Secondary)
                    ])
                ])
        );
        return Ok(());
    };
    let slot: u16 = slot.parse()?;

    let mut tx = data.pool.begin().await?;

    let rows: (i16, i16) = sqlx::query_as(
        "SELECT unlocked_traits_slots, spins_today \
        FROM traits_users WHERE id = $1 FOR UPDATE",
    )
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    if slot + 1 > rows.0 as u16 {
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Кус, читерок")
                .embeds(Vec::new())
                .components(Vec::new())
        );
        return Ok(());
    }

    if rows.1 > 5 {
        tx.rollback().await?;

        let dialogue = dialogues::get_dialogue("traits:spin:max_spins").unwrap();

        let embed = serenity::CreateEmbed::new()
            .title("Мутации")
            .description(dialogue.content)
            .colour(serenity::colours::branding::RED);

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(dialogue.buttons)
        );

        return Ok(());
    }

    let user_balance: Decimal =
        sqlx::query_scalar("SELECT balance FROM sbp_users WHERE id = $1 FOR UPDATE")
            .bind(press.user.id.get() as i64)
            .fetch_one(&mut *tx)
            .await?;

    if user_balance < Decimal::from(500) {
        tx.rollback().await?;

        let dialogue = dialogues::get_dialogue("traits:not_enough_money").unwrap();

        let embed = serenity::CreateEmbed::new()
            .title("Мутации")
            .description(dialogue.content)
            .colour(serenity::colours::branding::RED);

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(dialogue.buttons)
        );
        return Ok(());
    }

    sqlx::query("UPDATE sbp_users SET balance = balance - 500 WHERE id = $1")
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    let non_system_traits: Vec<_> = all_traits
        .iter()
        .filter(|t| !t.0.starts_with("!"))
        .collect();
    let random_trait = {
        let mut rng = rand::rng();
        non_system_traits
            .choose_weighted(&mut rng, |t| get_trait_rarity_weight(t.1))
            .expect("traits list is empty")
    };

    let is_inserted: bool = sqlx::query_scalar(
        "INSERT INTO user_traits (user_id, slot_index, trait_id) \
        VALUES ($1, $2, $3) \
        ON CONFLICT (user_id, slot_index) \
        DO UPDATE SET trait_id = EXCLUDED.trait_id \
        RETURNING (xmax = 0) AS is_created;",
    )
    .bind(user_id)
    .bind(slot as i16)
    .bind(random_trait.0)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("UPDATE traits_users SET spins_today = spins_today + 1 WHERE id = $1")
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    let text = if is_inserted {
        "💉 Доктор Хальцер набирает в шприц зелёную жидкость из 3-литровой банки...\n\n".to_string()
    } else {
        "🪚 Доктор Хальцер достает клизму и огромный отсос...\n\n".to_string()
    };

    let mut embed = serenity::CreateEmbed::new()
        .title("[▱▱▱▱▱▱▱▱▱▱▱▱▱▱]")
        .description(&text)
        .colour(serenity::colours::branding::BLURPLE);

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .embed(embed.clone())
            .components(Vec::new())
    );

    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    embed = embed.title("[▰▰▰▰▱▱▱▱▱▱▱▱▱▱]");

    press
        .edit_response(
            ctx,
            serenity::EditInteractionResponse::new().embed(embed.clone()),
        )
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;

    embed = embed.title("[▰▰▰▰▰▰▰▱▱▱▱▱▱▱]");

    press
        .edit_response(
            ctx,
            serenity::EditInteractionResponse::new().embed(embed.clone()),
        )
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_millis(900)).await;

    embed = embed.title("[▰▰▰▰▰▰▰▰▰▰▰▱▱▱]");

    press
        .edit_response(
            ctx,
            serenity::EditInteractionResponse::new().embed(embed.clone()),
        )
        .await?;

    tokio::time::sleep(tokio::time::Duration::from_millis(1_000)).await;

    let dialogue = dialogues::get_dialogue_with_vars(
        if is_inserted {
            "traits:spin:empty"
        } else {
            "traits:spin:replace"
        },
        &[("trait_name", random_trait.1)],
    )
    .unwrap();

    embed = embed
        .title("[▰▰▰▰▰▰▰▰▰▰▰▰▰▰]")
        .description(text + &dialogue.content)
        .colour(serenity::colours::branding::GREEN);

    press
        .edit_response(
            ctx,
            serenity::EditInteractionResponse::new()
                .embed(embed)
                .components(dialogue.buttons),
        )
        .await?;

    Ok(())
}
