use crate::types::*;
use crate::modules::*;

pub async fn route_button_interaction(
    ctx: &serenity::Context,
    component: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = component.data.custom_id.as_str();
    let prefix = custom_id.split(':').next().unwrap_or(custom_id);

    match prefix {
        "casino" => sbp::casino::handle_casino_buttons(ctx, component, data).await?,
        "sbp" => sbp::handle_sbp_buttons(ctx, component, data).await?,
        "marriage" => marriages::handle_marriages_buttons(ctx, component, data).await?,
        "harem" => harems::handle_harems_buttons(ctx, component, data).await?,
        "dl" => dromland::handle_dromland_buttons(ctx, component, data).await?,
        "giveaway" => giveaways::handle_giveaway_buttons(ctx, component, data).await?,
        "quests" => quests::handle_quests_buttons(ctx, component, data).await?,
        "dialogue" => dialogues::buttons::handle_dialogue_buttons(ctx, component).await?,
        "traits" => traits::main_menu::handle_traits_buttons(ctx, component, data).await?,
        "mining" => mining::buttons::handle_mining_buttons(ctx, component, data).await?,
        "kys_btn" => fun::kys::handle_kys_button(ctx, component, data).await?,
        _ => {}
    }

    Ok(())
}

pub async fn route_string_select_interaction(
    ctx: &serenity::Context,
    component: &serenity::ComponentInteraction,
    data: &Data,
    values: &[String],
) -> Result<(), Error> {
    let custom_id = component.data.custom_id.as_str();

    match custom_id {
        "quest_status_select" => {
            quests::handle_quests_select(ctx, component, data, values).await?;
        }
        _ => {}
    }

    Ok(())
}
