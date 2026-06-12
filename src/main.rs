mod types;
mod modules;
mod database;
mod checks;
mod buttons;

use types::*;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: modules::all(),
            command_check: Some(|ctx| Box::pin(checks::global_check(ctx))),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let db: sea_orm::DatabaseConnection = sea_orm::Database
                    ::connect(std::env::var("DATABASE_URL")
                    .expect("env DATABASE_URL not set"))
                    .await
                    .expect("Cannot connect to db");

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                ctx.online();
                ctx.set_activity(Some(serenity::ActivityData::playing("Visual Studio Code")));

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