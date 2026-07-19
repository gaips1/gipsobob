mod buttons;
mod checks;
mod helpers;
mod modules;
mod routes;
mod types;

use poise::{CreateReply, serenity_prelude as serenity};
use sqlx::postgres::PgPoolOptions;
use types::*;

// ДОБАВИТЬ ЕВЕНТЫ ПРИ ВХОДЕ НА СЕРВЕР

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: modules::all(),
            on_error: |err| Box::pin(on_error(err)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(on_event(ctx, event, framework, data))
            },
            command_check: Some(|ctx| Box::pin(checks::global_check(ctx))),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let pool = PgPoolOptions::new()
                    .max_connections(
                        std::env::var("DATABASE_MAX_CONNECTIONS")
                            .unwrap_or("5".to_owned())
                            .as_str()
                            .parse()
                            .expect("DATABASE_MAX_CONNECTIONS must be integer"),
                    )
                    .min_connections(2)
                    .connect(
                        std::env::var("DATABASE_URL")
                            .expect("env DATABASE_URL not set")
                            .as_str(),
                    )
                    .await
                    .expect("Cannot connect to db");

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                ctx.online();
                ctx.set_activity(Some(serenity::ActivityData::playing("Visual Studio Code")));

                Ok(Data { pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    if std::env::var("BOT_SHARDED").as_deref() == Ok("true") {
        log::info!("start sharded");
        client.unwrap().start_autosharded().await.unwrap();
    } else {
        log::info!("start nonsharded");
        client.unwrap().start().await.unwrap();
    }
}

async fn on_event<'a>(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> Result<(), Error> {
    let serenity::FullEvent::InteractionCreate { interaction } = event else {
        return Ok(());
    };

    let serenity::Interaction::Component(component) = interaction else {
        return Ok(());
    };

    if !matches!(
        component.data.kind,
        serenity::ComponentInteractionDataKind::Button
    ) {
        return Ok(());
    }

    routes::route_button_interaction(ctx, component, data).await?;

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    if let poise::FrameworkError::CommandCheckFailed { error: None, .. } = error {
        return;
    }

    if let poise::FrameworkError::CooldownHit {
        remaining_cooldown,
        ctx,
        ..
    } = error
    {
        let _ = ctx
            .send(CreateReply::default().content(format!(
                "Подожди ещё {:.1} секунд перед повторным использованием команды.",
                remaining_cooldown.as_secs_f32()
            )))
            .await;
        return;
    }

    log::error!("{:?}", error);
    if let Some(ctx) = error.ctx() {
        let _ = ctx
            .send(
                CreateReply::default()
                    .content("Произошла ошибка при выполнении команды.")
                    .ephemeral(true),
            )
            .await;
    }
}
