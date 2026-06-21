use crate::{database::sbp_users, types::*};
use poise::serenity_prelude as serenity;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};

pub async fn sbp_register(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data
) -> Result<(), Error> {
    let sbp_user = sbp_users::ActiveModel {
        id: Set(interaction.user.id.into()),
        ..Default::default()
    }.insert(&data.db).await;

    match sbp_user {
        Ok(_) => {
            interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                serenity::CreateInteractionResponseMessage::default()
                    .content("Вы успешно зарегистрированы! Посмотрите свой баланс используя команду **/account**!")
                    .ephemeral(true)
                    .components(vec![])
            )).await?;
        }

        Err(err) => {
            if let DbErr::RecordNotInserted = err {
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