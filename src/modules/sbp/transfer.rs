use poise::serenity_prelude::utils::CreateQuickModal;
use pretty_decimal::PrettyDecimal;
use rust_decimal::Decimal;

use crate::checks::sbp_check;
use crate::modules::sbp::USER_UNAUTHORIZED_ERROR;
use crate::types::*;

async fn transfer(
    ctx: Context<'_>,
    user: serenity::User,
    amount: f64,
    comment: Option<String>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let amount = match Decimal::try_from(amount) {
        Ok(d) => d.round_dp(2),
        Err(_) => {
            ctx.say("Указана некорректная сумма").await?;
            return Ok(());
        }
    };

    if user.bot {
        ctx.say("Нельзя переводить бебры боту, баранище!!!!!!!!!!!!!")
            .await?;
        return Ok(());
    }

    if amount.is_zero() || amount.is_sign_negative() {
        ctx.say("Пожалуйста, введите положительное или не нулевое число бебр")
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.say("Нельзя переводить бебры самому себе").await?;
        return Ok(());
    }

    if let Some(c) = &comment {
        if c.len() > 50 {
            ctx.say("Длина комментария к переводу не может превышать 50 символов")
                .await?;
            return Ok(());
        }
    }

    let mut tx = pool.begin().await?;

    let author_balance: Decimal =
        sqlx::query_scalar("SELECT balance FROM sbp_users WHERE id = $1 FOR UPDATE")
            .bind::<i64>(ctx.author().id.into())
            .fetch_one(&mut *tx)
            .await?;

    if author_balance < amount {
        ctx.say("У вас не хватает бебр").await?;
        return Ok(());
    }

    let user_sbp: Option<(Decimal, bool)> =
        sqlx::query_as("SELECT balance, notifications FROM sbp_users WHERE id = $1 FOR UPDATE")
            .bind::<i64>(user.id.into())
            .fetch_optional(&mut *tx)
            .await?;

    let Some(user_sbp) = user_sbp else {
        tx.rollback().await?;
        ctx.say(USER_UNAUTHORIZED_ERROR).await?;
        return Ok(());
    };

    sqlx::query("UPDATE sbp_users SET balance = balance - $1 WHERE id = $2")
        .bind(amount)
        .bind::<i64>(ctx.author().id.into())
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
        .bind(amount)
        .bind::<i64>(user.id.into())
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    let _ = ctx
        .say(format!(
            "Успешно перевёл {} бебр {}",
            PrettyDecimal::comma3dot(amount),
            user.display_name()
        ))
        .await;

    if user_sbp.1 {
        let mut embed = serenity::CreateEmbed::default()
            .title(format!(
                "Получен перевод от {} суммой {} бебр.",
                ctx.author().display_name(),
                PrettyDecimal::comma3dot(amount)
            ))
            .colour(serenity::colours::branding::GREEN);

        if let Some(c) = comment {
            embed = embed.description(format!("Комментарий от отправителя: ```{c}```"));
        }

        let _ = user
            .dm(&ctx, serenity::CreateMessage::default().embed(embed))
            .await;
    }

    Ok(())
}

/// Отправить бебры пользователю с помощью СБП
#[poise::command(
    slash_command,
    ephemeral,
    rename = "transfer",
    check = "sbp_check",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn transfer_slash_command(
    ctx: Context<'_>,
    #[description = "Кому переводить"] user: serenity::User,
    #[description = "Сумма в бебрах"] amount: f64,
    #[description = "Комментарий к переводу"] comment: Option<String>,
) -> Result<(), Error> {
    transfer(ctx, user, amount, comment).await?;
    Ok(())
}

/// Отправить бебры пользователю с помощью СБП
#[poise::command(
    context_menu_command = "Перевод бебр",
    check = "sbp_check",
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn transfer_context_menu_command(
    ctx: Context<'_>,
    #[description = "Кому переводить"] user: serenity::User,
) -> Result<(), Error> {
    let poise::Context::Application(app_ctx) = ctx else {
        return Ok(());
    };

    let modal = CreateQuickModal::new(format!("Перевод бебр пользователю {}", user.display_name()))
        .timeout(std::time::Duration::from_secs(300))
        .field(serenity::CreateInputText::new(
            serenity::InputTextStyle::Short,
            "Сумма перевода",
            "",
        ))
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Комментарий к переводу",
                "",
            )
            .max_length(50)
            .required(false),
        );

    let response = app_ctx
        .interaction
        .quick_modal(ctx.serenity_context(), modal)
        .await?;

    app_ctx
        .has_sent_initial_response
        .store(true, std::sync::atomic::Ordering::SeqCst);

    if let Some(response) = response {
        let Ok(amount) = response.inputs[0].parse::<f64>() else {
            response
                .interaction
                .create_response(
                    ctx.serenity_context(),
                    serenity::CreateInteractionResponse::Acknowledge,
                )
                .await?;
            ctx.say("Введите корректное число").await?;
            return Ok(());
        };

        let comment: Option<String> = Some(response.inputs[1].clone()).filter(|s| !s.is_empty());

        response
            .interaction
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::Acknowledge,
            )
            .await?;
        transfer(ctx, user, amount, comment).await?;
    }

    Ok(())
}
