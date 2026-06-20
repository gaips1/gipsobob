mod sex;

use poise::serenity_prelude as serenity;
use poise::{CreateReply};
use rand::prelude::*;
use tokio::time::{sleep, Duration};
use std::sync::OnceLock;

use crate::types::*;

/// Да или нет
#[poise::command(slash_command, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
async fn yes_or_no(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(if rand::random_bool(0.5) { "Да" } else { "Нет" }).await?;
    Ok(())
}

/// Подбросить монетку
#[poise::command(slash_command, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
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
    msg.edit(ctx, CreateReply::default().content(choice)).await?;
    Ok(())
}

/// Русская рулетка
#[poise::command(slash_command, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
async fn russian_roulette(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.say("Вставляю пулю...").await?;
    sleep(Duration::from_millis(1_500)).await;
    msg.edit(ctx, CreateReply::default().content("Раскручиваю барабан..")).await?;
    sleep(Duration::from_millis(1_500)).await;
    msg.edit(ctx, CreateReply::default().content(if rand::random_bool(0.167) { "Бум! Тебе разорвало лицо" } else { "Повезло, ты остался жив" })).await?;
    Ok(())
}

/// Кинуть кости
#[poise::command(slash_command, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
async fn kosti(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.say("Кидаю...").await?;
    sleep(Duration::from_millis(2_500)).await;
    msg.edit(ctx, CreateReply::default().content(format!("Выпало число {}", rand::random_range(1..=6)))).await?;
    Ok(())
}

/// Слава узбии!
#[poise::command(slash_command, rename = "слава_узбии", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
async fn slava_uzbii(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        CreateReply::default()
            .embed(
                serenity::CreateEmbed::default()
                    .title("Слава узбии!")
                    .color(serenity::colours::branding::RED)
            )
    ).await?;
    Ok(())
}

static KYS_LIST: OnceLock<Vec<String>> = OnceLock::new();
pub fn get_kys_list() -> &'static [String] {
    KYS_LIST.get_or_init(|| {
        let data = include_str!("kys.json");
        serde_json::from_str(data).expect("Failed to parse kys.json")
    })
}
/// KEEP YOURSELF SAFE
#[poise::command(slash_command, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
async fn kys(ctx: Context<'_>) -> Result<(), Error> {
    let list = get_kys_list();
    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    ctx.send(
        CreateReply::default()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(
                vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("kys_btn")
                        .label("KYS")
                        .emoji('☠')
                        .style(serenity::ButtonStyle::Danger)
                ])]
            )
            .ephemeral(true)
    ).await?;

    Ok(())
}

pub async fn handle_kys_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let list = get_kys_list();

    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    interaction.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::new()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(
                vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("kys_btn")
                        .label("KYS")
                        .emoji('☠')
                        .style(serenity::ButtonStyle::Danger)
                ])]
            )
    )).await?;
    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        yes_or_no(),
        monetka(),
        russian_roulette(),
        kosti(),
        slava_uzbii(),
        kys()
    ]
}