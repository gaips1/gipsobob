use crate::types::*;

pub async fn handle_locations_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    _data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let locations = super::get_locations();

    let mut embed = serenity::CreateEmbed::new()
        .title("Переезд в новую локацию")
        .colour(serenity::colours::branding::BLURPLE);

    let player_idx = mining_user.location_index(locations);
    let mut buttons: Vec<serenity::CreateActionRow> = Vec::new();
    let mut current_row: Vec<serenity::CreateButton> = Vec::new();

    for (i, key) in locations.keys().enumerate() {
        let location = &locations[key];

        embed = embed.field(&location.name, &location.description, true);

        let mut button = serenity::CreateButton::new(format!("mining:locations:{}", key))
            .label(&location.name);

        if i <= player_idx {
            button = button.disabled(true);
        }

        if i == player_idx + 1 {
            button = button.style(serenity::ButtonStyle::Success)
        }

        current_row.push(button);

        if current_row.len() == 3 {
            buttons.push(serenity::CreateActionRow::Buttons(std::mem::take(&mut current_row)));
        }
    }

    if !current_row.is_empty() {
        buttons.push(serenity::CreateActionRow::Buttons(current_row));
    }

    buttons.push(serenity::CreateActionRow::Buttons(
        vec![
            serenity::CreateButton::new("mining:mm")
                .label("Назад")
                .style(serenity::ButtonStyle::Success)
        ]
    ));

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("")
            .embed(embed)
            .components(buttons)
    );

    Ok(())
}
