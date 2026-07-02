mod types;
mod modules;
// mod database;
mod checks;
mod buttons;

use sqlx::postgres::PgPoolOptions;
use types::*;
use poise::{CreateReply, serenity_prelude as serenity};

// глобальные обработчики кнопок
use modules::fun::kys::handle_kys_button;
use modules::sbp::register::sbp_register;
use modules::sbp::account::handle_change_notifications_button;
use modules::sbp::casino::{handle_guess_button, handle_slots_button};

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
                    .max_connections(15)
                    .connect(
                        std::env::var("DATABASE_URL")
                            .expect("env DATABASE_URL not set")
                            .as_str()
                        )
                    .await
                    .expect("Cannot connect to db");

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                ctx.online();
                ctx.set_activity(Some(serenity::ActivityData::playing("Visual Studio Code")));

                Ok(Data{ pool })
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

async fn on_event<'a>(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data
) -> Result<(), Error> {
    if let serenity::FullEvent::InteractionCreate { interaction } = event {
        if let serenity::Interaction::Component(component) = interaction {
            if matches!(component.data.kind, serenity::ComponentInteractionDataKind::Button) {
                match component.data.custom_id.as_str() {
                    "kys_btn" => {
                        handle_kys_button(ctx, component).await?
                    }

                    "sbp_register_btn" => {
                        sbp_register(ctx, component, data).await?
                    }

                    "sbp_notifications_change" => {
                        handle_change_notifications_button(ctx, component, data).await?
                    }

                    "casino:slots" => {
                        handle_slots_button(ctx, component, data).await?
                    }

                    "casino:guess" => {
                        handle_guess_button(ctx, component, data).await?
                    }

                    _ => { }
                }
            }
        }
    }
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    if let poise::FrameworkError::CommandCheckFailed { error: None, .. } = error {
        return;
    }

    if let poise::FrameworkError::CooldownHit { remaining_cooldown, ctx, .. } = error {
        let _ = ctx.send(CreateReply::default().content(format!(
            "Подожди ещё {:.1} секунд перед повторным использованием команды.",
            remaining_cooldown.as_secs_f32()
        )))
            .await;
        return;
    }

    log::error!("{:?}", error);
    if let Some(ctx) = error.ctx() {
        let _ = ctx.send(
            CreateReply::default()
                .content("Произошла ошибка при выполнении команды.")
                .ephemeral(true)
        ).await;
    }
}