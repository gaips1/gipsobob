use pretty_decimal::PrettyDecimal;
use rust_decimal::Decimal;

use crate::checks::sbp_check;
use crate::{
    modules::{
        dialogues::get_dialogue,
        mining::{helpers::bar, types::*},
    },
    types::*,
};

pub fn get_main_menu(
    user: MiningUser<'_>,
) -> (serenity::CreateEmbed, Vec<serenity::CreateActionRow>) {
    let time_until_restart = chrono::Utc::now() - user.restarted_at;
    let hours_until_restart = time_until_restart.num_hours();

    let user_earn = user.earn_per_second().round_dp(3);
    let user_earn = if user_earn > Decimal::ZERO {
        format!("+{} UCS/сек", PrettyDecimal::comma3dot(user_earn))
    } else if user_earn < Decimal::ZERO {
        format!("{} UCS/сек", PrettyDecimal::comma3dot(user_earn))
    } else {
        "0 UCS/сек".to_string()
    };

    let mut embed = serenity::CreateEmbed::new()
        .title(format!("{} · {} UCS", user.location.name, PrettyDecimal::comma3dot(user.balance)))
        .field(
            "⚡ Энергопотребление",
            format!(
                "{} {}/{} Ватт",
                bar(
                    user.videocards_power_sum() as f64,
                    user.location.max_power as f64,
                    10
                ),
                user.videocards_power_sum(),
                user.location.max_power,
            ),
            false,
        )
        .field("💰 Чистый доход", user_earn, false)
        .colour(serenity::colours::branding::BLURPLE);

    if hours_until_restart >= 24 {
        embed = embed.field("💥 Ваш сервер перегружен! 💥",
            "Видеокарты перегрелись и больше не могут работать, перезапустите сервер!\n\
        ||Спасибо `Cool'Cold'у` за программу, которая автоматически выключает видеокарты, иначё всё бы сгорело...||",
            false
        );
    } else if user.videocards_power_sum() > user.location.max_power as u64 {
        embed = embed.field(
            "💥 Ваш сервер перегружен! 💥",
            "Вы превысили максимальное энергопотребление на вашей локации!\n\
            Переедьте в новую локацию или продайте видеокарты, чтобы продолжить майнить.",
            false,
        )
    } else {
        embed = embed.field(
            "💥 Перегрузка сервера",
            bar(hours_until_restart as f64, 24.0, 14),
            false,
        )
    }

    let buttons = vec![
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("mining:restart")
                .label("🔁 Перезагрузить сервер")
                .style(serenity::ButtonStyle::Success),
            serenity::CreateButton::new("mining:shop")
                .label("🛒 Магазин карт")
                .style(serenity::ButtonStyle::Primary),
        ]),
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("mining:locations")
                .label("🏘️ Локации")
                .style(serenity::ButtonStyle::Primary),
            serenity::CreateButton::new("mining:power_info")
                .label("⚡ Электричество")
                .style(serenity::ButtonStyle::Primary),
        ]),
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("mining:trading")
                .label("🔁 Обменник")
                .style(serenity::ButtonStyle::Primary),
        ]),
    ];

    (embed, buttons)
}

/// Ваша майнинг-ферма
#[poise::command(
    slash_command,
    rename = "майнинг",
    check = "sbp_check",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn mining(ctx: Context<'_>) -> Result<(), Error> {
    let Some(mining_user) = MiningUser::get(&ctx.data().pool, ctx.author()).await else {
        let dialogue = get_dialogue("mining:first_hi").unwrap();
        ctx.send(
            poise::CreateReply::default()
                .content(dialogue.content)
                .components(dialogue.buttons)
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    };

    let mm = get_main_menu(mining_user);
    ctx.send(
        poise::CreateReply::default()
            .embed(mm.0)
            .components(mm.1)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
