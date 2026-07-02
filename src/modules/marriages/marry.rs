use crate::{buttons::handle_buttons, types::*};
use poise::serenity_prelude::{self as serenity, Mentionable};

/// Сделать предложение руки и сердца
#[poise::command(slash_command, ephemeral, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn marry(
    ctx: Context<'_>,
    #[description = "Кому предлагаете"] user: serenity::User
) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    if user.bot {
        ctx.say("Даже не пробуй").await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.say("Даже не пробуй").await?;
        return Ok(());
    }

    let is_author_marriaged: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM marriages WHERE user_id = $1 OR partner_id = $1)"
    )
        .bind::<i64>(ctx.author().id.into())
        .fetch_one(pool)
        .await?;

    if is_author_marriaged {
        ctx.say("Ты, чё, изменщик? KYS").await?;
        return Ok(());
    }

    let is_user_marriaged: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM marriages WHERE user_id = $1 OR partner_id = $1)"
    )
        .bind::<i64>(user.id.into())
        .fetch_one(pool)
        .await?;

    if is_user_marriaged {
        ctx.say("Пользователь уже в браке").await?;
        return Ok(());
    }

    let embed = serenity::CreateEmbed::new()
        .title("Предложение брака")
        .description(format!(
            "**{}** предлагает брак **{}**",
            ctx.author().display_name(),
            user.display_name()
        ))
        .colour(serenity::colours::branding::YELLOW);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(format!("{}:marry:yes", ctx.id())).label("Да").style(serenity::ButtonStyle::Success),
        serenity::CreateButton::new(format!("{}:marry:no", ctx.id())).label("Нет").style(serenity::ButtonStyle::Danger),
    ])];

    let msg = ctx.send(poise::CreateReply::default().embed(embed).components(buttons).ephemeral(false)).await?;

    handle_buttons(ctx, format!("{}:marry:", ctx.id()).as_str(), 300, 
        move |press, id| {
            let user = user.clone();
            async move {
                if press.user.id != user.id {
                    press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Тише будь")
                            .ephemeral(true)
                    )).await?;
                    return Ok(false);
                }

                let is_author_marriaged: bool = sqlx::query_scalar(
                    "SELECT EXISTS (SELECT 1 FROM marriages WHERE user_id = $1 OR partner_id = $1)"
                )
                    .bind::<i64>(ctx.author().id.into())
                    .fetch_one(pool)
                    .await?;

                if is_author_marriaged {
                    press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Ты, чё, изменщик? KYS")
                            .ephemeral(true)
                    )).await?;
                    return Ok(false);
                }

                let is_user_marriaged: bool = sqlx::query_scalar(
                    "SELECT EXISTS (SELECT 1 FROM marriages WHERE user_id = $1 OR partner_id = $1)"
                )
                    .bind::<i64>(user.id.into())
                    .fetch_one(pool)
                    .await?;

                if is_user_marriaged {
                    press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Пользователь уже в браке")
                            .ephemeral(true)
                    )).await?;
                    return Ok(false);
                }

                match id.as_str() {
                    "yes" => {
                        sqlx::query(
                            "INSERT INTO marriages (user_id, partner_id) VALUES ($1, $2)"
                        )
                            .bind::<i64>(ctx.author().id.into())
                            .bind::<i64>(user.id.into())
                            .execute(pool)
                            .await?;

                        press.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                            serenity::CreateInteractionResponseMessage::default()
                                .components(vec![])
                        )).await?;

                        press.create_followup(&ctx, 
                            serenity::CreateInteractionResponseFollowup::new()
                                .content(format!("**Поздравим молодожёнов!\n{} и {} теперь официально вместе!**", ctx.author().mention(), user.mention()))
                                .ephemeral(false)
                        ).await?;
                    }

                    "no" => {
                        press.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                            serenity::CreateInteractionResponseMessage::default()
                                .components(vec![])
                        )).await?;

                        press.create_followup(&ctx, 
                            serenity::CreateInteractionResponseFollowup::new()
                                .content(format!("**{}, вот чёрт, тебе отказал(а) {}**", ctx.author().mention(), user.mention()))
                                .ephemeral(false)
                        ).await?;
                    }

                    _ => {}
                }

                Ok(false)
            }
        }, move || {
            async move {
                let embed = serenity::CreateEmbed::new().title("Предложение брака").description("Предложение просрочено. Думайте быстрее!!!!!!");
                msg.edit(ctx, poise::CreateReply::default().embed(embed).components(vec![])).await?;
                Ok(())
            }
        }
    ).await?;

    Ok(())
}