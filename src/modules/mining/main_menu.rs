use crate::{modules::{dialogues::get_dialogue, mining::types::*}, types::*};
use crate::checks::sbp_check;

pub fn get_main_menu(user: MiningUser<'_>) -> Result<(), Error> {
    Ok(())
}

/// Ваша майнинг ферма
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

    let mm = get_main_menu(mining_user)?;

    Ok(())
}
