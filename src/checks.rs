use poise::CreateReply;
use sea_orm::{ColumnTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QuerySelect};
use crate::types::*;
use crate::database::{prelude::*, users, sbp};

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
            let new_user = users::ActiveModel {
                id: Set(user_id.into()),
                ..Default::default()
            };
            let _ = new_user.insert(db).await;
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

    let is_sbp_user_exists = Sbp::find()
        .select_only()
        .filter(sbp::Column::Id.eq::<u64>(user_id.into()))
        .exists(db)
        .await;

    match is_sbp_user_exists {
        Ok(is_exists) => {
            if !is_exists {
                let _ = ctx.send(
                    CreateReply::default()
                        .content("Вы не зарегистрированы в Системе Быстрых Платежей! Сделайте это, написав **/reg**.")
                        .ephemeral(true)
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