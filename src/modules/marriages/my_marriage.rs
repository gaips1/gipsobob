use crate::types::*;
use poise::serenity_prelude::Mentionable;

/// Управление моим браком
#[poise::command(
    slash_command,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn my_marriage(ctx: Context<'_>) -> Result<(), Error> {
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

    let partner_id = if row.0 as u64 == ctx.author().id.get() {
        row.1
    } else {
        row.0
    };

    let embed = serenity::CreateEmbed::new()
        .title("Информация о браке")
        .description(format!(
            "**Партнёр: <@{}>\nВы вступили в брак** <t:{}:R>",
            partner_id,
            row.2.timestamp()
        ))
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("marriage:divorce")
            .label("Развестись")
            .style(serenity::ButtonStyle::Danger),
    ])];

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .components(buttons)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

pub async fn handle_divorce_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    match interaction.data.custom_id.as_str() {
        "marriage:divorce" => {
            let embed = serenity::CreateEmbed::new()
                .title("Развод")
                .description("Вы уверены, что хотите развестись?")
                .colour(serenity::colours::branding::RED);

            let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("marriage:divorce:yes")
                    .label("Да")
                    .style(serenity::ButtonStyle::Danger),
            ])];

            crate::create_response!(
                ctx,
                interaction,
                serenity::CreateInteractionResponseMessage::default()
                    .embed(embed)
                    .components(buttons)
                    .ephemeral(true)
            );
        }

        "marriage:divorce:yes" => {
            let row: Option<(i64, i64)> = sqlx::query_as(
                "SELECT user_id, partner_id FROM marriages WHERE user_id = $1 OR partner_id = $1",
            )
            .bind::<i64>(interaction.user.id.into())
            .fetch_optional(&data.pool)
            .await?;

            let Some(row) = row else {
                crate::create_edit_response!(
                    ctx,
                    interaction,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы в данный момент не в браке")
                        .embeds(Vec::new())
                        .components(Vec::new())
                        .ephemeral(true)
                );
                return Ok(());
            };

            sqlx::query("DELETE FROM marriages WHERE user_id = $1 OR partner_id = $1")
                .bind::<i64>(interaction.user.id.into())
                .execute(&data.pool)
                .await?;

            crate::create_edit_response!(
                ctx,
                interaction,
                serenity::CreateInteractionResponseMessage::default()
                    .content("Вы успешно развелись :(")
                    .embeds(Vec::new())
                    .components(Vec::new())
            );

            let partner_id = if row.0 as u64 == interaction.user.id.get() {
                row.1
            } else {
                row.0
            };
            let partner = serenity::UserId::new(partner_id as u64).to_user(ctx).await;
            let Ok(partner) = partner else { return Ok(()) };
            let _ = partner
                .dm(
                    ctx,
                    serenity::CreateMessage::new().content(format!(
                        "{} развёлся с вами! :(",
                        interaction.user.mention()
                    )),
                )
                .await;
        }
        _ => {}
    }

    Ok(())
}
