use crate::types::*;

/// Ограбить пользователя
#[poise::command(
    slash_command,
    rename = "ограбить",
    user_cooldown = 86400,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn rob(
    ctx: Context<'_>,
    #[description = "Кого грабить"] user: serenity::User,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.bot {
        ctx.say("Зачем грабить бота").await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.say("Зачем грабить себя").await?;
        return Ok(());
    }

    if rand::random_bool(0.3) {
        let win = rand::random_range(150..=900);

        let result = sqlx::query("UPDATE sbp_users SET balance = balance + $1 WHERE id = $2")
            .bind(win)
            .bind::<i64>(ctx.author().id.into())
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            ctx.say(format!("Вы успешно украли {} бебр!\n||Но у вас не было СБП и вы не получите бебры :(\nЗарегистрируйтесь, используя `/сбп регистрация`!||", win)).await?;
            return Ok(());
        }

        ctx.say(format!("Вы успешно украли {} бебр!", win)).await?;
    } else {
        ctx.say("Вы попались!").await?;
    }

    Ok(())
}
