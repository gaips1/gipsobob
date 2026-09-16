use crate::{modules::dialogues::get_dialogue, types::*};

pub async fn handle_buy_access_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let mut tx = data.pool.begin().await?;

    let result = sqlx::query(
        "UPDATE sbp_users \
        SET balance = balance - 7000 \
        WHERE id = $1 AND balance >= 7000",
    )
    .bind(press.user.id.get() as i64)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        tx.rollback().await?;

        let dialogue = get_dialogue("mining:first_hi_4").unwrap();

        press
            .edit_reply(
                ctx,
                serenity::CreateInteractionResponseMessage::new()
                    .content(dialogue.content)
                    .components(Vec::new()),
            )
            .await?;

        return Ok(());
    }

    let result = sqlx::query("INSERT INTO mining_users (id) VALUES ($1)")
        .bind(press.user.id.get() as i64)
        .execute(&mut *tx)
        .await;

    match result {
        Ok(_) => {}
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            tx.rollback().await?;
            press
                .edit_reply(
                    ctx,
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Вы уже можете майнить!")
                        .embeds(Vec::new())
                        .components(Vec::new()),
                )
                .await?;
            return Ok(());
        }
        Err(_) => {
            tx.rollback().await?;
            press
                .edit_reply(
                    ctx,
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Произошла страшная ошибка")
                        .embeds(Vec::new())
                        .components(Vec::new()),
                )
                .await?;
            return Ok(());
        }
    }

    tx.commit().await?;

    let dialogue = get_dialogue("mining:successful_buy").unwrap();
    press
        .edit_reply(
            ctx,
            serenity::CreateInteractionResponseMessage::new()
                .content(dialogue.content)
                .components(dialogue.buttons),
        )
        .await?;

    Ok(())
}
