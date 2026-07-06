use crate::types::*;
use poise::serenity_prelude::{self as serenity};
use std::fmt::Write;

/// Топ 10 гаремов по количеству пользователей
#[poise::command(
    slash_command,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn harems(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let harems: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT COUNT(all_users.id), h.author_id \
        FROM users target_user \
        INNER JOIN harems h ON target_user.harem_id = h.id \
        INNER JOIN users all_users ON h.id = all_users.harem_id \
        WHERE target_user.id = $1 \
        GROUP BY h.author_id \
        LIMIT 10;",
    )
    .bind(ctx.author().id.get() as i64)
    .fetch_all(pool)
    .await?;

    if harems.len() == 0 {
        ctx.say("К сожалению, гаремов пока нет.").await?;
        return Ok(());
    }

    let mut text = String::new();
    for (i, &h) in harems.iter().enumerate() {
        let _ = write!(text, "**{}.** <@{}> - {} пользователей\n", i + 1, h.1, h.0);
    }

    let embed = serenity::CreateEmbed::default()
        .title("Топ 10 гаремов по количеству пользователей (включая создателя)")
        .description(text)
        .colour(serenity::colours::branding::BLURPLE);

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}
