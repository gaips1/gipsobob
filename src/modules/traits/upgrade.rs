use crate::{modules::dialogues, types::*};
use rust_decimal::Decimal;

pub async fn handle_traits_upgrade_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let user_id = press.user.id.get() as i64;

    let mut tx = data.pool.begin().await?;

    let unlocked_slots: i16 = sqlx::query_scalar(
        "SELECT unlocked_traits_slots \
        FROM traits_users \
        WHERE id = $1 \
        FOR UPDATE",
    )
    .bind(user_id)
    .fetch_one(&data.pool)
    .await?;

    if unlocked_slots >= 3 {
        tx.rollback().await?;

        let dialogue = dialogues::get_dialogue("traits:upgrade:max").unwrap();

        let embed = serenity::CreateEmbed::new()
            .title("Мутации")
            .description(dialogue.content)
            .colour(serenity::colours::branding::RED);

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(dialogue.buttons)
        );
        return Ok(());
    }

    let user_balance: Decimal =
        sqlx::query_scalar("SELECT balance FROM sbp_users WHERE id = $1 FOR UPDATE")
            .bind(press.user.id.get() as i64)
            .fetch_one(&mut *tx)
            .await?;

    if user_balance < Decimal::from(3000) {
        tx.rollback().await?;

        let dialogue = dialogues::get_dialogue("traits:not_enough_money").unwrap();

        let embed = serenity::CreateEmbed::new()
            .title("Мутации")
            .description(dialogue.content)
            .colour(serenity::colours::branding::RED);

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(dialogue.buttons)
        );
        return Ok(());
    }

    sqlx::query("UPDATE sbp_users SET balance = balance - 3000 WHERE id = $1")
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "UPDATE traits_users SET unlocked_traits_slots = unlocked_traits_slots + 1 WHERE id = $1",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let next_slot = unlocked_slots + 1;
    let dialogue = dialogues::get_dialogue(if next_slot == 2 {
        "traits:upgrade:second_slot"
    } else {
        "traits:upgrade:third_slot"
    })
    .unwrap();

    let embed = serenity::CreateEmbed::new()
        .title("Мутации")
        .description(format!(
            "🔪 Доктор Хальцер радостно потирает руки и достает канцелярский нож:\n\n{}\n\n🔓 Слот № {} успешно разблокирован!",
            dialogue.content,
            next_slot
        ))
        .colour(serenity::colours::branding::GREEN);

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(dialogue.buttons)
    );

    Ok(())
}
