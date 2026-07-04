use crate::{types::*};
use poise::serenity_prelude::{self as serenity};

use crate::modules::sbp::handle_sbp_buttons;
use crate::modules::sbp::casino::handle_casino_buttons;
use crate::modules::fun::kys::handle_kys_button;
use crate::modules::marriages::handle_marriages_buttons;

pub async fn route_button_interaction(
    ctx: &serenity::Context,
    component: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = component.data.custom_id.as_str();

    if custom_id.starts_with("casino:") {
        handle_casino_buttons(ctx, component, data).await?;
    } else if custom_id.starts_with("sbp:") {
        handle_sbp_buttons(ctx, component, data).await?;
    } else if custom_id.starts_with("marriage:") {
        handle_marriages_buttons(ctx, component, data).await?;
    } else {
        match custom_id {
            "kys_btn" => handle_kys_button(ctx, component).await?,
            _ => {}
        }
    }

    Ok(())
}