use crate::types::*;
use poise::serenity_prelude::{self as serenity};
use rust_decimal::{Decimal, prelude::FromPrimitive};

pub async fn handle_shop_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let modal = serenity::CreateQuickModal::new("Что желаешь купить, путник?")
        .timeout(std::time::Duration::from_secs(300))
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Всё по 399 монет! (хп/урон/мана)",
                "",
            )
            .max_length(5)
            .min_length(2),
        );

    let response = press.quick_modal(ctx, modal).await?;

    let Some(response) = response else {
        return Ok(());
    };
    let press = response.interaction;

    let query = match response.inputs[0].trim() {
        "хп" => "UPDATE dl_users SET health = health + 10 WHERE id = $1",
        "урон" => "UPDATE dl_users SET damage = damage + 10 WHERE id = $1",
        "мана" => "UPDATE dl_users SET mana = mana + 10 WHERE id = $1",

        _ => {
            crate::create_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Неизвестный товар. Доступные товары: хп | урон | мана")
                    .ephemeral(true)
            );
            return Ok(());
        }
    };

    let pool = &data.pool;
    let mut tx = pool.begin().await?;

    let balance: Option<Decimal> =
        sqlx::query_scalar("SELECT balance FROM dl_users WHERE id = $1 FOR UPDATE")
            .bind(press.user.id.get() as i64)
            .fetch_optional(&mut *tx)
            .await?;

    let Some(balance) = balance else {
        tx.rollback().await?;
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Сначала создай персонажа")
                .ephemeral(true)
        );
        return Ok(());
    };

    if balance < Decimal::from_u16(399).unwrap() {
        tx.rollback().await?;
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("У вас не хватает денег!")
                .ephemeral(true)
        );
        return Ok(());
    }

    sqlx::query("UPDATE dl_users SET balance = balance - 399 WHERE id = $1")
        .bind(press.user.id.get() as i64)
        .execute(&mut *tx)
        .await?;

    sqlx::query(query)
        .bind(press.user.id.get() as i64)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    crate::create_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("Спасибо за покупку!")
            .ephemeral(true)
    );

    Ok(())
}
