use poise::serenity_prelude as serenity;
use poise::{CreateReply};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use pretty_decimal::PrettyDecimal;

use crate::checks::sbp_check;
use crate::database;
use crate::types::*;
use crate::modules::sbp::register::sbp_register;

/// Твой личный кабинет Системы Быстрых Платежей 
#[poise::command(slash_command, check = "sbp_check", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn account(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;

    let user = database::sbp_users::Entity::find_by_id::<i64>(ctx.author().id.into())
        .one(db)
        .await?
        .unwrap();

    let embed = serenity::CreateEmbed::default()
        .title(format!("Личный кабинет: {}", ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name)))
        .description(format!(
            "**Добро пожаловать в Систему Быстрых Платежей, {}
            Баланс: {} бебр(ы)
            **",
            ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name),
            PrettyDecimal::comma3dot(user.balance),
        ));

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        if user.notifications {
            serenity::CreateButton::new("sbp_notifications_change").label("Выключить уведомления").emoji('✖').style(serenity::ButtonStyle::Danger)
        } else {
            serenity::CreateButton::new("sbp_notifications_change").label("Включить уведомления").emoji('✅').style(serenity::ButtonStyle::Success)
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
    let user = database::sbp_users::Entity::find_by_id::<i64>(interaction.user.id.into())
        .one(&data.db)
        .await?;

    let Some(user) = user else {
        return sbp_register(ctx, interaction, data).await;
    };

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        if !user.notifications {
            serenity::CreateButton::new("sbp_notifications_change").label("Выключить уведомления").emoji('✖').style(serenity::ButtonStyle::Danger)
        } else {
            serenity::CreateButton::new("sbp_notifications_change").label("Включить уведомления").emoji('✅').style(serenity::ButtonStyle::Success)
        }
    ])];

    let mut active_user = user.into_active_model();
    active_user.notifications = Set(!active_user.notifications.unwrap());
    active_user.update(&data.db).await?;

    interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::default().components(buttons)
    )).await?;
    Ok(())
}