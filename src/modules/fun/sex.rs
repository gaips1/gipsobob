use poise::serenity_prelude::{self as serenity, Mentionable};
use poise::{CreateReply};
use rand::seq::IndexedRandom;
use crate::buttons::{handle_buttons};
use crate::types::*;

const GIFS: [&str; 5] = [
    "https://media.tenor.com/pn5xTq0WtqcAAAAC/anime-girl.gif",
    "https://media.tenor.com/9G1zsVIiV6UAAAAC/anime-bed.gif",
    "https://media.tenor.com/tdK59AzAWZgAAAAC/pokemon-anime.gif",
    "https://media.tenor.com/i7S2Taae5H8AAAAC/sex-anime.gif",
    "https://media.tenor.com/eq-B2_glw0sAAAAC/ver-anime.gif"
];

/// Предложить секс пользователю
#[poise::command(slash_command, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn sex(
    ctx: Context<'_>,
    #[description = "Кому предлагается секс"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true)).await?;
        return Ok(());
    }

    // if user.id == ctx.author().id {
    //     ctx.send(CreateReply::default().content("Ты чё ебать себя собираешься?").ephemeral(true)).await?;
    //     return Ok(());
    // }

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(format!("{}:sex:yes", ctx.id()))
            .label("Да")
            .style(serenity::ButtonStyle::Success),

        serenity::CreateButton::new(format!("{}:sex:no", ctx.id()))
            .label("Нет")
            .style(serenity::ButtonStyle::Danger)
    ])];

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{}, {} предложил Вам секс, Вы согласны?",
            user.global_name.as_deref().unwrap_or_else(|| &user.name),
            ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name)
        ))
        .colour(serenity::colours::branding::GREEN);

    ctx.send(
        CreateReply::default()
            .embed(embed)
            .components(buttons.clone())
    ).await?;

    handle_buttons(ctx, format!("{}:sex:", ctx.id()).as_str(), 300, move |press, relative_id| {
        let user = user.clone();
        let mut buttons = buttons.clone();

        async move {
            if press.user.id != user.id {
                press.create_response(&ctx, serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Завидуй молча, это не тебе секс предлагали")
                        .ephemeral(true)
                )).await?;
                return Ok(false);
            }

            for row in &mut buttons {
                if let serenity::CreateActionRow::Buttons(btns) = row {
                    for button in btns {
                        *button = button.clone().disabled(true);
                    }
                }
            }

            if relative_id == "yes" {
                press.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::default()
                        .components(buttons)
                )).await?;

                let gif = {
                    let mut rng = rand::rng();
                    *GIFS.choose(&mut rng).unwrap()
                };

                let embed = serenity::CreateEmbed::default()
                    .title(format!(
                        "{} согласился на секс с {}",
                        user.global_name.as_deref().unwrap_or_else(|| &user.name),
                        ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name)
                    ))
                    .image(gif)
                    .colour(serenity::colours::branding::GREEN);

                press.create_followup(&ctx,
                    serenity::CreateInteractionResponseFollowup::new()
                        .embed(embed)
                ).await?;

            } else if relative_id == "no" {
                press.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::default()
                        .components(buttons)
                )).await?;

                press.create_followup(&ctx,
                    serenity::CreateInteractionResponseFollowup::new()
                        .content(format!("**{}, вот чёрт, тебе отказал {} :(**", ctx.author().mention(), user.mention()))
                ).await?;
            }
            Ok(true)
        }
    }).await?;

    Ok(())
}