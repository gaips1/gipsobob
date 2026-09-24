use std::ops::Mul;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use crate::{modules::traits::get_user_traits, types::*};

pub async fn handle_guess_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let modal = serenity::CreateQuickModal::new("Угадай число")
        .timeout(std::time::Duration::from_secs(300))
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Ваша ставка", "")
                .max_length(20)
                .value("100"),
        )
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "До какого числа будете угадывать? (включит.)",
                "",
            )
            .max_length(20)
            .value("10"),
        )
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Ваше число", "")
                .max_length(20),
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
    let stavka: Decimal = stavka.into();

    let range = response
        .inputs
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u64>();
    let Ok(range) = range else {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Ваше число не является числом")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    let number = response
        .inputs
        .get(2)
        .map(|s| s.as_str())
        .unwrap_or("")
        .parse::<u64>();
    let Ok(number) = number else {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Ваше число не является числом")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    if number < 1 || number > range {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Ваше число не входит в указанный диапазон!")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    }

    let mut rand_num = rand::random_range(1..=range);

    // 🟢 gambler: +3% шанс, что казино "промахнётся" и загаданное число совпадёт с вашим
    if rand_num != number && user_traits.contains(&"gambler".to_string()) && rand::random_bool(0.03)
    {
        rand_num = number;
    }

    let win: Option<Decimal> = if rand_num == number {
        Some(
            stavka
                .mul(Decimal::from_u64(range).unwrap())
                .mul(Decimal::from_f64(0.2).unwrap()),
        )
    } else {
        None
    };

    let delta = match win {
        Some(w) => w - stavka,
        None => -stavka,
    };

    let result = sqlx::query(
        "UPDATE sbp_users
         SET balance = balance + $1
         WHERE id = $2 AND balance >= $3",
    )
    .bind(delta)
    .bind::<i64>(interaction.user.id.into())
    .bind(stavka)
    .execute(&data.pool)
    .await?;

    if result.rows_affected() == 0 {
        response
            .interaction
            .reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content("У вас не хватает бебр")
                    .ephemeral(true),
            )
            .await?;
        return Ok(());
    };

    let _ = add_user_quest_progress(
        &data.pool,
        ctx,
        interaction.user.id.get(),
        "casino",
        None,
        None,
    )
    .await;

    match win {
        Some(w) => {
            response
                .interaction
                .reply(
                    ctx,
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(
                            serenity::CreateEmbed::default()
                                .title(format!("Вы выиграли!"))
                                .description(format!(
                                    "Ваша ставка: {stavka} бебр\nВыигрыш: {w} бебр"
                                ))
                                .colour(serenity::colours::branding::GREEN),
                        )
                        .ephemeral(true),
                )
                .await?;
        }

        None => {
            response
                .interaction
                .reply(
                    ctx,
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(
                            serenity::CreateEmbed::default()
                                .title(format!("Вы проиграли!"))
                                .description(format!(
                                    "Я выдумал число {}\nВы могли бы выиграть {} бебр!",
                                    rand_num,
                                    stavka
                                        .mul(Decimal::from_u64(range).unwrap())
                                        .mul(Decimal::from_f64(0.2).unwrap())
                                ))
                                .colour(serenity::colours::branding::FUCHSIA),
                        )
                        .ephemeral(true),
                )
                .await?;
        }
    }

    Ok(())
}