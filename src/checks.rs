use poise::CreateReply;
use poise::serenity_prelude as serenity;
use crate::types::*;

pub async fn global_check(ctx: Context<'_>) -> Result<bool, Error> {
    let pool = &ctx.data().pool;
    let user_id = ctx.author().id;

    let user_is_banned = sqlx::query_scalar(
        "SELECT is_banned FROM users WHERE id = $1"
    ).bind::<i64>(user_id.into()).fetch_optional(pool).await;
    
    match user_is_banned {
        Ok(Some(is_banned)) => {
            if is_banned {
                let _ = ctx.send(
                    CreateReply::default()
                        .content("Вы заблокированы в боте.")
                        .ephemeral(true)
                ).await;
                return Ok(false);
            }
            
            Ok(true)
        }
        Ok(None) => {
            let _ = sqlx::query(
                "INSERT INTO users (id) VALUES ($1)"
            ).bind::<i64>(user_id.into()).execute(pool).await;
            Ok(true)
        }
        Err(e) => {
            let _ = ctx.send(
                CreateReply::default()
                    .content("Ошибка при обращении к бд.")
                    .ephemeral(true)
            ).await;
            log::error!("{:?}", e);
            Ok(false)
        }
    }
}

pub async fn sbp_check(ctx: Context<'_>) -> Result<bool, Error> {
    let pool = &ctx.data().pool;
    let user_id = ctx.author().id;

    let is_sbp_user_exists: Result<bool, sqlx::Error> = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM sbp_users WHERE id = $1)"
    ).bind::<i64>(user_id.into()).fetch_one(pool).await;

    match is_sbp_user_exists {
        Ok(is_exists) => {
            if !is_exists {
                let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("sbp_register_btn")
                            .label("Зарегистрироваться в СБП")
                            .style(serenity::ButtonStyle::Success)
                    ]
                )];

                let _ = ctx.send(
                    CreateReply::default()
                        .content("Вы не зарегистрированы в Системе Быстрых Платежей! Сделайте это, нажав кнопку ниже.")
                        .ephemeral(true)
                        .components(buttons)
                ).await;
                return Ok(false);
            }
        }
        Err(e) => {
            let _ = ctx.send(
                CreateReply::default()
                    .content("Ошибка при обращении к бд.")
                    .ephemeral(true)
            ).await;
            log::error!("{:#}", e);
            return Ok(false);
        }
    }

    Ok(true)
}