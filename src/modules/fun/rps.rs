use poise::serenity_prelude as serenity;
// use poise::{CreateReply};
use rust_decimal::Decimal;
use crate::types::*;

/// Камень-ножницы-бумага
#[poise::command(slash_command, rename = "цуефа", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn rps(
    ctx: Context<'_>,
    #[description = "С кем играть"] user: serenity::User,
    #[description = "Ставка в бебрах"] amount: Option<u64>,
) -> Result<(), Error> {
    if user.bot {
        ctx.say("Бро").await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.say("Бро").await?;
        return Ok(());
    }

    if let Some(amount) = amount {
        let amount = match Decimal::try_from(amount) {
            Ok(d) => d.round_dp(2),
            Err(_) => {
                ctx.say("Указана некорректная сумма").await?;
                return Ok(());
            }
        };

        if amount.is_zero() || amount.is_sign_negative() {
            ctx.say("Пожалуйста, введите положительное или не нулевое число бебр").await?;
            return Ok(());
        }


    }

    Ok(())
}