use poise::serenity_prelude as serenity;
use poise::{CreateReply};
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::Decimal;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, QuerySelect, RelationTrait};
use pretty_decimal::PrettyDecimal;

use crate::checks::sbp_check;
use crate::database;
use crate::types::*;
use crate::modules::sbp::register::sbp_register;

/// Твой личный кабинет Системы Быстрых Платежей 
#[poise::command(slash_command, check = "sbp_check", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn account(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;

    let user: (Decimal, bool, i64) = database::sbp_users::Entity::find_by_id::<i64>(ctx.author().id.into())
        .select_only()
        .columns([database::sbp_users::Column::Balance, database::sbp_users::Column::Notifications])
        .column_as(database::sbp_invites::Column::UserId.count(), "invite_count")
        .join_rev(
            JoinType::LeftJoin,
            database::sbp_invites::Relation::SbpUsers1.def()
        )
        .group_by(database::sbp_users::Column::Id)
        .into_tuple()
        .one(db)
        .await?
        .unwrap();

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
    let user_notifications: Option<bool> = database::sbp_users::Entity::find_by_id::<i64>(interaction.user.id.into())
        .select_only()
        .column(database::sbp_users::Column::Notifications)
        .into_tuple()
        .one(&data.db)
        .await?;

    let Some(user_notifications) = user_notifications else {
        return sbp_register(ctx, interaction, data).await;
    };

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        if !user_notifications {
            serenity::CreateButton::new("sbp_notifications_change").label("Выключить уведомления").emoji('✖').style(serenity::ButtonStyle::Danger)
        } else {
            serenity::CreateButton::new("sbp_notifications_change").label("Включить уведомления").emoji('✅').style(serenity::ButtonStyle::Success)
        }
    ])];

    database::sbp_users::ActiveModel {
        id: Set(interaction.user.id.into()),
        notifications: Set(!user_notifications),
        ..Default::default()
    }.update_without_returning(&data.db).await?;

    interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::default().components(buttons)
    )).await?;
    Ok(())
}