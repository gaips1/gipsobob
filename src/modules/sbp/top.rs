use rust_decimal::Decimal;
use std::fmt::Write;

use crate::types::*;

/// Топ самых богатых людей
#[poise::command(
    slash_command,
    rename = "таблица-лидеров",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn top(ctx: Context<'_>) -> Result<(), Error> {
    let users: Vec<(i64, Decimal)> =
        sqlx::query_as("SELECT id, balance FROM sbp_users ORDER BY balance ASC LIMIT 10")
            .fetch_all(&ctx.data().pool)
            .await?;

    let mut text = String::new();
    for (i, &m) in users.iter().enumerate() {
        let _ = write!(text, "**{}.** <@{}> - {} бебр\n", i + 1, m.0, m.1,);
    }

    let embed = serenity::CreateEmbed::default()
        .title("Топ 10 самых богатых людей в гипсобобе")
        .description(text)
        .colour(serenity::colours::branding::BLURPLE);

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}
