use crate::types::*;
use std::fmt::Write as _;

pub async fn handle_power_info_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let mut text = String::new();

    for (v, count) in &mining_user.videocards {
        if *count == 0 {
            continue;
        }
        write!(
            &mut text,
            "{}x {}: {} Вт\n",
            count,
            v.name,
            v.power as u64 * count
        )
        .unwrap();
    }

    write!(
        &mut text,
        "\n`Итого: {} Ватт`",
        mining_user.videocards_power_sum()
    )
    .unwrap();

    let embed = serenity::CreateEmbed::new()
        .title("⚡ Электричество")
        .description(text)
        .colour(serenity::colours::branding::BLURPLE);

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("")
            .embed(embed)
    );

    Ok(())
}
