use crate::{types::*};

/// Создать свой гарем
#[poise::command(slash_command, ephemeral, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn create_harem(ctx: Context<'_>,) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let author_harem_id: Option<i64> = sqlx::query_scalar(
        "SELECT harem_id FROM users WHERE id = $1"
    )
        .bind(ctx.author().id.get() as i64)
        .fetch_optional(pool)
        .await?
        .unwrap();

    if author_harem_id.is_some() {
        ctx.say("Вы уже находитесь в гареме").await?;
        return Ok(());
    }
    
    let mut tx = pool.begin().await?;

    let harem_id: i64 = sqlx::query_scalar(
        "INSERT INTO harems (author_id) VALUES ($1) RETURNING id"
    )
        .bind(ctx.author().id.get() as i64)
        .fetch_one(&mut *tx)
        .await?;

    sqlx::query(
        "UPDATE users SET harem_id = $1 WHERE id = $2"
    )
        .bind(harem_id)
        .bind(ctx.author().id.get() as i64)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    ctx.say("Вы успешно создали свой гарем\nИспользуйте команду `/harem` для управления гаремом").await?;

    Ok(())
}