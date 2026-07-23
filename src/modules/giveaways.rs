use std::time::Duration;

use rand::seq::IndexedRandom as _;
use tokio::time::{MissedTickBehavior, interval};

use crate::types::*;

pub async fn restore_giveaways(ctx: serenity::Context, pool: sqlx::PgPool) {
    let giveaways: Vec<(i64, chrono::DateTime<chrono::Utc>, i32, i64)> =
        sqlx::query_as("SELECT id, ends_at, prize, channel_id FROM giveaways")
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

    log::info!("restoring active giveaways ({}..)", giveaways.len());

    for g in giveaways {
        let ctx = ctx.clone();
        let pool = pool.clone();
        let (id, ends_at, prize, channel_id) = g;
        spawn_giveaway_timer(ctx, pool, id, ends_at, prize, channel_id).await;
    }
}

pub async fn run_giveaway_poller(ctx: serenity::Context, pool: sqlx::PgPool) {
    log::info!("giveaway poller started");

    tokio::time::sleep(Duration::from_mins(5)).await;

    let mut ticker = interval(Duration::from_mins(5));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;

        let giveaways: Vec<(i64, i32, i64)> = sqlx::query_as(
            "SELECT id, prize, channel_id FROM giveaways \
                WHERE ends_at <= NOW() LIMIT 20",
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        for g in giveaways {
            let ctx = ctx.clone();
            let pool = pool.clone();
            let (id, prize, channel_id) = g;

            if let Err(e) = process_giveaway(&ctx, &pool, id, prize, channel_id).await {
                log::error!("Failed to process giveaway {}: {:?}", id, e);
            }
        }
    }
}

pub async fn spawn_giveaway_timer(
    ctx: serenity::Context,
    pool: sqlx::PgPool,
    giveaway_id: i64,
    ends_at: chrono::DateTime<chrono::Utc>,
    prize: i32,
    channel_id: i64,
) {
    let now = chrono::Utc::now();
    if ends_at <= now {
        if let Err(e) = process_giveaway(&ctx, &pool, giveaway_id, prize, channel_id).await {
            log::error!("Failed to process giveaway {}: {:?}", giveaway_id, e);
        }
        return;
    }

    let duration = (ends_at - now)
        .to_std()
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));

    tokio::spawn(async move {
        tokio::time::sleep(duration).await;

        if let Err(e) = process_giveaway(&ctx, &pool, giveaway_id, prize, channel_id).await {
            log::error!("Failed to process giveaway {}: {:?}", giveaway_id, e);
        }
    });
}

async fn process_giveaway(
    ctx: &serenity::Context,
    pool: &sqlx::PgPool,
    id: i64,
    prize: i32,
    channel_id: i64,
) -> Result<(), Error> {
    let result = sqlx::query("DELETE FROM giveaways WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(());
    }

    let users: Vec<i64> = sqlx::query_scalar(
        "SELECT user_id \
        FROM giveaway_participants \
        WHERE giveaway_id = $1",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut msg = serenity::ChannelId::new(channel_id as u64)
        .message(ctx, id as u64)
        .await?;

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("meow")
            .label("Конкурс обкончен")
            .style(serenity::ButtonStyle::Success)
            .disabled(true),
    ])];

    let _ = msg
        .edit(ctx, serenity::EditMessage::new().components(buttons))
        .await;

    if users.len() == 0 {
        let embed = serenity::CreateEmbed::new()
            .title("К сожалению, никто не поучаствовал в розыгрыше :(")
            .colour(serenity::colours::branding::RED);

        let _ = msg
            .channel_id
            .send_message(
                ctx,
                serenity::CreateMessage::new()
                    .embed(embed)
                    .reference_message(&msg),
            )
            .await;

        return Ok(());
    }

    let winner_id = {
        let mut rng = rand::rng();
        *users.choose(&mut rng).unwrap()
    };

    let result = sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
        .bind(prize)
        .bind(winner_id)
        .execute(pool)
        .await?;

    let embed = serenity::CreateEmbed::new()
        .title("Ура! У нас есть победитель")
        .description(format!(
            "Поздравим <@{}> c победой, приз уже на его счету!\n{}",
            winner_id,
            if result.rows_affected() == 0
                {"||Шучу, он не был зарегистрирован в СБП, так что ничего не получит. Не повторяйте его ошибок - `/сбп регистрация`||"}
            else {""}
        ))
        .colour(serenity::colours::branding::GREEN);

    let _ = msg
        .channel_id
        .send_message(
            ctx,
            serenity::CreateMessage::new()
                .embed(embed)
                .reference_message(&msg),
        )
        .await;

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new_link(msg.link()).label("Розыгрыш"),
    ])];

    let _ = serenity::UserId::new(winner_id as u64)
        .dm(
            ctx,
            serenity::CreateMessage::new()
                .content("Поздравляю с победой в розыгрыше!")
                .components(buttons),
        )
        .await;

    Ok(())
}

pub async fn handle_giveaway_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = &press.data.custom_id;

    if custom_id == "giveaway:join" {
        match sqlx::query(
            "INSERT INTO giveaway_participants (giveaway_id, user_id) VALUES ($1, $2)",
        )
        .bind(press.message.id.get() as i64)
        .bind(press.user.id.get() as i64)
        .execute(&data.pool)
        .await
        {
            Ok(_) => {}

            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new(format!(
                        "giveaway:leave:{}",
                        press.message.id.get()
                    ))
                    .label("Не хочу участвовать")
                    .style(serenity::ButtonStyle::Danger),
                ])];

                crate::create_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Вы уже участвуете в розыгрыше!")
                        .components(buttons)
                        .ephemeral(true)
                );
                return Ok(());
            }

            Err(sqlx::Error::Database(db_err)) if db_err.is_foreign_key_violation() => {
                crate::create_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Такого розыгрыша не существует")
                        .ephemeral(true)
                );
                return Ok(());
            }

            Err(err) => {
                return Err(err.into());
            }
        }

        let buttons = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(format!("giveaway:leave:{}", press.message.id.get()))
                .label("Не хочу участвовать")
                .style(serenity::ButtonStyle::Danger),
        ])];

        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Успешно!")
                .components(buttons)
                .ephemeral(true)
        );
    } else if custom_id.starts_with("giveaway:leave:") {
        let giveaway_id: i64 = match custom_id.split(':').nth(2).and_then(|s| s.parse().ok()) {
            Some(id) => id,
            None => {
                return Ok(());
            }
        };

        let result = sqlx::query(
            "DELETE FROM giveaway_participants WHERE user_id = $1 AND giveaway_id = $2",
        )
        .bind(press.user.id.get() as i64)
        .bind(giveaway_id)
        .execute(&data.pool)
        .await?;

        if result.rows_affected() == 0 {
            crate::create_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Вы не участвуете в этом розыгрыше")
                    .ephemeral(true)
            );
            return Ok(());
        }

        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Успешно!")
                .ephemeral(true)
        );
        return Ok(());
    }

    Ok(())
}

/// Создать розыгрыш
#[poise::command(
    slash_command,
    rename = "розыгрыш",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn create_giveaway(
    ctx: Context<'_>,
    #[description = "Сколько бебр розыгрыш"] amount: u32,
    #[description = "Описание розыгрыша"] description: String,
    #[description = "Когда заканчивается"] ends_at_timestamp: u64,
) -> Result<(), Error> {
    if ctx.author().id.get() != 449882524697493515 {
        ctx.say("Недостаточно прав").await?;
        return Ok(());
    }

    let embed = serenity::CreateEmbed::new()
        .title(format!("Розыгрыш {amount} бебр"))
        .description(format!(
            "**{}\nЧтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{}:R>**",
            description, ends_at_timestamp
        ))
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("giveaway:join")
            .label("Принять участие")
            .style(serenity::ButtonStyle::Success),
    ])];

    let msg = ctx
        .channel_id()
        .send_message(
            ctx,
            serenity::CreateMessage::new()
                .embed(embed)
                .components(buttons),
        )
        .await?;

    let ends_at: chrono::DateTime<chrono::Utc> =
        chrono::DateTime::from_timestamp(ends_at_timestamp as i64, 0)
            .ok_or_else(|| Error::from("invalid timestamp"))?;

    sqlx::query("INSERT INTO giveaways (ends_at, prize, channel_id, id) VALUES ($1, $2, $3, $4)")
        .bind(ends_at)
        .bind(amount as i32)
        .bind(ctx.channel_id().get() as i64)
        .bind(msg.id.get() as i64)
        .execute(&ctx.data().pool)
        .await?;

    let pool = ctx.data().pool.clone();
    spawn_giveaway_timer(
        ctx.serenity_context().clone(),
        pool,
        msg.id.get() as i64,
        ends_at,
        amount as i32,
        ctx.channel_id().get() as i64,
    )
    .await;

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![create_giveaway()]
}
