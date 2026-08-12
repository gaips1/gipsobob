use sqlx::types::Json;
use std::collections::HashMap;

use crate::{modules::{dialogues::get_dialogue, mining::types::*}, types::*};

pub async fn get_main_menu(user_id: u64, user: MiningUser<'_>) -> Result<(), Error> {
    Ok(())
}

/// Ваша майнинг ферма
#[poise::command(
    slash_command,
    rename = "майнинг",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn mining(ctx: Context<'_>) -> Result<(), Error> {
    let row: Option<(i64, String, Json<HashMap<String, u64>>)> =
        sqlx::query_as("SELECT balance, location, videocards FROM mining_users WHERE id = $1")
            .bind(ctx.author().id.get() as i64)
            .fetch_optional(&ctx.data().pool)
            .await?;

    let Some((balance, location, Json(user_videocards))) = row else {
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

    let all_videocards = super::get_videocards();
    let all_locations = super::get_locations();

    let mining_user = MiningUser {
        serenity_user: ctx.author(),
        balance: balance as u64,
        location: all_locations.get(&location).expect("unknown location in DB"),
        videocards: user_videocards
            .into_iter()
            .map(|(id, count)| (all_videocards.get(&id).expect("unknown videocard in DB"), count))
            .collect(),
    };

    Ok(())
}
