use poise::Modal;
use rust_decimal::Decimal;
use std::{borrow::Cow, str::FromStr as _};

use crate::{
    modules::mining::{exchange_rate::ExchangeRate, helpers::bar},
    types::*,
};

pub async fn handle_trading_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.as_str();

    match custom_id {
        "mining:trading" => handle_trading_button(ctx, press, mining_user).await?,
        "mining:trading:trade" => {
            handle_trading_trade_button(ctx, press, data, mining_user).await?
        }
        _ => {}
    }

    Ok(())
}

pub async fn handle_trading_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let exchange_rate = ExchangeRate::get();

    let embed = serenity::CreateEmbed::new()
        .title("💱 Обменник UCS → Бебры")
        .field(
            "Текущий курс",
            format!("1 UCS = {exchange_rate} бебр"),
            false,
        )
        .field(
            "Ваш лимит сегодня",
            format!(
                "{} {}/{} бебр",
                bar(
                    mining_user.traded_today.as_f64(),
                    mining_user.location.trading_limit as f64,
                    10
                ),
                mining_user.traded_today,
                mining_user.location.trading_limit
            ),
            false,
        )
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("mining:mm")
            .label("Назад")
            .style(serenity::ButtonStyle::Danger),
        serenity::CreateButton::new("mining:trading:trade")
            .label("Обменять")
            .style(serenity::ButtonStyle::Success)
            .disabled(
                mining_user.traded_today >= Decimal::from(mining_user.location.trading_limit),
            ),
    ])];

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("")
            .embed(embed)
            .components(buttons)
    );

    Ok(())
}

#[derive(Debug, Modal)]
#[name = "💱 Обменник UCS → Бебры"]
struct TradeModal {
    #[name = "Количество UCS"]
    #[placeholder = "10.1"]
    amount: String,
}

pub async fn handle_trading_trade_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    if mining_user.traded_today >= Decimal::from(mining_user.location.trading_limit) {
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Превышен лимит на обмен валют в день")
                .ephemeral(true)
        );
        return Ok(());
    }

    let modal_response = poise::execute_modal_on_component_interaction::<TradeModal>(
        Cow::Borrowed(ctx),
        press.clone(),
        None,
        None,
    )
    .await?;

    if let Some(trade) = modal_response {
        let Ok(amount) = Decimal::from_str(&trade.amount) else {
            press
                .create_followup(
                    &ctx.http,
                    serenity::CreateInteractionResponseFollowup::new()
                        .content("Введите число")
                        .ephemeral(true),
                )
                .await?;
            return Ok(());
        };

        if amount.is_zero() || amount.is_sign_negative() {
            press
                .create_followup(
                    &ctx.http,
                    serenity::CreateInteractionResponseFollowup::new()
                        .content("Введите положительное число")
                        .ephemeral(true),
                )
                .await?;
            return Ok(());
        }

        let exchange_rate = ExchangeRate::get();

        let mut tx = data.pool.begin().await?;
        let user: (Decimal, Decimal) = sqlx::query_as(
            "SELECT balance, traded_today FROM mining_users WHERE id = $1 FOR UPDATE",
        )
        .bind(press.user.id.get() as i64)
        .fetch_one(&mut *tx)
        .await?;

        if user.0 < amount {
            tx.rollback().await?;
            press
                .create_followup(
                    &ctx.http,
                    serenity::CreateInteractionResponseFollowup::new()
                        .content("У вас не хватает UCS")
                        .ephemeral(true),
                )
                .await?;
            return Ok(());
        }

        let bebrs = amount * exchange_rate;
        if (user.1 + bebrs) > Decimal::from(mining_user.location.trading_limit) {
            tx.rollback().await?;
            press
                .create_followup(
                    &ctx.http,
                    serenity::CreateInteractionResponseFollowup::new()
                        .content("Превышен лимит на обмен валют в день")
                        .ephemeral(true),
                )
                .await?;
            return Ok(());
        }

        sqlx::query("UPDATE mining_users SET balance = balance - $1, traded_today = traded_today + $2 WHERE id = $3")
            .bind(amount)
            .bind(bebrs)
            .bind(press.user.id.get() as i64)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
            .bind(bebrs)
            .bind(press.user.id.get() as i64)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        press.create_followup(&ctx.http,
            serenity::CreateInteractionResponseFollowup::new()
                .content(format!("Успешно перевёл `{amount}` UCS в `{bebrs}` бебр по курсу `{exchange_rate}` бебр за 1 UCS"))
                .ephemeral(true)
        ).await?;
    }

    Ok(())
}
