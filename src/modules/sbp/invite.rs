use poise::serenity_prelude::Mentionable;
use poise::{serenity_prelude as serenity};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, ExprTrait, QueryFilter, QuerySelect, SelectExt};
use sea_orm::TransactionTrait;
use poise::{CreateReply};

use crate::buttons::handle_button;
use crate::checks::sbp_check;
use crate::database::{sbp_invites, sbp_users};
use crate::types::*;

/// Пригласить друзей в СБП и получить бебры
#[poise::command(slash_command, check = "sbp_check", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn invite(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::default()
        .title("Приглашение зарегистрироваться в СБП")
        .description("Чтобы принять, нажмите на кнопку ниже");

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(&ctx.id().to_string())
            .label("Принять приглашение")
    ])];

    let msg = ctx.send(CreateReply::default().embed(embed).components(buttons)).await?;

    handle_button(ctx, &ctx.id().to_string(), 3600,
        move |press| {
            async move {
                let db = &ctx.data().db;

                let is_sbp_user_exists = sbp_users::Entity::find()
                    .select_only()
                    .filter(sbp_users::Column::Id.eq::<u64>(press.user.id.into()))
                    .exists(db)
                    .await?;

                if is_sbp_user_exists {
                    press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Вы уже зарегистрированы в СБП")
                            .ephemeral(true)
                    )).await?;
                    return Ok(false);
                }

                let author_id = ctx.author().id.into();
                let invited_id = press.user.id.into();
                db.transaction::<_, (), DbErr>(|txn| {
                    Box::pin(async move {
                        let model = sbp_users::ActiveModel {
                            id: Set(invited_id),
                            ..Default::default()
                        };

                        sbp_users::Entity::insert(model)
                            .exec_without_returning(txn)
                            .await?;

                        let model = sbp_invites::ActiveModel {
                            user_id: Set(author_id),
                            invited_user_id: Set(invited_id),
                        };

                        sbp_invites::Entity::insert(model)
                            .exec_without_returning(txn)
                            .await?;

                        sbp_users::Entity::update_many()
                            .col_expr(
                                sbp_users::Column::Balance, 
                                Expr::col(sbp_users::Column::Balance).add(200)
                            )
                            .filter(sbp_users::Column::Id.eq(author_id))
                            .exec(txn)
                            .await?;

                        Ok(())
                    })
                }).await?;

                let embed = serenity::CreateEmbed::default()
                    .title("Приглашение принято")
                    .description(
                        format!(
                            "Вы зарегистрировались в СБП по ссылке от {}",
                            ctx.author().name
                        )
                    );

                press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::default()
                        .embed(embed)
                        .ephemeral(true)
                )).await?;

                let author_notifications: bool = sbp_users::Entity::find_by_id::<i64>(ctx.author().id.into())
                    .select_only()
                    .column(sbp_users::Column::Notifications)
                    .into_tuple()
                    .one(db)
                    .await?
                    .unwrap();

                if author_notifications {
                    ctx.author().dm(ctx, serenity::CreateMessage::default()
                        .content(format!(
                            "{} зарегистрировался в СБП по вашей ссылке!",
                            press.user.mention()
                        ))
                    ).await?;
                }

                return Ok(false);
            }
        }, move || {
            async move {
                let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("")
                        .label("Приглашение истекло")
                        .disabled(true)
                ])];
                msg.edit(ctx, CreateReply::default().components(buttons)).await?;
                return Ok(());
            }
        })
    .await?;

    Ok(())
}