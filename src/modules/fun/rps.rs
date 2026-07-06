use std::collections::HashMap;
use std::ops::Mul as _;

use crate::buttons::{handle_button, handle_buttons};
use crate::types::*;
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use rust_decimal::Decimal;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RpsChoice {
    Rock,
    Scissors,
    Paper,
}

impl RpsChoice {
    fn beats(self, other: RpsChoice) -> bool {
        matches!(
            (self, other),
            (RpsChoice::Rock, RpsChoice::Scissors)
                | (RpsChoice::Scissors, RpsChoice::Paper)
                | (RpsChoice::Paper, RpsChoice::Rock)
        )
    }
}

/// Камень-ножницы-бумага
#[poise::command(
    slash_command,
    rename = "цуефа",
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn rps(
    ctx: Context<'_>,
    #[description = "С кем играть"] user: serenity::User,
    #[description = "Ставка в бебрах"] amount: Option<u64>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.bot {
        ctx.say("Бро").await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.say("Бро").await?;
        return Ok(());
    }

    let embed;
    if let Some(amount) = amount {
        let amount = match Decimal::try_from(amount) {
            Ok(d) => d,
            Err(_) => {
                ctx.say("Указана некорректная сумма").await?;
                return Ok(());
            }
        };

        if amount.is_zero() || amount.is_sign_negative() {
            ctx.say("Пожалуйста, введите положительное или не нулевое число бебр")
                .await?;
            return Ok(());
        }

        let result = sqlx::query(
            "UPDATE sbp_users SET balance = balance - $1 WHERE id = $2 AND balance >= $1",
        )
        .bind(amount)
        .bind::<i64>(ctx.author().id.into())
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            ctx.say("У Вас не хватает бебр на ставку, либо вы не зарегистрированы в СБП. Сделайте это, используя команду `/reg`").await?;
            return Ok(());
        }

        let is_user_exists: bool =
            sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM sbp_users WHERE id = $1)")
                .bind::<i64>(user.id.into())
                .fetch_one(pool)
                .await?;

        if !is_user_exists {
            ctx.say(
                "Пользователь, которому вы предлагаете играть со ставкой, не зарегистрирован в СБП",
            )
            .await?;
            return Ok(());
        }

        embed = serenity::CreateEmbed::default()
            .title("Цуефа")
            .description(
                format!("**{}** предложил **{}** поиграть в цуефа!\nСтавка {amount} бебр\nУ него 1 минута на ответ!", ctx.author().display_name(), user.display_name())
            );
    } else {
        embed = serenity::CreateEmbed::default()
            .title("Цуефа")
            .description(format!(
                "**{}** предложил **{}** поиграть в цуефа!\nБез ставки!\nУ него 1 минута на ответ!",
                ctx.author().display_name(),
                user.display_name()
            ));
    }

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(format!("{}:rps:yes", ctx.id()))
            .label("Согласен играть")
            .style(serenity::ButtonStyle::Success),
    ])];

    let msg = ctx
        .send(
            CreateReply::default()
                .components(buttons)
                .embed(embed)
                .ephemeral(false),
        )
        .await?;

    let accepted_user = user.clone();
    let accepted_msg = msg.clone();
    let choiced_msg = msg.clone();

    let accepted = handle_button(
        ctx,
        format!("{}:rps:yes", ctx.id()).as_str(),
        60,
        move |press| {
            let user = accepted_user.clone();
            async move {
                if press.user.id != user.id {
                    press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Тише будь")
                            .ephemeral(true)
                    )).await?;
                    return Ok(false);
                }

                if let Some(amount) = amount {
                    let amount = Decimal::try_from(amount).unwrap();
                    let result = sqlx::query(
                        "UPDATE sbp_users SET balance = balance - $1 WHERE id = $2 AND balance >= $1"
                    )
                        .bind(amount)
                        .bind::<i64>(user.id.into())
                        .execute(pool)
                        .await?;

                    if result.rows_affected() == 0 {
                        press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                            serenity::CreateInteractionResponseMessage::default()
                                .content("У Вас не хватает бебр на ставку, либо вы не зарегистрированы в СБП. Сделайте это, используя команду `/reg`")
                                .ephemeral(true)
                        )).await?;
                        return Ok(false);
                    }
                }

                let embed = serenity::CreateEmbed::new()
                    .title(format!("Цуефа {} VS {}", ctx.author().display_name(), user.display_name()))
                    .description("Игроки думают...\n||У них 3 минуты на то чтоб думоц||")
                    .colour(serenity::colours::branding::YELLOW);

                let choice_buttons = vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new(format!("{}:rps:choice:rock", ctx.id())).label("Камень"),
                    serenity::CreateButton::new(format!("{}:rps:choice:scissors", ctx.id())).label("Ножницы"),
                    serenity::CreateButton::new(format!("{}:rps:choice:paper", ctx.id())).label("Бумага"),
                ])];

                press.create_response(
                    &ctx,
                    serenity::CreateInteractionResponse::UpdateMessage(
                        serenity::CreateInteractionResponseMessage::default()
                            .components(choice_buttons)
                            .embed(embed)
                    )
                ).await?;

                Ok(true)
            }
        },
        move || {
            async move {
                if let Some(amount) = amount {
                    let amount = Decimal::try_from(amount).unwrap();
                    let (_, _) = tokio::join!(
                        sqlx::query(
                            "UPDATE sbp_users SET balance = balance + $1 WHERE id = $2"
                        )
                            .bind(amount)
                            .bind::<i64>(ctx.author().id.into())
                            .execute(pool),

                        accepted_msg.edit(ctx, CreateReply::default().components(vec![]).content("Время ожидания истекло. Игра отменена."))
                    );
                } else {
                    accepted_msg.edit(ctx, CreateReply::default().components(vec![]).content("Время ожидания истекло. Игра отменена.")).await?;
                }
                Ok(())
            }
        },
    ).await?;

    if !accepted {
        return Ok(());
    }

    let choices: Arc<Mutex<HashMap<u64, RpsChoice>>> = Arc::new(Mutex::new(HashMap::new()));
    let button_choices = choices.clone();

    let choiced = handle_buttons(
        ctx,
        format!("{}:rps:choice:", ctx.id()).as_str(),
        180,
        move |press, btn_id| {
            let choices = button_choices.clone();
            async move {
                if press.user.id != user.id && press.user.id != ctx.author().id {
                    press
                        .create_response(
                            &ctx,
                            serenity::CreateInteractionResponse::Message(
                                serenity::CreateInteractionResponseMessage::default()
                                    .content("Тише будь")
                                    .ephemeral(true),
                            ),
                        )
                        .await?;
                    return Ok(false);
                }

                let choice = match btn_id.as_str() {
                    "rock" => RpsChoice::Rock,
                    "scissors" => RpsChoice::Scissors,
                    "paper" => RpsChoice::Paper,
                    _ => return Ok(false),
                };

                let mut choices = choices.lock().await;

                if choices.contains_key(&press.user.id.get()) {
                    press
                        .create_response(
                            &ctx,
                            serenity::CreateInteractionResponse::Message(
                                serenity::CreateInteractionResponseMessage::default()
                                    .content("Ты уже выбрал")
                                    .ephemeral(true),
                            ),
                        )
                        .await?;
                    return Ok(false);
                }

                choices.insert(press.user.id.get(), choice);

                press
                    .create_response(
                        &ctx,
                        serenity::CreateInteractionResponse::Message(
                            serenity::CreateInteractionResponseMessage::default()
                                .content(format!(
                                    "Успешно выбрал {}",
                                    if let RpsChoice::Paper = choice {
                                        "бумагу"
                                    } else if let RpsChoice::Rock = choice {
                                        "камень"
                                    } else {
                                        "ножницы"
                                    }
                                ))
                                .ephemeral(true),
                        ),
                    )
                    .await?;

                Ok(choices.len() == 2)
            }
        },
        move || async move {
            if let Some(amount) = amount {
                let amount = Decimal::try_from(amount).unwrap();
                let (_, _) = tokio::join!(
                    sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = ANY($2)")
                        .bind(amount)
                        .bind([ctx.author().id.get() as i64, user.id.get() as i64])
                        .execute(pool),
                    choiced_msg.edit(
                        ctx,
                        CreateReply::default()
                            .components(vec![])
                            .content("Время ожидания истекло. Игра отменена.")
                    )
                );
            } else {
                choiced_msg
                    .edit(
                        ctx,
                        CreateReply::default()
                            .components(vec![])
                            .content("Время ожидания истекло. Игра отменена."),
                    )
                    .await?;
            }
            Ok(())
        },
    )
    .await?;

    if !choiced {
        return Ok(());
    }

    let choices = choices.lock().await;

    let author_choice = *choices.get(&ctx.author().id.get()).unwrap();
    let user_choice = *choices.get(&user.id.get()).unwrap();

    let mut embed = serenity::CreateEmbed::new().title(format!(
        "Цуефа {} VS {}",
        ctx.author().display_name(),
        user.display_name()
    ));

    if author_choice == user_choice {
        if let Some(amount) = amount {
            let amount = Decimal::try_from(amount).unwrap();
            sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = ANY($2)")
                .bind(amount)
                .bind([ctx.author().id.get() as i64, user.id.get() as i64])
                .execute(pool)
                .await?;
            embed = embed
                .description("Ничья!!! Бебры за ставку возвращены")
                .colour(serenity::colours::branding::YELLOW)
        } else {
            embed = embed
                .description("Ничья!!!")
                .colour(serenity::colours::branding::YELLOW)
        }
    } else if author_choice.beats(user_choice) {
        if let Some(amount) = amount {
            let amount = Decimal::try_from(amount).unwrap();
            sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
                .bind(amount.mul(Decimal::TWO))
                .bind(ctx.author().id.get() as i64)
                .execute(pool)
                .await?;
        }

        embed = embed
            .description(format!("**{}** победил!!!", ctx.author().display_name()))
            .colour(serenity::colours::branding::GREEN)
    } else {
        if let Some(amount) = amount {
            let amount = Decimal::try_from(amount).unwrap();
            sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
                .bind(amount.mul(Decimal::TWO))
                .bind(user.id.get() as i64)
                .execute(pool)
                .await?;
        }

        embed = embed
            .description(format!("**{}** победил!!!", user.display_name()))
            .colour(serenity::colours::branding::RED)
    }

    msg.edit(ctx, CreateReply::default().embed(embed).components(vec![]))
        .await?;

    Ok(())
}
