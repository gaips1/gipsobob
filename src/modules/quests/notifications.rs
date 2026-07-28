use crate::types::*;

pub async fn handle_notifications_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    custom_id: &str,
) -> Result<(), Error> {
    match custom_id {
        "on" => {
            sqlx::query("UPDATE users SET quest_notifications = true WHERE id = $1")
                .bind(press.user.id.get() as i64)
                .execute(&data.pool)
                .await?;

            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .components(super::helpers::get_notifications_button(false))
            );
        }
        "off" => {
            sqlx::query("UPDATE users SET quest_notifications = false WHERE id = $1")
                .bind(press.user.id.get() as i64)
                .execute(&data.pool)
                .await?;

            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .components(super::helpers::get_notifications_button(true))
            );
        }
        _ => {}
    }

    Ok(())
}
