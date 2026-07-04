use crate::{types::*};
use poise::serenity_prelude::{self as serenity};

/// Управление моим браком
#[poise::command(slash_command, ephemeral, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn my_marriage(ctx: Context<'_>,) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let row: Option<(i64, i64, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT user_id, partner_id, created_at FROM marriages WHERE user_id = $1 OR partner_id = $1"
    )
        .bind::<i64>(ctx.author().id.into())
        .fetch_optional(pool)
        .await?;

    let Some(row) = row else {
        ctx.say("Вы в данный момент не состоите в браке").await?;
        return Ok(());
    };

    let partner_id = if row.0 as u64 == ctx.author().id.get() { row.1 } else { row.0 };

    let embed = serenity::CreateEmbed::new()
        .title("Информация о браке")
        .description(format!(
            "**Партнёр: <@{}>\nВы вступили в брак** <t:{}:R>",
            partner_id, row.2.timestamp()
        ));

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("marriage:divorce").label("Развестись").style(serenity::ButtonStyle::Danger),
    ])]; 

    ctx.send(poise::CreateReply::default().embed(embed).components(buttons).ephemeral(true)).await?;

    Ok(())
}

pub async fn handle_divorce_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data
) -> Result<(), Error> {
    // TODO

    Ok(())
}