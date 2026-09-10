use crate::modules::dialogues::get_dialogue;

pub use super::main_menu::get_main_menu;
pub use super::types::*;
pub use super::*;

mod buy_access;
mod locations;
mod main_menu;
mod restart;
mod trading;

pub async fn handle_mining_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.as_str();

    if custom_id == "mining:buy_access" {
        return buy_access::handle_buy_access_button(ctx, press, data).await;
    }

    let Some(mining_user) = MiningUser::get(&data.pool, &press.user).await else {
        let dialogue = get_dialogue("mining:first_hi").unwrap();

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content(dialogue.content)
                .embeds(Vec::new())
                .components(dialogue.buttons)
        );

        return Ok(());
    };

    if custom_id.starts_with("mining:locations") {
        locations::handle_locations_buttons(ctx, press, data, mining_user).await?;
        return Ok(());
    } else if custom_id.starts_with("mining:trading") {
        trading::handle_trading_button(ctx, press, data, mining_user).await?;
        return Ok(());
    }

    match custom_id {
        "mining:mm" => main_menu::handle_main_menu_button(ctx, press, data, mining_user).await?,
        "mining:restart" => restart::handle_restart_button(ctx, press, data).await?,
        _ => {}
    }

    Ok(())
}
