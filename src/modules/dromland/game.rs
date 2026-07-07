use crate::{modules::dromland::display_class, types::*};
use poise::serenity_prelude::{self as serenity};

pub fn get_main_menu_buttons() -> Vec<serenity::CreateActionRow> {
    vec![
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("dl:enter")
                .label("Отправиться в лабиринт")
                .style(serenity::ButtonStyle::Danger),

            serenity::CreateButton::new("dl:char_info")
                .label("Информация о персонаже")
                .style(serenity::ButtonStyle::Success)
        ]),

        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("dl:delete_char")
                .label("Удалить персонажа")
                .style(serenity::ButtonStyle::Danger),

            serenity::CreateButton::new("dl:shop")
                .label("Магазин")
                .style(serenity::ButtonStyle::Success)
        ]),

        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("dl:donate")
                .label("Донат")
        ])
    ]
}

/// Войти в Дромляндия: Онлайн
#[poise::command(
    slash_command,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn game(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let dl_user: Option<(String, String, bool)> = sqlx::query_as("SELECT name, class, in_game FROM dl_users WHERE id = $1")
        .bind(ctx.author().id.get() as i64)
        .fetch_optional(pool)
        .await?;

    let Some(dl_user) = dl_user else {
        let buttons = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("dl:create_char")
                .label("Создать персонажа")
        ])];

        ctx.send(
            poise::CreateReply::default()
                .content("Добро пожаловать в края Дромляндии, путник! Создай своего персонажа, нажав на кнопку ниже")
                .components(buttons)
                .ephemeral(true)
        ).await?;
        return Ok(());
    };

    if dl_user.2 {
        ctx.say("Вы в данный момент в лабиринте").await?;
        return Ok(());
    }
    
    ctx.send(
        poise::CreateReply::default()
            .content(format!("Добро пожаловать обратно, {} {}", display_class(dl_user.1.as_str()).unwrap(), dl_user.0))
            .components(get_main_menu_buttons())
            .ephemeral(true)
    ).await?;

    Ok(())
}
