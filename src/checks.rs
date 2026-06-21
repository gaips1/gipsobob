use poise::CreateReply;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QuerySelect};
use poise::serenity_prelude as serenity;
use crate::types::*;
use crate::database::{prelude::*, users, sbp_users};

pub async fn global_check(ctx: Context<'_>) -> Result<bool, Error> {
    let db = &ctx.data().db;
    let user_id = ctx.author().id;

    let user_is_banned = Users::find_by_id(user_id)
        .select_only()
        .column(users::Column::IsBanned)
        .into_tuple::<bool>()
        .one(db)
        .await;
    
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
            let _ = users::ActiveModel {
                id: Set(user_id.into()),
                ..Default::default()
            }.insert(db).await;
            Ok(true)
        }
        Err(e) => {
            let _ = ctx.send(
                CreateReply::default()
                    .content("Ошибка при обращении к бд.")
                    .ephemeral(true)
            ).await;
            log::error!("{:#}", e);
            Ok(false)
        }
    }
}

pub async fn sbp_check(ctx: Context<'_>) -> Result<bool, Error> {
    let db = &ctx.data().db;
    let user_id = ctx.author().id;

    let is_sbp_user_exists = SbpUsers::find()
        .select_only()
        .filter(sbp_users::Column::Id.eq::<u64>(user_id.into()))
        .exists(db)
        .await;

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