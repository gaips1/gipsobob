use crate::{database::sbp_users, types::*};
use poise::serenity_prelude as serenity;
use sea_orm::{ActiveValue::Set, EntityTrait, SqlErr};

pub async fn sbp_register(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data
) -> Result<(), Error> {
    let model = sbp_users::ActiveModel {
        id: Set(interaction.user.id.into()),
        ..Default::default()
    };

    let sbp_user = sbp_users::Entity::insert(model)
        .exec_without_returning(&data.db)
        .await;

    match sbp_user {
        Ok(_) => {
            interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::default()
                    .content("Вы успешно зарегистрированы! Посмотрите свой баланс используя команду `/account`!")
                    .ephemeral(true)
                    .components(vec![])
            )).await?;
        }

        Err(err) => {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы уже зарегистрированы в СБП")
                        .ephemeral(true)
                        .components(vec![])
                )).await?;
            } else {
                interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Произошла неизвестная ошибка при регистрации")
                        .ephemeral(true)
                        .components(vec![])
                )).await?;
            }
        }
    }

    Ok(())
}

/// Регистрация в СБП
#[poise::command(slash_command, ephemeral, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn reg(ctx: Context<'_>) -> Result<(), Error> {
    let model = sbp_users::ActiveModel {
        id: Set(ctx.author().id.into()),
        ..Default::default()
    };

    let sbp_user = sbp_users::Entity::insert(model)
        .exec_without_returning(&ctx.data().db)
        .await;

    match sbp_user {
        Ok(_) => {
            ctx.say("Вы успешно зарегистрированы! Посмотрите свой баланс используя команду `/account`!").await?;
        }

        Err(err) => {
            if let Some(SqlErr::UniqueConstraintViolation(_)) = err.sql_err() {
                ctx.say("Вы уже зарегистрированы в СБП").await?;
            } else {
                log::error!("{:?}", err);
                ctx.say("Произошла неизвестная ошибка при регистрации").await?;
            }
        }
    }

    Ok(())
}