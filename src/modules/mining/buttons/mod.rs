use crate::types::*;

mod buy_access;

pub async fn handle_mining_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.as_str();

    match custom_id {
        "mining:mm" => todo!(),
        "mining:buy_access" => buy_access::handle_buy_access_button(ctx, press, data).await?,
        _ => {}
    }

    Ok(())
}
