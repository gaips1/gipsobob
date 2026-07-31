mod buttons;
mod checks;
mod helpers;
mod modules;
mod routes;
mod types;

use poise::{CreateReply, serenity_prelude as serenity};
use rand::seq::IndexedRandom as _;
use sqlx::postgres::PgPoolOptions;
use types::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: modules::all(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: None,
                mention_as_prefix: true,
                ..Default::default()
            },
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
                            .unwrap_or(String::from("10"))
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

                modules::dialogues::load("src/modules/traits/traits.json")?;

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

const NEW_MEMBER_GIFS: [&str; 6] = [
    "https://tenor.com/view/lucky-star-wave-hello-gif-3561948478976684501",
    "https://media.tenor.com/3o2hRDX8vw0AAAAC/hello-cute.gif",
    "https://media.tenor.com/J_JT8JsNDlUAAAAC/hello-anime.gif",
    "https://media.tenor.com/Q1dW7INg5ioAAAAC/hello-anime.gif",
    "https://media.tenor.com/mIteh_Sas9QAAAAd/anime-hello.gif",
    "https://tenor.com/view/ranma-anime-hello-chat-hi-hi-chat-gif-6124024137643951182",
];

const DELETED_MEMBER_GIFS: [&str; 5] = [
    "https://media.tenor.com/m0MabzE7tLIAAAAC/bye-anime-girl.gif",
    "https://media.tenor.com/lOMogKtB3E8AAAAC/goodbye-bye.gif",
    "https://media.tenor.com/4NHXeITTdKcAAAAC/anime-wave.gif",
    "https://media.tenor.com/oiYL8iyWwmkAAAAC/anime-jujutsu-kaisen.gif",
    "https://media.tenor.com/KasGopE0HIsAAAAC/bye-bye-anime.gif",
];

async fn on_event<'a>(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot: _ } => {
            tokio::spawn(modules::giveaways::restore_giveaways(
                ctx.clone(),
                data.pool.clone(),
            ));
            tokio::spawn(modules::giveaways::run_giveaway_poller(
                ctx.clone(),
                data.pool.clone(),
            ));
            tokio::spawn(modules::giveaways::run_daily_giveaway_scheduler(
                ctx.clone(),
                data.pool.clone(),
                843475272107163648,
            ));
            tokio::spawn(modules::quests::helpers::run_random_quests_adder(
                ctx.clone(),
                data.pool.clone(),
            ));
            tokio::spawn(modules::quests::helpers::run_expired_quests_poller(
                ctx.clone(),
                data.pool.clone(),
            ));
            tokio::spawn(modules::quests::helpers::run_old_quests_cleaner(
                data.pool.clone(),
            ));

            ctx.online();
            ctx.set_activity(Some(serenity::ActivityData::playing("Visual Studio Code")));
        }

        serenity::FullEvent::InteractionCreate { interaction } => {
            let serenity::Interaction::Component(component) = interaction else {
                return Ok(());
            };

            match &component.data.kind {
                serenity::ComponentInteractionDataKind::Button => {
                    routes::route_button_interaction(ctx, component, data).await?;
                }

                serenity::ComponentInteractionDataKind::StringSelect { values } => {
                    routes::route_string_select_interaction(ctx, component, data, values).await?;
                }

                _ => {}
            }
        }

        serenity::FullEvent::GuildMemberAddition { new_member } => {
            if new_member.guild_id != 621378615174758421 {
                return Ok(());
            }

            let random_gif = {
                let mut rng = rand::rng();
                *NEW_MEMBER_GIFS.choose(&mut rng).unwrap()
            };

            let embed = serenity::CreateEmbed::new()
                .title(format!(
                    "**{}, привет! Возможно, мы рады тебя видеть...**",
                    new_member.display_name()
                ))
                .image(random_gif);

            let _ = serenity::ChannelId::new(807651258520436736)
                .send_message(ctx, serenity::CreateMessage::new().embed(embed))
                .await;
        }

        serenity::FullEvent::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _,
        } => {
            if guild_id.get() != 621378615174758421 {
                return Ok(());
            }

            let random_gif = {
                let mut rng = rand::rng();
                *DELETED_MEMBER_GIFS.choose(&mut rng).unwrap()
            };

            let embed = serenity::CreateEmbed::new()
                .title(format!(
                    "**{}, надеемся ты к нам ещё придёшь**",
                    user.display_name()
                ))
                .image(random_gif);

            let _ = serenity::ChannelId::new(807651258520436736)
                .send_message(ctx, serenity::CreateMessage::new().embed(embed))
                .await;
        }

        serenity::FullEvent::Message { new_message: msg } => {
            if msg.author.bot {
                return Ok(());
            }

            if msg.author.id == ctx.cache.current_user().id {
                return Ok(());
            }

            if msg.channel_id.get()
                == if cfg!(debug_assertions) {
                    1217813620705067010
                } else {
                    1072943570962620477
                }
            {
                modules::counter::handle_counter_messages(ctx, msg).await?;
            }
        }

        _ => {}
    }

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    if let poise::FrameworkError::CommandCheckFailed { error: None, .. } = error {
        return;
    }
    if let poise::FrameworkError::UnknownCommand { .. } = error {
        return;
    }
    if let poise::FrameworkError::ArgumentParse { .. } = error {
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

    if let poise::FrameworkError::CommandPanic { payload, ctx, .. } = error {
        let mut msg = payload
            .as_deref()
            .unwrap_or_else(|| "Произошла ошибка при выполнении команды.");

        if msg.starts_with("!!") {
            msg = &msg[2..];
        } else {
            msg = "Произошла ошибка при выполнении команды.";
        }

        let _ = ctx
            .send(
                CreateReply::default()
                    .content(format!("{}", msg))
                    .ephemeral(true),
            )
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
