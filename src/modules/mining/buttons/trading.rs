use crate::{modules::mining::exchange_rate::ExchangeRate, types::*};

pub async fn handle_trading_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    _data: &Data,
    mining_user: super::MiningUser<'_>,
) -> Result<(), Error> {
    let exchange_rate = ExchangeRate::get();

    let embed = serenity::CreateEmbed::new()
        .title("💱 Обменник UCS → Бебры")
        .field("Текущий курс", format!("1 UCS = {exchange_rate} бебр"), true)
        .field(
            "Ваш лимит сегодня",
            format!(
                "{}/{} бебр",
                mining_user.traded_today,
                mining_user.location.trading_limit
            ),
            true,
        );

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("")
            .embed(embed)
    );

    Ok(())
}
