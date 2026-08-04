pub mod kys;
mod rps;
mod sex;

use crate::modules::traits::get_user_traits;
use crate::types::*;
use poise::CreateReply;
use rand::prelude::*;
use tokio::time::{Duration, sleep};

/// Да или нет
#[poise::command(
    slash_command,
    rename = "данет",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn yes_or_no(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(if rand::random_bool(0.5) {
        "Да"
    } else {
        "Нет"
    })
    .await?;
    Ok(())
}

/// Подбросить монетку
#[poise::command(
    slash_command,
    rename = "монетка",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn monetka(ctx: Context<'_>) -> Result<(), Error> {
    // 🔵 lucky_coin: шанс выпадения ребра увеличен
    let author_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
    let has_lucky_coin = author_traits.contains(&"lucky_coin".to_string());
    let edge_weight: u32 = if has_lucky_coin { 20 } else { 10 };

    let choice = {
        let mut rng = rand::rng();
        [("Орёл!", 45), ("Решка!", 45), ("Ребро!", edge_weight)]
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
    };
    let msg = ctx.say("Подбрасываю...").await?;
    sleep(Duration::from_millis(2_500)).await;
    if choice == "Ребро!" {
        let _ = add_user_quest_progress(
            &ctx.data().pool,
            ctx.serenity_context(),
            ctx.author().id.get(),
            "monetka",
            None,
            None,
        )
        .await;
    }
    msg.edit(ctx, CreateReply::default().content(choice))
        .await?;
    Ok(())
}

/// Русская рулетка
#[poise::command(
    slash_command,
    rename = "русская-рулетка",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn russian_roulette(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.say("Вставляю пулю...").await?;
    sleep(Duration::from_millis(1_500)).await;
    msg.edit(ctx, CreateReply::default().content("Раскручиваю барабан.."))
        .await?;
    sleep(Duration::from_millis(1_500)).await;

    let mut death_chance: f64 = 0.167;
    // 🔵 bulletproof_skull: 10% шанс выжить при выстреле в русской рулетке
    let author_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
    if author_traits.contains(&"bulletproof_skull".to_string()) {
        death_chance = (death_chance - 0.1).max(0.0);
    }

    let is_dead = rand::random_bool(death_chance);
    if is_dead {
        let _ = add_user_quest_progress(
            &ctx.data().pool,
            ctx.serenity_context(),
            ctx.author().id.get(),
            "rr",
            None,
            None,
        )
        .await;
    }
    msg.edit(
        ctx,
        CreateReply::default().content(if is_dead {
            "Бум! Тебе разорвало лицо"
        } else {
            "Повезло, ты остался жив"
        }),
    )
    .await?;
    Ok(())
}

/// Кинуть кости
#[poise::command(
    slash_command,
    rename = "кости",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn kosti(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.say("Кидаю...").await?;
    sleep(Duration::from_millis(2_500)).await;

    // 🟡 loaded_dice: минимальное выпадение гарантированно не ниже 3
    let author_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
    let has_loaded_dice = author_traits.contains(&"loaded_dice".to_string());
    let roll = if has_loaded_dice {
        rand::random_range(3..=6)
    } else {
        rand::random_range(1..=6)
    };

    msg.edit(
        ctx,
        CreateReply::default().content(format!("Выпало число {}", roll)),
    )
    .await?;
    Ok(())
}

/// Слава узбии!
#[poise::command(
    slash_command,
    rename = "слава",
    subcommands("uzbii"),
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn slava(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Слава узбии!
#[poise::command(
    slash_command,
    rename = "узбии",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn uzbii(ctx: Context<'_>) -> Result<(), Error> {
    let _ = add_user_quest_progress(
        &ctx.data().pool,
        ctx.serenity_context(),
        ctx.author().id.get(),
        "slava_uzbii",
        None,
        None,
    )
    .await;

    // 🟢 uzbii_fan: 3% шанс получить 10 бебр за прославление Узбии
    let mut description = String::new();
    let author_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
    if author_traits.contains(&"uzbii_fan".to_string())
        && rand::random_bool(0.03)
    {
        let result = sqlx::query("UPDATE sbp_users SET balance = balance + 10 WHERE id = $1")
            .bind::<i64>(ctx.author().id.into())
            .execute(&ctx.data().pool)
            .await?;

        if result.rows_affected() > 0 {
            description = "Узбия услышала и кинула 10 бебр на карман!".to_string();
        }
    }

    ctx.send(
        CreateReply::default().embed(
            serenity::CreateEmbed::default()
                .title("Слава узбии!")
                .description(description)
                .color(serenity::colours::branding::RED),
        ),
    )
    .await?;
    Ok(())
}

const KISS_GIFS: [&str; 5] = [
    "https://media.tenor.com/jnndDmOm5wMAAAAC/kiss.gif",
    "https://media.tenor.com/fiafXWajQFoAAAAC/kiss-anime.gif",
    "https://media.tenor.com/dn_KuOESmUYAAAAC/engage-kiss-anime-kiss.gif",
    "https://media.tenor.com/9jB6M6aoW0AAAAAM/val-ally-kiss.gif",
    "https://media.tenor.com/SYwRyd6N1UIAAAAC/anime-kiss.gif",
];
/// Поцеловать пользователя
#[poise::command(
    slash_command,
    rename = "поцеловать",
    context_menu_command = "Поцеловать",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn kiss(
    ctx: Context<'_>,
    #[description = "Кого целуете"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.send(
            CreateReply::default()
                .content("Ты чё целовать себя собираешься?")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let _ = add_user_quest_progress(
        &ctx.data().pool,
        ctx.serenity_context(),
        ctx.author().id.get(),
        "kiss",
        Some(user.id.get()),
        None,
    )
    .await;

    let gif = {
        let mut rng = rand::rng();
        *KISS_GIFS.choose(&mut rng).unwrap()
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} поцеловал(а) {}",
            ctx.author()
                .global_name
                .as_deref()
                .unwrap_or_else(|| &ctx.author().name),
            user.global_name.as_deref().unwrap_or_else(|| &user.name),
        ))
        .image(gif)
        .colour(serenity::colours::branding::GREEN);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

const HUG_GIFS: [&str; 6] = [
    "https://media.tenor.com/hwsbuAcG8UQAAAAM/foxplushy-foxy.gif",
    "https://tenor.com/view/hugtrip-gif-2490966530865073004",
    "https://media.tenor.com/BmbTYhCZ5UsAAAAM/yuri-sleeping-yuri-sleep.gif",
    "https://media.tenor.com/MApGHq5Kvj0AAAAM/anime-hug.gif",
    "https://tenor.com/view/hugtrip-gif-2490966530865073004",
    "https://tenor.com/view/anime-comfort-hug-anime-hug-anime-wrap-hands-anime-hands-around-neck-anime-side-hug-gif-1321262205202367944",
];
/// Обнять пользователя
#[poise::command(
    slash_command,
    rename = "обнять",
    context_menu_command = "Обнять",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn hug(
    ctx: Context<'_>,
    #[description = "Кого обнимаете"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.send(
            CreateReply::default()
                .content("Ты чё обнимать себя собираешься?")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let _ = add_user_quest_progress(
        &ctx.data().pool,
        ctx.serenity_context(),
        ctx.author().id.get(),
        "hug",
        Some(user.id.get()),
        None,
    )
    .await;

    let gif = {
        let mut rng = rand::rng();
        *HUG_GIFS.choose(&mut rng).unwrap()
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} обнял(а) {}",
            ctx.author()
                .global_name
                .as_deref()
                .unwrap_or_else(|| &ctx.author().name),
            user.global_name.as_deref().unwrap_or_else(|| &user.name),
        ))
        .image(gif)
        .colour(serenity::colours::branding::GREEN);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

const PUNCH_GIFS: [&str; 5] = [
    "https://tenor.com/view/spy-family-spy-x-family-anya-cute-punch-gif-25751847",
    "https://tenor.com/view/smash-wall-smash-gif-16637040950689804334",
    "https://tenor.com/view/hxh-hunter-x-hunter-hxh1999-hunter-x-hunter1999-gon-gif-26633516",
    "https://tenor.com/view/vr-anime-girl-punch-gif-15917695959769675167",
    "https://tenor.com/view/some-guy-getting-punch-anime-punching-some-guy-anime-anime-punch-punch-anime-gif-22671439",
];
/// Ударить пользователя
#[poise::command(
    slash_command,
    rename = "ударить",
    context_menu_command = "Ударить",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn punch(
    ctx: Context<'_>,
    #[description = "Кого ударяете"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.send(
            CreateReply::default()
                .content("Ты чё пиздить себя собираешься?")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    // 🔵 ninja_dodge: у жертвы +20% шанс увернуться от удара
    let victim_traits = get_user_traits(&ctx.data().pool, user.id.get()).await?;
    let victim_dodged = victim_traits.contains(&"ninja_dodge".to_string())
        && rand::random_bool(0.2);

    if victim_dodged {
        ctx.send(CreateReply::default().embed(
            serenity::CreateEmbed::default()
                .title(format!(
                    "{} попытался(ась) ударить {}, но тот увернулся как ниндзя!",
                    ctx.author()
                        .global_name
                        .as_deref()
                        .unwrap_or_else(|| &ctx.author().name),
                    user.global_name.as_deref().unwrap_or_else(|| &user.name),
                ))
                .colour(serenity::colours::branding::YELLOW),
        ))
        .await?;
        return Ok(());
    }

    let _ = add_user_quest_progress(
        &ctx.data().pool,
        ctx.serenity_context(),
        ctx.author().id.get(),
        "punch",
        Some(user.id.get()),
        None,
    )
    .await;

    // 🔵 toxic_tongue: у жертвы 5% шанс получить 50 бебр компенсации за моральный ущерб
    let mut compensation_note = String::new();
    if victim_traits.contains(&"toxic_tongue".to_string())
        && rand::random_bool(0.05)
    {
        let result = sqlx::query("UPDATE sbp_users SET balance = balance + 50 WHERE id = $1")
            .bind::<i64>(user.id.get() as i64)
            .execute(&ctx.data().pool)
            .await?;

        if result.rows_affected() > 0 {
            compensation_note = format!(
                "\n{} отсудил(а) 50 бебр компенсации за моральный ущерб!",
                user.global_name.as_deref().unwrap_or_else(|| &user.name)
            );
        }
    }

    let gif = {
        let mut rng = rand::rng();
        *PUNCH_GIFS.choose(&mut rng).unwrap()
    };

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} ударил(а) {}",
            ctx.author()
                .global_name
                .as_deref()
                .unwrap_or_else(|| &ctx.author().name),
            user.global_name.as_deref().unwrap_or_else(|| &user.name),
        ))
        .description(compensation_note)
        .image(gif)
        .colour(serenity::colours::branding::GREEN);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Выстрелить спермой в пользователя
#[poise::command(
    slash_command,
    rename = "камшот",
    context_menu_command = "Камшот",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn cumshot(
    ctx: Context<'_>,
    #[description = "В кого камшотить"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        let msg = ctx.say("Выпускаю сперму себе в глаз...").await?;
        sleep(Duration::from_millis(1_500)).await;
        if rand::random_bool(0.5) {
            msg.edit(
                ctx,
                CreateReply::default().content("Успешно попал спермой себе в глаз"),
            )
            .await?;
        } else {
            msg.edit(
                ctx,
                CreateReply::default().content("Увернулся от своей же спермы"),
            )
            .await?;
        }
    } else {
        let msg = ctx
            .say(format!(
                "Выпускаю сперму в {}...",
                user.global_name.as_deref().unwrap_or_else(|| &user.name)
            ))
            .await?;
        sleep(Duration::from_millis(1_500)).await;

        let pool = &ctx.data().pool;

        // 🔵 rubber_body: у жертвы 15% шанс отразить камшот обратно в атакующего
        let victim_traits = get_user_traits(pool, user.id.get()).await?;
        let author_traits = get_user_traits(pool, ctx.author().id.get()).await?;
        if victim_traits.contains(&"rubber_body".to_string()) && rand::random_bool(0.15) {
            msg.edit(
                ctx,
                CreateReply::default().content(format!(
                    "Резиновая плоть {} отражает вашу же сперму прямо вам в глаз!",
                    user.global_name.as_deref().unwrap_or_else(|| &user.name)
                )),
            )
            .await?;
            return Ok(());
        }

        // 🟡 cum_god: гарантированное попадание, минуя всё
        let has_cum_god = author_traits.contains(&"cum_god".to_string());

        let mut hit_chance: f64 = 0.5;
        // 🔵 sperm_sniper: +25% к шансу попасть
        if author_traits.contains(&"sperm_sniper".to_string()) {
            hit_chance += 0.25;
        }
        // 🔵 ninja_dodge: у жертвы +20% к уклонению
        if victim_traits.contains(&"ninja_dodge".to_string()) {
            hit_chance -= 0.2;
        }
        // 🟢 cheap_skincare: у жертвы -10% шанс словить камшот в лицо
        if victim_traits.contains(&"cheap_skincare".to_string()) {
            hit_chance -= 0.1;
        }

        let is_hit = has_cum_god || rand::random_bool(hit_chance.clamp(0.0, 1.0));

        if is_hit {
            let _ = add_user_quest_progress(
                pool,
                ctx.serenity_context(),
                ctx.author().id.get(),
                "cumshot",
                Some(user.id.get()),
                None,
            )
            .await;
            msg.edit(
                ctx,
                CreateReply::default().content(format!(
                    "Успешно попал спермой в глаз {}",
                    user.global_name.as_deref().unwrap_or_else(|| &user.name)
                )),
            )
            .await?;
        } else {
            msg.edit(
                ctx,
                CreateReply::default().content(format!(
                    "{} уворачивается от спермы!",
                    user.global_name.as_deref().unwrap_or_else(|| &user.name)
                )),
            )
            .await?;
        }
    }

    Ok(())
}

/// Отсосать пользователю
#[poise::command(
    slash_command,
    rename = "минет",
    context_menu_command = "Отсосать",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn blowjob(
    ctx: Context<'_>,
    #[description = "Кому сосать"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        let msg = ctx.say("Вы пытаетесь сделать само-отсос...").await?;
        sleep(Duration::from_millis(3_500)).await;
        if rand::random_bool(0.3) {
            let _ = add_user_quest_progress(
                &ctx.data().pool,
                ctx.serenity_context(),
                ctx.author().id.get(),
                "self-minet",
                Some(ctx.author().id.get()),
                None,
            )
            .await;
            msg.edit(
                ctx,
                CreateReply::default().content("Вы успешно отсосали самому себе"),
            )
            .await?;
        } else {
            msg.edit(
                ctx,
                CreateReply::default().content("Вы не смогли отсосать самому себе..."),
            )
            .await?;
        }
    } else {
        let msg = ctx
            .say(format!(
                "Вы сосёте {}...",
                user.global_name.as_deref().unwrap_or_else(|| &user.name)
            ))
            .await?;
        sleep(Duration::from_millis(3_500)).await;

        // 🔵 vacuum_throat: +20% к шансу успешного минета
        let mut success_chance: f64 = 0.5;
        let author_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
        if author_traits.contains(&"vacuum_throat".to_string()) {
            success_chance += 0.2;
        }

        if rand::random_bool(success_chance.min(1.0)) {
            let _ = add_user_quest_progress(
                &ctx.data().pool,
                ctx.serenity_context(),
                ctx.author().id.get(),
                "minet",
                Some(user.id.get()),
                None,
            )
            .await;
            msg.edit(
                ctx,
                CreateReply::default().content(format!(
                    "Вы успешно довели до оргазма {}",
                    user.global_name.as_deref().unwrap_or_else(|| &user.name)
                )),
            )
            .await?;
        } else {
            msg.edit(
                ctx,
                CreateReply::default().content(format!(
                    "Вы не смогли заставить кончить {} :(",
                    user.global_name.as_deref().unwrap_or_else(|| &user.name)
                )),
            )
            .await?;
        }
    }

    Ok(())
}

/// Нафутджобить пользователю
#[poise::command(
    slash_command,
    rename = "футджоб",
    context_menu_command = "Футджоб",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn footjob(
    ctx: Context<'_>,
    #[description = "Кому футджобить"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.send(
            CreateReply::default()
                .content("Ты чё футджобить себе собираешься?")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let msg = ctx
        .say(format!(
            "Вы пытаетесь сделать футджоб {}...",
            user.global_name.as_deref().unwrap_or_else(|| &user.name)
        ))
        .await?;
    sleep(Duration::from_millis(3_500)).await;

    // 🔵 footjober: +20% к шансу успешного футджоба
    let mut success_chance: f64 = 0.5;
    let author_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
    if author_traits.contains(&"footjober".to_string()) {
        success_chance += 0.2;
    }

    if rand::random_bool(success_chance.min(1.0)) {
        let _ = add_user_quest_progress(
            &ctx.data().pool,
            ctx.serenity_context(),
            ctx.author().id.get(),
            "footjob",
            Some(user.id.get()),
            None,
        )
        .await;
        msg.edit(
            ctx,
            CreateReply::default().content(format!(
                "Вы успешно сделали футджоб {}",
                user.global_name.as_deref().unwrap_or_else(|| &user.name)
            )),
        )
        .await?;
    } else {
        msg.edit(
            ctx,
            CreateReply::default().content(format!(
                "Вы не смогли сделать футджоб {} :(",
                user.global_name.as_deref().unwrap_or_else(|| &user.name)
            )),
        )
        .await?;
    }

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        yes_or_no(),
        monetka(),
        russian_roulette(),
        kosti(),
        slava(),
        kiss(),
        hug(),
        punch(),
        cumshot(),
        blowjob(),
        footjob(),
        rps::rps(),
        kys::kys(),
        sex::sex(),
    ]
}
