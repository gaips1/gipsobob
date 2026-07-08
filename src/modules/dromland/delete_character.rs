use crate::{modules::dromland::game::get_main_menu_buttons, types::*};
use pretty_decimal::PrettyDecimal;

pub async fn handle_char_delete_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    dl_user: DlUser,
) -> Result<(), Error> {
    match press.data.custom_id.as_str() {
        "dl:char_delete" => {
            let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("dl:char_delete:yes")
                    .label("Да")
                    .style(serenity::ButtonStyle::Danger),
                serenity::CreateButton::new("dl:char_delete:no")
                    .label("Нет")
                    .style(serenity::ButtonStyle::Success),
            ])];

            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .content(format!(
                        "Вы действительно хотите удалить персонажа?\nВы потеряете {} монет!",
                        PrettyDecimal::comma3dot(dl_user.balance)
                    ))
                    .components(buttons)
                    .embeds(Vec::new())
            );
        }

        "dl:char_delete:yes" => {
            let _ = sqlx::query("DELETE FROM dl_users WHERE id = $1")
                .bind(press.user.id.get() as i64)
                .execute(&data.pool)
                .await;

            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Пока, путник!")
                    .components(Vec::new())
                    .embeds(Vec::new())
            );
        }

        "dl:char_delete:no" => {
            crate::create_edit_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Молодец, что одумался!")
                    .components(get_main_menu_buttons())
                    .embeds(Vec::new())
            );
        }
        _ => {}
    }

    Ok(())
}
