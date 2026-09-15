use crate::types::*;

pub async fn handle_main_menu_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    _data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
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
