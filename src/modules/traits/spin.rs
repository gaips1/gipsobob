use crate::{modules::traits::{UserTrait, main_menu::format_user_trait}, types::*};

pub async fn handle_traits_spin_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let all_traits = super::get_traits();

    let rows: Vec<(i16, Option<String>, Option<i16>)> = sqlx::query_as(
        "SELECT \
            u.unlocked_traits_slots, \
            ut.trait_id, \
            ut.slot_index \
        FROM users u \
        LEFT JOIN user_traits ut ON u.id = ut.user_id \
        WHERE u.id = $1 \
        ORDER BY ut.slot_index ASC",
    )
    .bind(press.user.id.get() as i64)
    .fetch_all(&data.pool)
    .await?;

    let user_unlocked_slots = rows.first().unwrap().0;
    let user_traits: Vec<UserTrait> = rows
        .into_iter()
        .filter_map(|r| Some(UserTrait { trait_id: r.1?, slot_index: r.2? }))
        .collect();

    let Some(slot) = press.data.custom_id.strip_prefix("traits:spin:") else {
        let buttons: Vec<_> = (0..user_unlocked_slots)
            .map(|slot| {
                let empty = UserTrait { trait_id: "empty".into(), slot_index: slot };
                let t = user_traits.iter().find(|t| t.slot_index == slot).unwrap_or(&empty);

                serenity::CreateButton::new(format!("traits:spin:{slot}"))
                    .label(format_user_trait(all_traits, t, false))
                    .style(serenity::ButtonStyle::Primary)
            })
            .collect();

        let embed = serenity::CreateEmbed::new()
            .title("Мутации")
            .description("Выберите слот, в который хотите вколоть мутацию.\n**Выбранный слот будет перезаписан**")
            .colour(serenity::colours::branding::BLURPLE);

        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![
                    serenity::CreateActionRow::Buttons(buttons),
                    serenity::CreateActionRow::Buttons(
                        vec![
                            serenity::CreateButton::new("traits:mm")
                                .label("Назад")
                                .style(serenity::ButtonStyle::Secondary)
                        ]
                    )
                ])
        );
        return Ok(());
    };
    let slot: u16 = slot.parse()?;

    if slot > user_unlocked_slots as u16 {
        crate::create_edit_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Кус, читерок")
                .embeds(Vec::new())
                .components(Vec::new())
        );
        return Ok(());
    }

    

    Ok(())
}