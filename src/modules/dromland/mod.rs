use crate::types::*;
use poise::serenity_prelude::{self as serenity};

mod game;
mod character_info;
mod create_character;

pub fn display_class(raw_class: &str) -> Option<&str> {
    match raw_class {
        "mage" => Some("маг"),
        "warrior" => Some("воин"),
        "heavy" => Some("танк"),
        _ => None
    }
}

pub async fn handle_dromland_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.as_str();

    if custom_id == "dl:create_char" {
        create_character::handle_char_create_button(ctx, press, data).await?;
        return Ok(());
    }

    let dl_user: Option<DlUser> = sqlx::query_as(
        "SELECT * FROM dl_users WHERE id = $1"
    )
        .bind(press.user.id.get() as i64)
        .fetch_optional(&data.pool)
        .await?;

    let Some(dl_user) = dl_user else {
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::default()
                .content("Сначала создай персонажа")
                .ephemeral(true)
        );
        return Ok(());
    };

    match custom_id {
        "dl:char_info" => {
            character_info::handle_char_info_button(ctx, press, data, dl_user).await?;
        }
        _ => {}
    }

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        game::game()
    ]
}
