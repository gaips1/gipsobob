use poise::CreateReply;
use rand::seq::IndexedRandom;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::ops::Mul;
use tokio::time::sleep;

use crate::checks::sbp_check;
use crate::types::*;

/// Казино "У Снюсоеда"
#[poise::command(
    slash_command,
    rename = "казино",
    check = "sbp_check",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn casino(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::default()
        .title("Добро пожаловать в казино!")
        .description("**Выбирайте игру:**");

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("casino:slots").label("Слоты"),
        serenity::CreateButton::new("casino:guess").label("Угадай число"),
    ])];

    ctx.send(
        CreateReply::default()
            .embed(embed)
            .ephemeral(true)
            .components(buttons),
    )
    .await?;
    Ok(())
}

pub async fn handle_casino_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    match interaction.data.custom_id.as_str() {
        "casino::slots" => {
            handle_slots_button(ctx, interaction, data).await?;
        }

        "casino::guess" => {
            handle_guess_button(ctx, interaction, data).await?;
        }

        _ => {}
    }
    Ok(())
}

async fn handle_slots_button(
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
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Ваша ставка не является числом")
                .ephemeral(true)
        );
        return Ok(());
    };

    if stavka < 300 {
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Минимальная ставка 300 бебр")
                .ephemeral(true)
        );
        return Ok(());
    }

    let stavka: Decimal = stavka.into();
    let emojis_pool = ["7️⃣", "☢️", "#️⃣", "🔥", "⚛️", "🦑", "🧪"];
    let slots: [&str; 3] = {
        let mut rng = rand::rng();
        std::array::from_fn(|_| *emojis_pool.choose(&mut rng).unwrap())
    };

    let win: Option<Decimal> = if slots[0] == slots[1] && slots[1] == slots[2] {
        Some(stavka * Decimal::from_f64(3.5).unwrap())
    } else if slots[0] == slots[1] || slots[1] == slots[2] || slots[0] == slots[2] {
        Some(stavka * Decimal::TWO)
    } else {
        None
    };

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
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("У вас не хватает бебр")
                .ephemeral(true)
        );
        return Ok(());
    };

    let _ = add_user_quest_progress(&data.pool, ctx, interaction.user.id.get(), "casino", None, None).await;

    crate::create_response!(
        ctx,
        response.interaction,
        serenity::CreateInteractionResponseMessage::new()
            .embed(
                serenity::CreateEmbed::default()
                    .title("Спасибо, ставка принята!")
                    .description("Кручу барабан, подождите немного...")
                    .colour(serenity::colours::branding::GREEN)
            )
            .ephemeral(true)
    );

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

async fn handle_guess_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let modal = serenity::CreateQuickModal::new("Угадай число")
        .timeout(std::time::Duration::from_secs(300))
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Ваша ставка", "")
                .max_length(20)
                .value("100"),
        )
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "До какого числа будете угадывать? (включит.)",
                "",
            )
            .max_length(20)
            .value("10"),
        )
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Ваше число", "")
                .max_length(20),
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
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Ваша ставка не является числом")
                .ephemeral(true)
        );
        return Ok(());
    };

    if stavka < 100 {
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Минимальная ставка 100 бебр")
                .ephemeral(true)
        );
        return Ok(());
    }
    let stavka: Decimal = stavka.into();

    let range = response
        .inputs
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u64>();
    let Ok(range) = range else {
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Ваше число не является числом")
                .ephemeral(true)
        );
        return Ok(());
    };

    let number = response
        .inputs
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u64>();
    let Ok(number) = number else {
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Ваше число не является числом")
                .ephemeral(true)
        );
        return Ok(());
    };

    if number < 1 || number > range {
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("Ваше число не входит в указанный диапазон!")
                .ephemeral(true)
        );
        return Ok(());
    }

    let rand_num = rand::random_range(1..=range);
    let win: Option<Decimal> = if rand_num == number {
        Some(
            stavka
                .mul(Decimal::from_u64(range).unwrap())
                .mul(Decimal::from_f64(0.2).unwrap()),
        )
    } else {
        None
    };

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
        crate::create_response!(
            ctx,
            response.interaction,
            serenity::CreateInteractionResponseMessage::new()
                .content("У вас не хватает бебр")
                .ephemeral(true)
        );
        return Ok(());
    };

    let _ = add_user_quest_progress(&data.pool, ctx, interaction.user.id.get(), "casino", None, None).await;

    match win {
        Some(w) => {
            crate::create_response!(
                ctx,
                response.interaction,
                serenity::CreateInteractionResponseMessage::new()
                    .embed(
                        serenity::CreateEmbed::default()
                            .title(format!("Вы выиграли!"))
                            .description(format!("Ваша ставка: {stavka} бебр\nВыигрыш: {w} бебр"))
                            .colour(serenity::colours::branding::GREEN)
                    )
                    .ephemeral(true)
            );
        }

        None => {
            crate::create_response!(
                ctx,
                response.interaction,
                serenity::CreateInteractionResponseMessage::new()
                    .embed(
                        serenity::CreateEmbed::default()
                            .title(format!("Вы проиграли!"))
                            .description(format!(
                                "Я выдумал число {}\nВы могли бы выиграть {} бебр!",
                                rand_num,
                                stavka
                                    .mul(Decimal::from_u64(range).unwrap())
                                    .mul(Decimal::from_f64(0.2).unwrap())
                            ))
                            .colour(serenity::colours::branding::FUCHSIA)
                    )
                    .ephemeral(true)
            );
        }
    }

    Ok(())
}
