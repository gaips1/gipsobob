use super::MiningUser;
use crate::types::*;

pub async fn handle_restart_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    sqlx::query("UPDATE mining_users SET restarted_at = NOW()")
        .execute(&data.pool)
        .await?;

    let mining_user = MiningUser::get(&data.pool, &press.user).await.unwrap();
    let mm = super::get_main_menu(mining_user);

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("")
            .embed(mm.0)
            .components(mm.1)
    );

    Ok(())
}
