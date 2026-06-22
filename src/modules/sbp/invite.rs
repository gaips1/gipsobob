use poise::serenity_prelude::Mentionable;
use poise::{serenity_prelude as serenity};
use poise::{CreateReply};

use crate::buttons::handle_button;
use crate::checks::sbp_check;
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

    let msg = ctx.send(CreateReply::default().embed(embed.clone()).components(buttons)).await?;

    handle_button(ctx, &ctx.id().to_string(), 3600,
        move |press| {
            async move {
                let pool = &ctx.data().pool;

                let is_sbp_user_exists: bool = sqlx::query_scalar(
                    "SELECT EXISTS (SELECT 1 FROM users WHERE id = $1)"
                ).bind::<i64>(press.user.id.into()).fetch_one(pool).await?;

                if is_sbp_user_exists {
                    press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Вы уже зарегистрированы в СБП")
                            .ephemeral(true)
                    )).await?;
                    return Ok(false);
                }

                let mut tx = pool.begin().await?;

                sqlx::query(
                    "INSERT INTO sbp_users (id) VALUES ($1)"
                )
                    .bind::<i64>(press.user.id.into())
                    .execute(&mut *tx)
                    .await?;
                
                sqlx::query(
                    "INSERT INTO sbp_invites (user_id, invited_user_id) VALUES ($1, $2)"
                )
                    .bind::<i64>(ctx.author().id.into())
                    .bind::<i64>(press.user.id.into())
                    .execute(&mut *tx)
                    .await?;

                sqlx::query(
                    "UPDATE sbp_users SET balance = balance + $1 WHERE id = $2"
                )
                    .bind(200)
                    .bind::<i64>(ctx.author().id.into())
                    .execute(&mut *tx)
                    .await?;

                tx.commit().await?;

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

                let author_notifications: bool = sqlx::query_scalar(
                    "SELECT notifications FROM sbp_users WHERE id = $1"
                ).bind::<i64>(ctx.author().id.into()).fetch_one(pool).await?;

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
                    serenity::CreateButton::new("disabled")
                        .label("Приглашение истекло")
                        .disabled(true)
                ])];
                msg.edit(ctx, CreateReply::default().components(buttons).embed(embed)).await?;
                return Ok(());
            }
        })
    .await?;

    Ok(())
}