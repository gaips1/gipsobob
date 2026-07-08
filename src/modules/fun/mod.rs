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
    msg.edit(ctx, CreateReply::default().content(choice))
        .await?;
    Ok(())
}

/// Русская рулетка
#[poise::command(
    slash_command,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn russian_roulette(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.say("Вставляю пулю...").await?;
    sleep(Duration::from_millis(1_500)).await;
    msg.edit(ctx, CreateReply::default().content("Раскручиваю барабан.."))
        .await?;
    sleep(Duration::from_millis(1_500)).await;
    msg.edit(
        ctx,
        CreateReply::default().content(if rand::random_bool(0.167) {
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

const HUG_GIFS: [&str; 5] = [
    "https://media.tenor.com/hwsbuAcG8UQAAAAM/foxplushy-foxy.gif",
    "https://media.tenor.com/WIOsEr_4XFcAAAAM/happy-anime.gif",
    "https://media.tenor.com/BmbTYhCZ5UsAAAAM/yuri-sleeping-yuri-sleep.gif",
    "https://media.tenor.com/MApGHq5Kvj0AAAAM/anime-hug.gif",
    "https://media.tenor.com/iEDbr-ZhHMkAAAAM/anime-hug.gif",
];
/// Обнять пользователя
#[poise::command(
    slash_command,
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
    "https://media.tenor.com/jnndDmOm5wMAAAAC/kiss.gif",
    "https://media.tenor.com/fiafXWajQFoAAAAC/kiss-anime.gif",
    "https://media.tenor.com/dn_KuOESmUYAAAAC/engage-kiss-anime-kiss.gif",
    "https://media.tenor.com/9jB6M6aoW0AAAAAM/val-ally-kiss.gif",
    "https://media.tenor.com/SYwRyd6N1UIAAAAC/anime-kiss.gif",
];
/// Ударить пользователя
#[poise::command(
    slash_command,
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
    context_menu_command = "Отсосать",
    name_localized("ru", "отсосать"),
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
    context_menu_command = "Футджоб",
    name_localized("ru", "футджоб"),
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
