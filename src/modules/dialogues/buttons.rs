use crate::{modules::dialogues::get_dialogue, types::*};

pub async fn handle_dialogue_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.strip_prefix("dialogue:").unwrap();

    let Some(dialogue) = get_dialogue(custom_id) else {
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Диалог не найден. Попробуйте снова.")
                .embeds(Vec::new())
                .components(Vec::new())
        );
        return Ok(());
    };

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content(dialogue.content)
            .embeds(Vec::new())
            .components(dialogue.buttons)
    );

    Ok(())
}
