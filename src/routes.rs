use crate::types::*;

use crate::modules::dialogues::buttons::handle_dialogue_buttons;
use crate::modules::dromland::handle_dromland_buttons;
use crate::modules::fun::kys::handle_kys_button;
use crate::modules::giveaways::handle_giveaway_buttons;
use crate::modules::harems::handle_harems_buttons;
use crate::modules::marriages::handle_marriages_buttons;
use crate::modules::quests::handle_quests_buttons;
use crate::modules::quests::handle_quests_select;
use crate::modules::sbp::casino::handle_casino_buttons;
use crate::modules::sbp::handle_sbp_buttons;

pub async fn route_button_interaction(
    ctx: &serenity::Context,
    component: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = component.data.custom_id.as_str();
    let prefix = custom_id.split(':').next().unwrap_or(custom_id);

    match prefix {
        "casino" => handle_casino_buttons(ctx, component, data).await?,
        "sbp" => handle_sbp_buttons(ctx, component, data).await?,
        "marriage" => handle_marriages_buttons(ctx, component, data).await?,
        "harem" => handle_harems_buttons(ctx, component, data).await?,
        "dl" => handle_dromland_buttons(ctx, component, data).await?,
        "giveaway" => handle_giveaway_buttons(ctx, component, data).await?,
        "quests" => handle_quests_buttons(ctx, component, data).await?,
        "dialogue" => handle_dialogue_buttons(ctx, component).await?,
        "kys_btn" => handle_kys_button(ctx, component).await?,
        _ => {}
    }

    Ok(())
}

pub async fn route_string_select_interaction(
    ctx: &serenity::Context,
    component: &serenity::ComponentInteraction,
    data: &Data,
    values: &Vec<String>,
) -> Result<(), Error> {
    let custom_id = component.data.custom_id.as_str();

    match custom_id {
        "quest_status_select" => {
            handle_quests_select(ctx, component, data, values).await?;
        }
        _ => {}
    }

    Ok(())
}
