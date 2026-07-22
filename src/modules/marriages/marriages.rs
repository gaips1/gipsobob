use crate::types::*;
use std::fmt::Write;

/// Показать топ браков по времени
#[poise::command(
    slash_command,
    ephemeral,
    rename = "таблица-лидеров",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn marriages(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let marriages: Vec<(i64, i64, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT user_id, partner_id, created_at FROM marriages ORDER BY created_at ASC LIMIT 10",
    )
    .fetch_all(pool)
    .await?;

    if marriages.len() == 0 {
        ctx.say("К сожалению, браков пока нет.").await?;
        return Ok(());
    }

    let mut text = String::new();
    for (i, &m) in marriages.iter().enumerate() {
        let _ = write!(
            text,
            "**{}.** <@{}> и <@{}> - <t:{}:R>\n",
            i + 1,
            m.0,
            m.1,
            m.2.timestamp()
        );
    }

    let embed = serenity::CreateEmbed::default()
        .title("Топ 10 браков по времени существования")
        .description(text)
        .colour(serenity::colours::branding::BLURPLE);

    ctx.send(poise::CreateReply::default().embed(embed).ephemeral(true))
        .await?;

    Ok(())
}
