use rand::seq::IndexedRandom;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use tokio::time::sleep;
use crate::{modules::traits::get_user_traits, types::*};

pub async fn handle_slots_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let modal = serenity::CreateQuickModal::new("Слоты")
        .timeout(std::time::Duration::from_secs(300))
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Ваша ставка:", "")
                .max_length(20)
                .value("300"),
        );
    let response = interaction.quick_modal(ctx, modal).await?;
    let Some(response) = response else {
        return Ok(());
    };

    let stavka = response
        .inputs
        .first()
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u64>();
    let Ok(stavka) = stavka else {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Ваша ставка не является числом")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    // 🟢 cheap_date: -5% к минимальной ставке в казино
    let user_traits = get_user_traits(&data.pool, interaction.user.id.get()).await?;
    let has_cheap_date = user_traits.contains(&"cheap_date".to_string());
    let min_slots_bet: u64 = if has_cheap_date { 285 } else { 300 };

    if stavka < min_slots_bet {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content(format!("Минимальная ставка {min_slots_bet} бебр"))
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    }

    let stavka: Decimal = stavka.into();
    let emojis_pool = ["7️⃣", "☢️", "#️⃣", "🔥", "⚛️", "🦑", "🧪"];
    let mut slots: [&str; 3] = {
        let mut rng = rand::rng();
        std::array::from_fn(|_| *emojis_pool.choose(&mut rng).unwrap())
    };

    // 🟢 gambler: +3% шанс, что казино сжалится и подгонит третий символ под первые два
    if user_traits.contains(&"gambler".to_string())
        && slots[0] != slots[1]
        && slots[1] != slots[2]
        && slots[0] != slots[2]
        && rand::random_bool(0.03)
    {
        slots[2] = slots[0];
    }

    let mut win: Option<Decimal> = if slots[0] == slots[1] && slots[1] == slots[2] {
        Some(stavka * Decimal::from_f64(3.5).unwrap())
    } else if slots[0] == slots[1] || slots[1] == slots[2] || slots[0] == slots[2] {
        Some(stavka * Decimal::TWO)
    } else {
        None
    };

    // 🟡 casino_king: 3% шанс при победе в слотах сорвать куш x2
    if let Some(w) = win {
        if user_traits.contains(&"casino_king".to_string()) && rand::random_bool(0.03) {
            win = Some(w * Decimal::TWO);
        }
    }

    let delta = match win {
        Some(w) => w - stavka,
        None => -stavka,
    };

    let result = sqlx::query(
        "UPDATE sbp_users
         SET balance = balance + $1
         WHERE id = $2 AND balance >= $3",
    )
    .bind(delta)
    .bind::<i64>(interaction.user.id.into())
    .bind(stavka)
    .execute(&data.pool)
    .await?;

    if result.rows_affected() == 0 {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("У вас не хватает бебр")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    let _ = add_user_quest_progress(
        &data.pool,
        ctx,
        interaction.user.id.get(),
        "casino",
        None,
        None,
    )
    .await;

    response
        .interaction
        .reply(
            ctx,
            serenity::CreateInteractionResponseMessage::new()
                .embed(
                    serenity::CreateEmbed::default()
                        .title("Спасибо, ставка принята!")
                        .description("Кручу барабан, подождите немного...")
                        .colour(serenity::colours::branding::GREEN),
                )
                .ephemeral(true),
        )
        .await?;

    let mut revealed = String::new();
    for emoji in &slots {
        sleep(std::time::Duration::from_millis(2_000)).await;

        revealed += emoji;

        response
            .interaction
            .edit_response(
                ctx,
                serenity::EditInteractionResponse::new()
                    .embed(serenity::CreateEmbed::default().title(&revealed)),
            )
            .await?;

        sleep(std::time::Duration::from_millis(1_000)).await;
    }

    let full_slots = slots.concat();
    match win {
        Some(win) => {
            response
                .interaction
                .edit_response(
                    ctx,
                    serenity::EditInteractionResponse::new().embed(
                        serenity::CreateEmbed::default()
                            .title(format!("Вы выиграли! {full_slots}"))
                            .description(format!("Ваша ставка: {stavka} бебр\nВыигрыш: {win} бебр"))
                            .colour(serenity::colours::branding::GREEN),
                    ),
                )
                .await?;
        }
        None => {
            response
                .interaction
                .edit_response(
                    ctx,
                    serenity::EditInteractionResponse::new().embed(
                        serenity::CreateEmbed::default()
                            .title(format!("Вы проиграли! {full_slots}"))
                            .description(format!(
                                "Вы могли бы выиграть {} бебр!",
                                stavka * Decimal::from_f64(3.5).unwrap()
                            ))
                            .colour(serenity::colours::branding::FUCHSIA),
                    ),
                )
                .await?;
        }
    }

    Ok(())
}