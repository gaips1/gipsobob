use crate::{buttons::handle_button, types::*};
use poise::serenity_prelude::{self as serenity};

/// Пригласить пользователей в свой гарем
#[poise::command(
    slash_command,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn invite_to_harem(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let author_harem_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM harems WHERE author_id = $1")
            .bind(ctx.author().id.get() as i64)
            .fetch_optional(pool)
            .await?;

    let Some(author_harem_id) = author_harem_id else {
        ctx.say("Вы в данный момент не находитесь в гареме или не являетесь его создателем")
            .await?;
        return Ok(());
    };

    let embed = serenity::CreateEmbed::new()
        .title(format!("Гарем {}", ctx.author().display_name()))
        .description("Нажмите на кнопку ниже, чтобы присоединиться к его гарему\n||Приглашение будет действительно 1 час||")
        .colour(serenity::colours::branding::BLURPLE);

    let btn_id = format!("{}:harem:invite", ctx.id());
    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(btn_id.clone()).label("Присоединиться"),
    ])];

    let msg = ctx
        .send(
            poise::CreateReply::default()
                .embed(embed.clone())
                .components(buttons)
                .ephemeral(false),
        )
        .await?;

    handle_button(
        ctx,
        &btn_id,
        3600,
        move |press| async move {
            let user_harem_id: Option<Option<i64>> =
                sqlx::query_scalar("SELECT harem_id FROM users WHERE id = $1")
                    .bind(press.user.id.get() as i64)
                    .fetch_optional(pool)
                    .await?;

            let Some(user_harem_id) = user_harem_id else {
                crate::create_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы не зарегистрированы в боте. Напишите любую команду")
                        .ephemeral(true)
                );
                return Ok(false);
            };

            if user_harem_id.is_some() {
                crate::create_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы в данный момент в чужом гареме")
                        .ephemeral(true)
                );
                return Ok(false);
            }

            sqlx::query("UPDATE users SET harem_id = $1 WHERE id = $2")
                .bind(author_harem_id)
                .bind(press.user.id.get() as i64)
                .execute(pool)
                .await?;

            crate::create_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::default()
                    .content("Вы успешно присоединились к гарему. Подробнее в `/my_harem`")
                    .ephemeral(true)
            );

            Ok(false)
        },
        move || async move {
            let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("meow")
                    .label("Приглашение истекло")
                    .disabled(true),
            ])];

            let _ = msg
                .edit(
                    ctx,
                    poise::CreateReply::default()
                        .embed(embed)
                        .components(buttons),
                )
                .await;

            Ok(())
        },
    )
    .await?;

    Ok(())
}
