use super::types::*;
use crate::types::*;
use num_format::{Locale, ToFormattedString};
use pretty_decimal::PrettyDecimal;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub async fn handle_shop_buttons(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let mut custom_id = press.data.custom_id.split(':');

    let Some(videocard_id) = custom_id.nth(2) else {
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("")
                .embeds(Vec::new())
                .components(videocards_buttons())
        );
        return Ok(());
    };

    match custom_id.next() {
        Some("buy") => {
            buy_videocard(ctx, press, data, mining_user, videocard_id).await?;
            return Ok(());
        }
        Some("sell") => {
            sell_videocard(ctx, press, data, mining_user, videocard_id).await?;
            return Ok(());
        }
        _ => {}
    }

    let Some((embed, buttons)) = videocard_info(videocard_id, &mining_user.videocards) else {
        return Ok(());
    };

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

fn videocards_buttons() -> Vec<serenity::CreateActionRow> {
    let videocards: Vec<(&String, &super::types::Videocard)> =
        super::get_videocards().iter().collect();
    let mut buttons: Vec<serenity::CreateActionRow> =
        Vec::with_capacity(videocards.len().div_ceil(5));

    for chunk in videocards.chunks(5) {
        let mut row: Vec<serenity::CreateButton> = Vec::with_capacity(5);
        for &(id, v) in chunk {
            row.push(serenity::CreateButton::new(format!("mining:shop:{id}")).label(&v.name));
        }
        buttons.push(serenity::CreateActionRow::Buttons(row));
    }

    buttons.push(serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("mining:mm")
            .label("Назад")
            .style(serenity::ButtonStyle::Danger),
    ]));

    buttons
}

fn videocard_info(
    videocard_id: &str,
    user_videocards: &HashMap<&Videocard, u64>,
) -> Option<(serenity::CreateEmbed, Vec<serenity::CreateActionRow>)> {
    let videocard = super::get_videocards().get(videocard_id)?;
    let user_count = user_videocards.get(videocard).copied().unwrap_or_default();
    let sell_price = videocard.price / 2;

    let embed = serenity::CreateEmbed::new()
        .title(&videocard.name)
        .field(
            "💰 Стоимость",
            format!("{} UCS", videocard.price.to_formatted_string(&Locale::ru)),
            true,
        )
        .field(
            "♻️ Продажа (Б/У)",
            format!("{} UCS (-50%)", sell_price.to_formatted_string(&Locale::ru)),
            true,
        )
        .field(
            "💰 Добыча UCS",
            format!(
                "{} в минуту",
                PrettyDecimal::comma3dot(videocard.earn_per_second * Decimal::from(60))
            ),
            false,
        )
        .field(
            "⚡ Энергопотребление",
            format!("{} Ватт", videocard.power.to_formatted_string(&Locale::ru)),
            true,
        )
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("mining:shop")
                .label(format!("Назад"))
                .style(serenity::ButtonStyle::Danger),
            serenity::CreateButton::new(format!("mining:shop:{videocard_id}:buy"))
                .label(format!("Купить [у вас {user_count} шт.]",))
                .style(serenity::ButtonStyle::Success),
        ]),
        serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(format!("mining:shop:{videocard_id}:sell"))
                .label(format!(
                    "Продать Б/У (+{} UCS)",
                    sell_price.to_formatted_string(&Locale::ru)
                ))
                .disabled(user_count == 0),
        ]),
    ];

    Some((embed, buttons))
}

async fn buy_videocard(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mut mining_user: super::MiningUser<'_>,
    videocard_id: &str,
) -> Result<(), Error> {
    let Some(videocard) = super::get_videocards().get(videocard_id) else {
        return Ok(());
    };

    let result = sqlx::query(
        r#"
        UPDATE mining_users
        SET balance = balance - $1,
            videocards = jsonb_set(
                COALESCE(videocards, '{}'::jsonb),
                ARRAY[$2],
                to_jsonb(COALESCE((videocards->>$2)::bigint, 0) + 1)
            )
        WHERE balance >= $1 AND id = $3
        "#,
    )
    .bind(videocard.price as i64)
    .bind(videocard_id)
    .bind(press.user.id.get() as i64)
    .execute(&data.pool)
    .await?;

    if result.rows_affected() == 0 {
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("❌ Недостаточно средств для покупки!")
                .ephemeral(true)
        );
        return Ok(());
    }

    *mining_user.videocards.entry(videocard).or_insert(0) += 1;

    let Some((embed, buttons)) = videocard_info(videocard_id, &mining_user.videocards) else {
        return Ok(());
    };

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

async fn sell_videocard(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    mut mining_user: super::MiningUser<'_>,
    videocard_id: &str,
) -> Result<(), Error> {
    let Some(videocard) = super::get_videocards().get(videocard_id) else {
        return Ok(());
    };

    let sell_price = (videocard.price / 2) as i64;

    let result = sqlx::query(
        r#"
        UPDATE mining_users
        SET balance = balance + $1,
            videocards = CASE
                WHEN COALESCE((videocards->>$2)::bigint, 0) <= 1 THEN videocards - $2
                ELSE jsonb_set(videocards, ARRAY[$2], to_jsonb((videocards->>$2)::bigint - 1))
            END
        WHERE COALESCE((videocards->>$2)::bigint, 0) > 0 AND id = $3
        "#,
    )
    .bind(sell_price)
    .bind(videocard_id)
    .bind(press.user.id.get() as i64)
    .execute(&data.pool)
    .await?;

    if result.rows_affected() == 0 {
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("❌ У вас нет этой видеокарты для продажи!")
                .ephemeral(true)
        );
        return Ok(());
    }

    if let Some(count) = mining_user.videocards.get_mut(videocard) {
        *count = count.saturating_sub(1);
        if *count == 0 {
            mining_user.videocards.remove(videocard);
        }
    }

    let Some((embed, buttons)) = videocard_info(videocard_id, &mining_user.videocards) else {
        return Ok(());
    };

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
