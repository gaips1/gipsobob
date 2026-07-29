pub mod kys;
mod rps;
mod sex;

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
    let choice = {
        let mut rng = rand::rng();
        [("Орёл!", 45), ("Решка!", 45), ("Ребро!", 10)]
            .choose_weighted(&mut rng, |item| item.1)
            .unwrap()
            .0
    };
    let msg = ctx.say("Подбрасываю...").await?;
    sleep(Duration::from_millis(2_500)).await;
    if choice == "Ребро!" {
        let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "monetka", None, None).await;
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
    let is_dead = rand::random_bool(0.167);
    if is_dead {
        let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "rr", None, None).await;
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
    msg.edit(
        ctx,
        CreateReply::default().content(format!("Выпало число {}", rand::random_range(1..=6))),
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
    let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "slava_uzbii", None, None).await;
    ctx.send(
        CreateReply::default().embed(
            serenity::CreateEmbed::default()
                .title("Слава узбии!")
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

    let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "kiss", Some(user.id.get()), None).await;

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
    "https://tenor.com/view/anime-comfort-hug-anime-hug-anime-wrap-hands-anime-hands-around-neck-anime-side-hug-gif-1321262205202367944"
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

    let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "hug", Some(user.id.get()), None).await;

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

    let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "punch", Some(user.id.get()), None).await;

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
        if rand::random_bool(0.5) {
            let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "cumshot", Some(user.id.get()), None).await;
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
            let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "self-minet", Some(ctx.author().id.get()), None).await;
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
        if rand::random_bool(0.5) {
            let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "minet", Some(user.id.get()), None).await;
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

    if rand::random_bool(0.5) {
        let _ = add_user_quest_progress(&ctx.data().pool, ctx.serenity_context(), ctx.author().id.get(), "footjob", Some(user.id.get()), None).await;
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
