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
    let embed = serenity::CreateEmbed::new()
        .title(format!("{} · {} UCS", user.location.name, user.balance))
        .description(format!(
            "⚡ Энергопотребление {} {}/{} Вт\n\
            💰 Чистый доход:   +{} UCS/сек\n",
            bar(
                user.videocards_power_sum() as f64,
                user.location.max_power as f64,
                10
            ),
            user.videocards_power_sum(),
            user.location.max_power,
            user.videocards_earn_per_second_sum(),
        ))
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("mining:take_money")
                .label("🔨 Забрать доход")
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
