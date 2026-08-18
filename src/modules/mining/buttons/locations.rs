use crate::{modules::dialogues::get_dialogue, types::*};
use num_format::{Locale, ToFormattedString};

pub async fn handle_locations_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let Some(location_id) = press.data.custom_id.strip_prefix("mining:locations:") else {
        all_locations(ctx, press, data, mining_user).await?;
        return Ok(());
    };

    location_info(ctx, press, data, mining_user, location_id).await?;

    Ok(())
}

async fn all_locations(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    _data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let locations = super::get_locations();

    let mut embed = serenity::CreateEmbed::new()
        .title("Переезд в новую локацию")
        .colour(serenity::colours::branding::BLURPLE);

    let player_idx = mining_user.location.index(locations);
    let mut buttons: Vec<serenity::CreateActionRow> = Vec::new();
    let mut current_row: Vec<serenity::CreateButton> = Vec::new();

    for (i, key) in locations.keys().enumerate() {
        let location = &locations[key];

        embed = embed.field(&location.name, &location.description, true);

        let mut button =
            serenity::CreateButton::new(format!("mining:locations:{}", key)).label(&location.name);

        if i <= player_idx {
            button = button.disabled(true);
        }

        if i == player_idx + 1 {
            button = button.style(serenity::ButtonStyle::Success)
        }

        current_row.push(button);

        if current_row.len() == 3 {
            buttons.push(serenity::CreateActionRow::Buttons(std::mem::take(
                &mut current_row,
            )));
        }
    }

    if !current_row.is_empty() {
        buttons.push(serenity::CreateActionRow::Buttons(current_row));
    }

    buttons.push(serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("mining:mm")
            .label("Назад")
            .style(serenity::ButtonStyle::Success),
    ]));

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

async fn location_info(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mining_user: super::MiningUser<'_>,
    location_parts: &str,
) -> Result<(), Error> {
    let locations = super::get_locations();

    let location_parts: Vec<&str> = location_parts.split(":").collect();
    let location_id = *location_parts.first().unwrap();

    let location = &locations[location_id];

    let Some(action) = location_parts.get(1) else {
        let formatted_price = &location.price.to_formatted_string(&Locale::ru);

        let embed = serenity::CreateEmbed::new()
            .title(&location.name)
            .colour(serenity::colours::branding::BLURPLE)
            .field("Описание", &location.description, false)
            .field(
                "⚡ Макс. Энергопотребление",
                format!(
                    "{} Ватт",
                    location.max_power.to_formatted_string(&Locale::ru)
                ),
                false,
            )
            .field("💰 Стоимость", format!("{} UCS", formatted_price), true)
            .field(
                "💰 Цена за киловатт",
                format!("{} UCS", location.price_per_kwh),
                true,
            );

        let buttons = vec![
            serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(format!("mining:locations:{}:buy", location_id))
                    .label(format!("Купить [{} UCS]", formatted_price))
                    .style(serenity::ButtonStyle::Success),
            ]),
            serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("mining:locations").label("Назад"),
            ]),
        ];

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(buttons)
        );

        return Ok(());
    };

    if *action == "buy" {
        buy_location(ctx, press, data, mining_user, location_id).await?;
    }

    Ok(())
}

async fn buy_location(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mining_user: super::MiningUser<'_>,
    location_id: &str,
) -> Result<(), Error> {
    let locations = super::get_locations();
    let location = &locations[location_id];

    if location.index(locations) <= mining_user.location.index(locations) {
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Кус")
                .embeds(Vec::new())
                .components(Vec::new())
        );
        return Ok(());
    };

    let mut tx = data.pool.begin().await?;

    let result = sqlx::query(
        "UPDATE mining_users \
        SET balance = balance - $2 \
        WHERE id = $1 AND balance >= $2",
    )
    .bind(press.user.id.get() as i64)
    .bind(location.price as i32)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        tx.rollback().await?;
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("У Вас не хватает УзбиКоинов!")
                .embeds(Vec::new())
                .components(vec![serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("mining:mm").label("В главное меню")
                ])])
        );

        return Ok(());
    }

    sqlx::query("UPDATE mining_users SET location = $2 WHERE id = $1")
        .bind(press.user.id.get() as i64)
        .bind(location_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content(format!(
                "Поздравляю с покупкой! Вы переехали в `{}`",
                location.name
            ))
            .embeds(Vec::new())
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("mining:mm").label("В главное меню")
            ])])
    );

    Ok(())
}
