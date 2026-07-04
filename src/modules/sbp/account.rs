use poise::serenity_prelude as serenity;
use poise::{CreateReply};
use pretty_decimal::PrettyDecimal;
use rust_decimal::Decimal;

use crate::checks::sbp_check;
use crate::types::*;
use crate::modules::sbp::register::sbp_register;

/// Твой личный кабинет Системы Быстрых Платежей 
#[poise::command(slash_command, check = "sbp_check", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn account(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let user: (Decimal, bool, i64) = sqlx::query_as(
        "SELECT \
            u.balance, \
            u.notifications, \
            COUNT(i.user_id) \
        FROM sbp_users u \
        LEFT JOIN sbp_invites i ON i.user_id = u.id \
        WHERE u.id = $1 \
        GROUP BY u.id;"
    )
        .bind::<i64>(ctx.author().id.into())
        .fetch_one(pool)
        .await?;

    let embed = serenity::CreateEmbed::default()
        .title(format!("Личный кабинет: {}", ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name)))
        .description(format!(
            "**Добро пожаловать в Систему Быстрых Платежей, {}
            Баланс: {} бебр(ы)
            Вы пригласили {} пользователей в СБП (`/invite`)
            **",
            ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name),
            PrettyDecimal::comma3dot(user.0),
            user.2
        ));

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        if user.1 {
            serenity::CreateButton::new("sbp:notifications_change").label("Выключить уведомления").emoji('✖').style(serenity::ButtonStyle::Danger)
        } else {
            serenity::CreateButton::new("sbp:notifications_change").label("Включить уведомления").emoji('✅').style(serenity::ButtonStyle::Success)
        }
    ])];

    ctx.send(CreateReply::default().embed(embed).ephemeral(true).components(buttons)).await?;
    Ok(())
}

pub async fn handle_change_notifications_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data
) -> Result<(), Error> {
    let new_notifications: Option<bool> = sqlx::query_scalar(
        "UPDATE sbp_users SET notifications = NOT notifications WHERE id = $1 RETURNING notifications"
    )
        .bind::<i64>(interaction.user.id.into())
        .fetch_optional(&data.pool)
        .await?;

    let Some(new_notifications) = new_notifications else {
        return sbp_register(ctx, interaction, data).await;
    };

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        if new_notifications {
            serenity::CreateButton::new("sbp:notifications_change")
                .label("Выключить уведомления")
                .emoji('✖')
                .style(serenity::ButtonStyle::Danger)
        } else {
            serenity::CreateButton::new("sbp:notifications_change")
                .label("Включить уведомления")
                .emoji('✅')
                .style(serenity::ButtonStyle::Success)
        }
    ])];

    interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::default().components(buttons)
    )).await?;
    
    Ok(())
}