use crate::types::*;

#[poise::command(prefix_command, rename = "бан")]
pub async fn ban(ctx: Context<'_>, user_id: serenity::UserId) -> Result<(), Error> {
    let poise::Context::Prefix(poise::PrefixContext { msg, .. }) = ctx else {
        return Ok(());
    };

    if ctx.author().id.get() != 449882524697493515 {
        return Ok(());
    }

    let _ = sqlx::query("UPDATE users SET is_banned = NOT is_banned WHERE id = $1")
        .bind(user_id.get() as i64)
        .execute(&ctx.data().pool)
        .await;

    let _ = msg.delete(ctx).await;

    Ok(())
}

#[poise::command(prefix_command, rename = "рассылка")]
pub async fn say_to_all(ctx: Context<'_>, text: String) -> Result<(), Error> {
    if ctx.author().id.get() != 449882524697493515 {
        return Ok(());
    }

    ctx.reply("Начинаю пересылку...").await?;

    let user_ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM users")
        .fetch_all(&ctx.data().pool)
        .await?;

    for user_id in user_ids {
        let user = serenity::UserId::new(user_id as u64);
        let _ = user
            .dm(ctx, serenity::CreateMessage::new().content(&text))
            .await;
    }

    let _ = ctx.reply("Успешно переслал!").await;

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![ban(), say_to_all()]
}
