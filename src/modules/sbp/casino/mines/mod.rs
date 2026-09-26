use crate::{modules::traits::get_user_traits, types::*};
use dashmap::DashMap;
use poise::serenity_prelude::CacheHttp;
use rust_decimal::Decimal;
use std::sync::LazyLock;
use tokio::time::Instant;

mod helpers;
pub mod tasks;
mod types;

pub static CASINO_MINES_SECRET_KEY: LazyLock<String> = LazyLock::new(|| {
    std::env::var("CASINO_MINES_SECRET_KEY").expect("missing CASINO_MINES_SECRET_KEY")
});

pub static ACTIVE_GAMES: LazyLock<DashMap<serenity::UserId, types::ActiveSession>> =
    LazyLock::new(DashMap::new);

const RTP: Decimal = Decimal::from_parts(96, 0, 0, false, 2);

pub async fn handle_mines_buttons(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = interaction.data.custom_id.as_str();

    match custom_id {
        "casino:mines" => {
            create_game(ctx, interaction, data).await?;
        }

        id if id.starts_with("casino:mines:empty") => {
            interaction.defer(ctx.http()).await?;
        }

        _ => {
            handle_game(ctx, interaction, data).await?;
        }
    }

    Ok(())
}

pub async fn handle_game(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let user_id = interaction.user.id.get() as i64;

    let mut game =
        match types::MinesGame::from_custom_id(&interaction.data.custom_id, interaction.user.id) {
            Ok(game) => game,

            Err(types::MinesError::AlreadyOpened(_)) => {
                let _ = interaction.defer(&ctx.http).await;
                return Ok(());
            }

            Err(err) => {
                let _ = interaction
                    .reply(
                        ctx,
                        serenity::CreateInteractionResponseMessage::new()
                            .content(err.to_string())
                            .ephemeral(true),
                    )
                    .await;

                return Ok(());
            }
        };

    match game.action {
        types::MinesAction::Tile(tile) => {
            if game.is_bomb(tile) {
                ACTIVE_GAMES.remove(&game.user_id);

                sqlx::query("DELETE FROM active_games WHERE user_id = $1 AND game_type = 'mines'")
                    .bind(user_id)
                    .execute(&data.pool)
                    .await?;

                let embed = serenity::CreateEmbed::new()
                    .title("💥 БУУУМ! ВЫ ПОДОРВАЛИСЬ!")
                    .colour(serenity::Colour::from_rgb(231, 76, 60))
                    .description(format!(
                        "Вы наступили на мину на клетке **#{}**.\n\
                        Сгорело: **{}** бебр.\n\n\
                        *В следующий раз повезет больше!*",
                        tile + 1,
                        game.bet
                    ));

                interaction
                    .edit_reply(
                        ctx,
                        serenity::CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .components(Vec::new()),
                    )
                    .await?;
            } else {
                game.open_tile(tile);

                let total_safe = 16 - game.bombs as u32;

                if game.opened_count() == total_safe {
                    let win_amount = game.current_win();
                    ACTIVE_GAMES.remove(&game.user_id);

                    let mut tx = data.pool.begin().await?;

                    sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
                        .bind(win_amount)
                        .bind(user_id)
                        .execute(&mut *tx)
                        .await?;

                    sqlx::query(
                        "DELETE FROM active_games WHERE user_id = $1 AND game_type = 'mines'",
                    )
                    .bind(user_id)
                    .execute(&mut *tx)
                    .await?;

                    tx.commit().await?;

                    interaction
                        .edit_reply(
                            ctx,
                            serenity::CreateInteractionResponseMessage::new()
                                .embed(game.get_embed())
                                .components(Vec::new()),
                        )
                        .await?;
                } else {
                    if let Some(mut session) = ACTIVE_GAMES.get_mut(&game.user_id) {
                        session.current_win = game.current_win();
                        session.last_activity = Instant::now();
                    }

                    interaction
                        .edit_reply(
                            ctx,
                            serenity::CreateInteractionResponseMessage::new()
                                .content("")
                                .embed(game.get_embed())
                                .components(game.get_buttons()),
                        )
                        .await?;
                }
            }
        }

        types::MinesAction::Cashout => {
            let win_amount = game.current_win();
            let multiplier = game.current_multiplier();

            ACTIVE_GAMES.remove(&game.user_id);

            let mut tx = data.pool.begin().await?;

            sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
                .bind(win_amount)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;

            sqlx::query("DELETE FROM active_games WHERE user_id = $1 AND game_type = 'mines'")
                .bind(user_id)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            let embed = serenity::CreateEmbed::new()
                .title("💰 КЭШАУТ УСПЕШЕН!")
                .colour(serenity::Colour::from_rgb(46, 204, 113))
                .description(format!(
                    "Вы вовремя остановились и забрали банк!\n\n\
                    💵 Начальная ставка: `{}`\n\
                    📈 Итоговый множитель: `x{}`\n\
                    🎉 Зачислено на баланс: **`{}`** бебр!",
                    game.bet, multiplier, win_amount
                ));

            interaction
                .edit_reply(
                    ctx,
                    serenity::CreateInteractionResponseMessage::new()
                        .content("")
                        .embed(embed)
                        .components(Vec::new()),
                )
                .await?;
        }
    }

    Ok(())
}

pub async fn create_game(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let user_id = interaction.user.id.get() as i64;

    if ACTIVE_GAMES.contains_key(&interaction.user.id) {
        interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("У вас уже есть активная игра в Сапёра! Закончите её.")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    }

    let modal = serenity::CreateQuickModal::new("Сапёр")
        .timeout(std::time::Duration::from_secs(100))
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Ваша ставка", "")
                .max_length(20)
                .value("100"),
        )
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Количество мин (макс. 15)",
                "",
            )
            .max_length(2)
            .value("3"),
        );

    let response = interaction.quick_modal(ctx, modal).await?;
    let Some(response) = response else {
        return Ok(());
    };

    let stavka = response
        .inputs
        .first()
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u64>();

    let Ok(stavka) = stavka else {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Ваша ставка не является числом")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    let bombs = response
        .inputs
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u8>();

    let Ok(bombs) = bombs else {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Количество мин не является числом")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    if bombs < 1 || bombs > 15 {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Введите корректное количество мин (от 1 до 15)")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    }

    // 🟢 cheap_date: -5% к минимальной ставке в казино
    let user_traits = get_user_traits(&data.pool, interaction.user.id.get()).await?;
    let has_cheap_date = user_traits.contains(&"cheap_date".to_string());
    let min_guess_bet: u64 = if has_cheap_date { 95 } else { 100 };

    if stavka < min_guess_bet {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content(format!("Минимальная ставка {min_guess_bet} бебр"))
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    }

    let mut tx = data.pool.begin().await?;

    let result = sqlx::query(
        "UPDATE sbp_users \
        SET balance = balance - $2 \
        WHERE id = $1 AND balance >= $2",
    )
    .bind(user_id)
    .bind(stavka as i64)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("У вас не хватает бебр!")
                    .ephemeral(true),
            )
            .await?;

        return Ok(());
    }

    let insert_game = sqlx::query(
        "INSERT INTO active_games (user_id, game_type, bet) \
         VALUES ($1, 'mines', $2) \
         ON CONFLICT (user_id) DO NOTHING",
    )
    .bind(user_id)
    .bind(Decimal::from(stavka))
    .execute(&mut *tx)
    .await?;

    if insert_game.rows_affected() == 0 {
        tx.rollback().await?;

        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("У вас уже начата другая игра!")
                    .ephemeral(true),
            )
            .await?;

        return Ok(());
    }

    tx.commit().await?;

    ACTIVE_GAMES.insert(
        response.interaction.user.id,
        types::ActiveSession {
            current_win: stavka.into(),
            last_activity: Instant::now(),
        },
    );

    let game = types::MinesGame::new(response.interaction.user.id, stavka.into(), bombs);

    response
        .interaction
        .reply(
            ctx,
            serenity::CreateInteractionResponseMessage::new()
                .content("")
                .embed(game.get_embed())
                .components(game.get_buttons())
                .ephemeral(true),
        )
        .await?;

    Ok(())
}
