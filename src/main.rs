mod types;
mod modules;

use types::*;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: modules::all(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
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