use crate::types::*;

mod buy_access;

pub async fn handle_mining_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    if press.data.custom_id.starts_with("mining:mm") {
        // let mm = get_main_menu(&data.pool, press.user.id.get()).await?;
        // crate::create_edit_response!(
        //     ctx,
        //     press,
        //     serenity::CreateInteractionResponseMessage::new()
        //         .content("")
        //         .components(mm.0)
        //         .embed(mm.1)
        // )
    } else if press.data.custom_id.starts_with("mining:buy_access") {
        buy_access::handle_buy_access_button(ctx, press, data).await?
    }

    Ok(())
}