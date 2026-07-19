use crate::types::*;
use pretty_decimal::PrettyDecimal;

pub async fn handle_char_info_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    _data: &Data,
    dl_user: DlUser,
) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "Имя персонажа: {}\nКласс: {}\nБаланс: {}\nЗдоровье: {}\nМана: {}\nУрон: {}",
            dl_user.name,
            dl_user.display_class(),
            PrettyDecimal::comma3dot(dl_user.balance),
            dl_user.health,
            dl_user.mana,
            dl_user.damage
        ))
        .footer(serenity::CreateEmbedFooter::new("Дромляндия: Онлайн"))
        .colour(serenity::colours::branding::BLURPLE);

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::default()
            .content("")
            .embed(embed)
            .ephemeral(true)
    );

    Ok(())
}
