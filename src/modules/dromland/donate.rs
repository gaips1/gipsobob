use super::types::*;
use crate::types::*;

use pretty_decimal::PrettyDecimal;
use rust_decimal::{Decimal, prelude::FromPrimitive as _};

pub async fn handle_donate_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    _dl_user: DlUser,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.as_str();

    match custom_id {
        "dl:donate" => {
            let embed = serenity::CreateEmbed::default()
                .title("Донатик")
                .description(
                    "Добро пожаловать в меню доната!
                    Чтобы перевести деньги из баланса **Дромляндии: Онлайн** на свой счёт СБП, нажмите кнопку `ДО в СБП`.
                    Чтобы перевести бебры из баланса **СБП** на свой счёт Дромляндии: Онлайн, нажмите кнопку `СБП в ДО`.
                    Курс перевода из ДО в СБП - 20 к 1
                    Курс перевода из СБП в ДО - 1 к 1.5"
                )
                .footer(serenity::CreateEmbedFooter::new("Дромляндия: Онлайн"))
                .colour(serenity::colours::branding::BLURPLE);

            let buttons = vec![
                serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("dl:donate:to_sbp")
                        .label("ДО в СБП")
                        .style(serenity::ButtonStyle::Success),
                    serenity::CreateButton::new("dl:donate:from_sbp")
                        .label("СБП в ДО")
                        .style(serenity::ButtonStyle::Success),
                ]),
                serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("dl:mm").label("Назад"),
                ]),
            ];

            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::default()
                    .content("")
                    .embed(embed)
                    .components(buttons)
                    .ephemeral(true)
            );
        }

        "dl:donate:to_sbp" => {
            let pool = &data.pool;

            let modal = serenity::CreateQuickModal::new("Перевод ДО в СБП (20 к 1)")
                .timeout(std::time::Duration::from_secs(300))
                .field(
                    serenity::CreateInputText::new(
                        serenity::InputTextStyle::Short,
                        "Введи сумму в ДО (20 ДО к 1 СБП)",
                        "",
                    )
                    .min_length(3),
                );

            let response = press.quick_modal(ctx, modal).await?;

            let Some(response) = response else {
                return Ok(());
            };

            let Ok(amount) = Decimal::from_str_exact(response.inputs[0].trim()) else {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Введите число")
                        .ephemeral(true)
                );
                return Ok(());
            };

            if amount.is_zero() || amount.is_sign_negative() {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Введите нормальное число")
                        .ephemeral(true)
                );
                return Ok(());
            }

            if amount.lt(&Decimal::from_i32(100).unwrap()) {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Минимальный перевод: 100 монет")
                        .ephemeral(true)
                );
                return Ok(());
            }

            let is_sbp_user_exists: bool =
                sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM sbp_users WHERE id = $1)")
                    .bind(response.interaction.user.id.get() as i64)
                    .fetch_one(pool)
                    .await?;

            if !is_sbp_user_exists {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content(
                            "Сначала зарегистрируйся в СБП, используя команду `/сбп регистрация`"
                        )
                        .ephemeral(true)
                );
                return Ok(());
            }

            let mut tx = pool.begin().await?;

            let user_balance: Decimal =
                sqlx::query_scalar("SELECT balance FROM dl_users WHERE id = $1 FOR UPDATE")
                    .bind(response.interaction.user.id.get() as i64)
                    .fetch_one(&mut *tx)
                    .await?;

            if user_balance.lt(&amount) {
                tx.rollback().await?;
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("У вас недостаточно монет.")
                        .ephemeral(true)
                );
                return Ok(());
            }

            let sbp_amount = amount / Decimal::from_i32(20).unwrap();

            sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
                .bind(sbp_amount)
                .bind(response.interaction.user.id.get() as i64)
                .execute(&mut *tx)
                .await?;

            sqlx::query("UPDATE dl_users SET balance = balance - $1 WHERE id = $2")
                .bind(amount)
                .bind(response.interaction.user.id.get() as i64)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            crate::create_response!(
                ctx,
                response.interaction,
                serenity::CreateInteractionResponseMessage::default()
                    .content(format!(
                        "Успешно! Перевёл `{}` бебр на ваш счёт СБП",
                        PrettyDecimal::comma3dot(sbp_amount)
                    ))
                    .ephemeral(true)
            );
        }

        "dl:donate:from_sbp" => {
            let pool = &data.pool;

            let modal = serenity::CreateQuickModal::new("Перевод СБП в ДО (1 к 1.5)")
                .timeout(std::time::Duration::from_secs(300))
                .field(
                    serenity::CreateInputText::new(
                        serenity::InputTextStyle::Short,
                        "Введи сумму (курс: 1 СБП к 1.5 ДО)",
                        "",
                    )
                    .min_length(1),
                );

            let response = press.quick_modal(ctx, modal).await?;

            let Some(response) = response else {
                return Ok(());
            };

            let Ok(amount) = Decimal::from_str_exact(response.inputs[0].trim()) else {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Введите число")
                        .ephemeral(true)
                );
                return Ok(());
            };

            if amount.is_zero() || amount.is_sign_negative() {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Введите нормальное число")
                        .ephemeral(true)
                );
                return Ok(());
            }

            let is_sbp_user_exists: bool =
                sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM sbp_users WHERE id = $1)")
                    .bind(response.interaction.user.id.get() as i64)
                    .fetch_one(pool)
                    .await?;

            if !is_sbp_user_exists {
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content(
                            "Сначала зарегистрируйся в СБП, используя команду `/сбп регистрация`"
                        )
                        .ephemeral(true)
                );
                return Ok(());
            }

            let mut tx = pool.begin().await?;

            let sbp_balance: Decimal =
                sqlx::query_scalar("SELECT balance FROM sbp_users WHERE id = $1 FOR UPDATE")
                    .bind(response.interaction.user.id.get() as i64)
                    .fetch_one(&mut *tx)
                    .await?;

            if sbp_balance.lt(&amount) {
                tx.rollback().await?;
                crate::create_response!(
                    ctx,
                    response.interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("У вас недостаточно бебр.")
                        .ephemeral(true)
                );
                return Ok(());
            }

            let dl_amount = amount * Decimal::from_f64(1.5).unwrap();

            sqlx::query("UPDATE sbp_users SET balance = balance - $1 WHERE id = $2")
                .bind(amount)
                .bind(response.interaction.user.id.get() as i64)
                .execute(&mut *tx)
                .await?;

            sqlx::query("UPDATE dl_users SET balance = balance + $1 WHERE id = $2")
                .bind(dl_amount)
                .bind(response.interaction.user.id.get() as i64)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            crate::create_response!(
                ctx,
                response.interaction,
                serenity::CreateInteractionResponseMessage::default()
                    .content(format!(
                        "Успешно! Перевёл `{}` монет на ваш счёт Дромляндии: Онлайн",
                        PrettyDecimal::comma3dot(dl_amount)
                    ))
                    .ephemeral(true)
            );
        }

        _ => {}
    }

    Ok(())
}
