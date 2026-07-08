use crate::types::*;
use poise::serenity_prelude::{self as serenity};

mod character_info;
mod create_character;
mod delete_character;
mod game;
mod shop;

pub const fn display_class(raw_class: &str) -> Option<&str> {
    match raw_class.as_bytes() {
        b"mage" => Some("маг"),
        b"warrior" => Some("воин"),
        b"heavy" => Some("танк"),
        _ => None,
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

    let dl_user: Option<DlUser> = sqlx::query_as("SELECT * FROM dl_users WHERE id = $1")
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

    if custom_id.starts_with("dl:char_delete") {
        delete_character::handle_char_delete_button(ctx, press, data, dl_user).await?;
    } else {
        match custom_id {
            "dl:char_info" => {
                character_info::handle_char_info_button(ctx, press, data, dl_user).await?;
            }
            "dl:shop" => {
                shop::handle_shop_button(ctx, press, data).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![game::game()]
}
