use crate::buttons::handle_buttons;
use crate::types::*;
use poise::CreateReply;
use poise::serenity_prelude::Mentionable;
use rand::seq::IndexedRandom;

const GIFS: [&str; 5] = [
    "https://media.tenor.com/pn5xTq0WtqcAAAAC/anime-girl.gif",
    "https://media.tenor.com/9G1zsVIiV6UAAAAC/anime-bed.gif",
    "https://media.tenor.com/tdK59AzAWZgAAAAC/pokemon-anime.gif",
    "https://media.tenor.com/i7S2Taae5H8AAAAC/sex-anime.gif",
    "https://media.tenor.com/eq-B2_glw0sAAAAC/ver-anime.gif",
];

/// Предложить секс пользователю
#[poise::command(
    slash_command,
    context_menu_command = "Предложить секс",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn sex(
    ctx: Context<'_>,
    #[description = "Кому предлагается секс"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.send(
            CreateReply::default()
                .content("Ты чё ебать себя собираешься?")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let mut buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(format!("{}:sex:yes", ctx.id()))
            .label("Да")
            .style(serenity::ButtonStyle::Success),
        serenity::CreateButton::new(format!("{}:sex:no", ctx.id()))
            .label("Нет")
            .style(serenity::ButtonStyle::Danger),
    ])];

    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{}, {} предложил Вам секс, Вы согласны?",
            user.global_name.as_deref().unwrap_or_else(|| &user.name),
            ctx.author()
                .global_name
                .as_deref()
                .unwrap_or_else(|| &ctx.author().name)
        ))
        .colour(serenity::colours::branding::GREEN);

    let msg = ctx
        .send(
            CreateReply::default()
                .embed(embed)
                .components(buttons.clone()),
        )
        .await?;

    for row in &mut buttons {
        if let serenity::CreateActionRow::Buttons(btns) = row {
            for button in btns {
                *button = button.clone().disabled(true);
            }
        }
    }

    let press_user = user.clone();
    handle_buttons(
        ctx,
        format!("{}:sex:", ctx.id()).as_str(),
        300,
        move |press, relative_id| {
            let user = press_user.clone();
            let buttons = buttons.clone();

            async move {
                if press.user.id != user.id {
                    crate::create_response!(
                        ctx,
                        press,
                        serenity::CreateInteractionResponseMessage::default()
                            .content("Завидуй молча, это не тебе секс предлагали")
                            .ephemeral(true)
                    );
                    return Ok(false);
                }

                if relative_id == "yes" {
                    crate::create_edit_response!(
                        ctx,
                        press,
                        serenity::CreateInteractionResponseMessage::default().components(buttons)
                    );

                    let gif = {
                        let mut rng = rand::rng();
                        *GIFS.choose(&mut rng).unwrap()
                    };

                    let embed = serenity::CreateEmbed::default()
                        .title(format!(
                            "{} согласился на секс с {}",
                            user.global_name.as_deref().unwrap_or_else(|| &user.name),
                            ctx.author()
                                .global_name
                                .as_deref()
                                .unwrap_or_else(|| &ctx.author().name)
                        ))
                        .image(gif)
                        .colour(serenity::colours::branding::GREEN);

                    press
                        .create_followup(
                            &ctx,
                            serenity::CreateInteractionResponseFollowup::new().embed(embed),
                        )
                        .await?;
                } else if relative_id == "no" {
                    crate::create_edit_response!(
                        ctx,
                        press,
                        serenity::CreateInteractionResponseMessage::default().components(buttons)
                    );

                    press
                        .create_followup(
                            &ctx,
                            serenity::CreateInteractionResponseFollowup::new().content(format!(
                                "**{}, вот чёрт, тебе отказал {} :(**",
                                ctx.author().mention(),
                                user.mention()
                            )),
                        )
                        .await?;
                }
                Ok(true)
            }
        },
        move || async move {
            let embed = serenity::CreateEmbed::default()
                .title(format!(
                    "{} не успел согласиться на предложение секса от {}",
                    user.global_name.as_deref().unwrap_or_else(|| &user.name),
                    ctx.author()
                        .global_name
                        .as_deref()
                        .unwrap_or_else(|| &ctx.author().name)
                ))
                .colour(serenity::colours::branding::RED);

            msg.edit(
                ctx,
                CreateReply::default().components(Vec::new()).embed(embed),
            )
            .await?;
            Ok(())
        },
    )
    .await?;

    Ok(())
}
