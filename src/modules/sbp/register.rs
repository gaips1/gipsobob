use crate::types::*;

pub async fn sbp_register(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let sbp_user = sqlx::query("INSERT INTO sbp_users (id) VALUES ($1)")
        .bind::<i64>(interaction.user.id.into())
        .execute(&data.pool)
        .await;

    match sbp_user {
        Ok(_) => {
            crate::create_edit_response!(
                ctx,
                interaction,
                serenity::CreateInteractionResponseMessage::default()
                    .content("Вы успешно зарегистрированы! Посмотрите свой баланс используя команду `/account`!")
                    .ephemeral(true)
                    .components(Vec::new())
            );
        }

        Err(err) => {
            if let Some(db_err) = err.as_database_error() {
                if db_err.is_unique_violation() {
                    crate::create_edit_response!(
                        ctx,
                        interaction,
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Вы уже зарегистрированы в СБП")
                            .ephemeral(true)
                            .components(Vec::new())
                    );
                    return Ok(());
                }
            }
            crate::create_edit_response!(
                ctx,
                interaction,
                serenity::CreateInteractionResponseMessage::default()
                    .content("Произошла неизвестная ошибка при регистрации")
                    .ephemeral(true)
                    .components(Vec::new())
            );
        }
    }

    Ok(())
}

/// Регистрация в СБП
#[poise::command(
    slash_command,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn reg(ctx: Context<'_>) -> Result<(), Error> {
    let sbp_user = sqlx::query("INSERT INTO sbp_users (id) VALUES ($1)")
        .bind::<i64>(ctx.author().id.into())
        .execute(&ctx.data().pool)
        .await;

    match sbp_user {
        Ok(_) => {
            ctx.say(
                "Вы успешно зарегистрированы! Посмотрите свой баланс используя команду `/account`!",
            )
            .await?;
        }

        Err(err) => {
            if let Some(db_err) = err.as_database_error() {
                if db_err.is_unique_violation() {
                    ctx.say("Вы уже зарегистрированы в СБП").await?;
                    return Ok(());
                }
            }
            ctx.say("Произошла неизвестная ошибка при регистрации")
                .await?;
        }
    }

    Ok(())
}
