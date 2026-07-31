use crate::{helpers::check_user_flag, modules::dialogues::DialoguesManager, types::*};

mod types;

pub fn get_main_menu() -> (Vec<serenity::CreateActionRow>, serenity::CreateEmbed) {
    let dialogue = DialoguesManager::get_dialogue("traits:main_menu").unwrap();

    let embed = serenity::CreateEmbed::new()
        .title("Мутации")
        .description(
            format!(
                "{}\n\n \
                🧬 Твои мутации:
                ",
                dialogue.content
            )
        )
        .colour(serenity::colours::branding::BLACK);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("Fekfk")
            .label("Да")
            .style(serenity::ButtonStyle::Success)
    ])];

    (buttons, embed)
}

/// Что же доктор вколет в этот раз?
#[poise::command(
    slash_command,
    rename = "мутации",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn traits(ctx: Context<'_>) -> Result<(), Error> {
    if !check_user_flag(&ctx.data().pool, ctx.author().id.get(), "is_traits_opened").await? {
        let dialogue = DialoguesManager::get_dialogue("traits:first_hi").unwrap();
        ctx.send(
            poise::CreateReply::default()
                .content(dialogue.content)
                .components(dialogue.buttons)
                .ephemeral(true)
        ).await?;
        return Ok(());
    }

    let mm = get_main_menu();
    ctx.send(
        poise::CreateReply::default()
            .components(mm.0)
            .embed(mm.1)
            .ephemeral(true)
    ).await?;

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![traits()]
}