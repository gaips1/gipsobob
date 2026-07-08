use crate::types::*;
use poise::serenity_prelude::{self as serenity, Mentionable};

/// Просмотр информации о твоем гареме
#[poise::command(
    slash_command,
    ephemeral,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn my_harem(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    let harem_users: Vec<(i64, i64, chrono::DateTime<chrono::Utc>, i64)> = sqlx::query_as(
        "SELECT \
            all_users.id, \
            h.author_id, \
            h.created_at, \
            h.id \
        FROM users target_user \
        INNER JOIN harems h ON target_user.harem_id = h.id \
        INNER JOIN users all_users ON h.id = all_users.harem_id \
        WHERE target_user.id = $1 \
        LIMIT 30;",
    )
    .bind::<i64>(ctx.author().id.into())
    .fetch_all(pool)
    .await?;

    if harem_users.len() == 0 {
        ctx.say("Вы в данный момент не состоите в гареме").await?;
        return Ok(());
    };

    let harem_author_id = harem_users.first().unwrap().1 as u64;
    let harem_author = if harem_author_id != ctx.author().id.get() {
        let user = match serenity::UserId::new(harem_author_id).to_user(ctx).await {
            Ok(user) => user,

            Err(err) => {
                if let serenity::Error::Http(serenity::HttpError::UnsuccessfulRequest(err)) = &err {
                    if err.error.code == 10013 {
                        let _ = sqlx::query("DELETE FROM harems WHERE id = $1")
                            .bind(harem_users.first().unwrap().3)
                            .execute(pool)
                            .await?;
                        return Ok(());
                    }
                }
                return Err(err.into());
            }
        };
        user
    } else {
        ctx.author().to_owned()
    };

    let harem_users_text = if harem_users.len() > 1 {
        harem_users
            .iter()
            .filter(|h| h.0 != harem_users.first().unwrap().1)
            .map(|h| format!("<@{}>", h.0))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        "нету".to_owned()
    };

    let embed = serenity::CreateEmbed::new()
        .title(format!("Гарем {}", harem_author.display_name()))
        .description(format!(
            "Участники гарема (последние 30): {}\nГарем был создан <t:{}:R>",
            harem_users_text,
            harem_users.first().unwrap().2.timestamp()
        ))
        .colour(serenity::colours::branding::BLURPLE);

    let buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("harem:leave")
            .label("Покинуть гарем")
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

pub async fn handle_harem_leave_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let custom_id = press.data.custom_id.as_str();

    match custom_id {
        "harem:leave" => {
            let buttons = vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("harem:leave:yes")
                    .label("Да")
                    .style(serenity::ButtonStyle::Danger),
            ])];

            crate::create_response!(ctx, press, serenity::CreateInteractionResponseMessage::default()
                .content("Вы уверены, что хотите покинуть гарем?\n## !! Если вы были его владельцем, он будет удалён. !!")
                .components(buttons)
                .ephemeral(true)
            );
        }

        "harem:leave:yes" => {
            let user_harem: Option<(i64, i64)> = sqlx::query_as(
                "SELECT h.author_id, h.id \
                FROM harems h \
                INNER JOIN users ON h.id = users.harem_id \
                WHERE users.id = $1;",
            )
            .bind(press.user.id.get() as i64)
            .fetch_optional(&data.pool)
            .await?;

            let Some(user_harem) = user_harem else {
                crate::create_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы в данный момент не в гареме")
                        .ephemeral(true)
                );
                return Ok(());
            };

            if user_harem.0 as u64 == press.user.id.get() {
                sqlx::query("DELETE FROM harems WHERE id = $1")
                    .bind(user_harem.1)
                    .execute(&data.pool)
                    .await?;

                crate::create_edit_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы удалили свой гарем :(")
                        .components(Vec::new())
                        .ephemeral(true)
                );

                // надо бы сделать уведомления для каждого пользователя, который входил в гарем, но мне так лееньь...
            } else {
                sqlx::query("UPDATE users SET harem_id = null WHERE id = $1")
                    .bind(press.user.id.get() as i64)
                    .execute(&data.pool)
                    .await?;

                crate::create_edit_response!(
                    ctx,
                    press,
                    serenity::CreateInteractionResponseMessage::default()
                        .content("Вы покинули свой гарем :(")
                        .components(Vec::new())
                        .ephemeral(true)
                );

                let harem_author = serenity::UserId::new(user_harem.0 as u64)
                    .to_user(ctx)
                    .await;
                let Ok(harem_author) = harem_author else {
                    return Ok(());
                };
                let _ = harem_author
                    .dm(
                        ctx,
                        serenity::CreateMessage::new()
                            .content(format!("{} покинул ваш гарем", press.user.mention())),
                    )
                    .await;
            }
        }
        _ => {}
    }

    Ok(())
}
