use crate::{helpers::check_user_flag, modules::dialogues::DialoguesManager, types::*};

/// Что же доктор вколет в этот раз?
#[poise::command(
    slash_command,
    rename = "мутации",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
async fn traits(ctx: Context<'_>) -> Result<(), Error> {
    if !check_user_flag(&ctx.data().pool, ctx.author().id.get(), "is_traits_opened").await? {
        let dialogue = DialoguesManager::get_dialogue("traits:first_pre_hi").unwrap();
        ctx.send(
            poise::CreateReply::default()
                .content(dialogue.content)
                .components(dialogue.buttons)
                .ephemeral(true)
        ).await?;
        return Ok(());
    }

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![traits()]
}