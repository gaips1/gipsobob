mod types;
mod modules;
mod database;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QuerySelect};
use types::*;
use poise::serenity_prelude as serenity;
use database::{prelude::*, users};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: modules::all(),
            command_check: Some(|ctx| {
                Box::pin(async move {
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
                                let _ = ctx.say("Вы заблокированы в боте.").await;
                                return Ok(false);
                            }
                            
                            return Ok(true);
                        }
                        Ok(None) => {
                            let new_user = users::ActiveModel {
                                id: Set(user_id.into()),
                                ..Default::default()
                            };
                            let _ = new_user.insert(db).await;
                            return Ok(true)
                        }
                        Err(e) => {
                            let _ = ctx.say("Ошибка при обращении к бд.").await;
                            println!("{:#}", e);
                            return Ok(false);
                        }
                    }
                })
            }),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let db: sea_orm::DatabaseConnection = sea_orm::Database::connect(std::env::var("DATABASE_URL").expect("env DATABASE_URL not set"))
                    .await
                    .expect("Cannot connect to db");

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data{ db })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if std::env::var("BOT_SHARDED").as_deref() == Ok("true") {
        client.unwrap().start_autosharded().await.unwrap();
    } else {
        client.unwrap().start().await.unwrap();
    }
}