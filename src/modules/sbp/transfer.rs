use poise::{serenity_prelude as serenity};
use poise::serenity_prelude::utils::CreateQuickModal;
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::Decimal;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QuerySelect};
use sea_orm::TransactionTrait;
use pretty_decimal::PrettyDecimal;

use crate::checks::sbp_check;
use crate::database::{sbp_users};
use crate::modules::sbp::USER_UNATHORIZED_ERROR;
use crate::types::*;

async fn transfer(
    ctx: Context<'_>,
    user: serenity::User,
    amount: f64,
    comment: Option<String>
) -> Result<(), Error> {
    let db = &ctx.data().db;

    let amount = match Decimal::try_from(amount) {
        Ok(d) => d.round_dp(2),
        Err(_) => {
            ctx.say("Указана некорректная сумма").await?;
            return Ok(());
        }
    };

    if user.bot {
        ctx.say("Нельзя переводить бебры боту, баранище!!!!!!!!!!!!!").await?;
        return Ok(());
    }

    if amount.is_zero() || amount.is_sign_negative() {
        ctx.say("Пожалуйста, введите положительное или не нулевое число бебр").await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.say("Нельзя переводить бебры самому себе").await?;
        return Ok(());
    }

    if let Some(c) = &comment {
        if c.len() > 50 {
            ctx.say("Длина комментария к переводу не может превышать 50 символов").await?;
            return Ok(());
        }
    }

    let author_balance: Decimal = sbp_users::Entity::find_by_id::<i64>(ctx.author().id.into())
        .select_only()
        .column(sbp_users::Column::Balance)
        .into_tuple()
        .one(db)
        .await?
        .unwrap();

    if author_balance < amount {
        ctx.say("У вас не хватает денег").await?;
        return Ok(());
    }

    let user_sbp: Option<(Decimal, bool)> = sbp_users::Entity::find_by_id::<i64>(user.id.into())
        .select_only()
        .columns([sbp_users::Column::Balance, sbp_users::Column::Notifications])
        .into_tuple()
        .one(db)
        .await?;

    let Some(user_sbp) = user_sbp else {
        ctx.say(USER_UNATHORIZED_ERROR).await?;
        return Ok(());
    };

    let author_id = ctx.author().id.into();
    let user_id = user.id.into();
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            let _ = sbp_users::ActiveModel {
                id: Set(author_id),
                balance: Set(author_balance - amount),
                ..Default::default()
            }.update(txn).await?;

            let _ = sbp_users::ActiveModel {
                id: Set(user_id),
                balance: Set(Decimal::from(user_sbp.0) + amount),
                ..Default::default()
            }.update(txn).await?;

            Ok(())
        })
    })
    .await?;

    let _ = ctx.say(format!(
        "Успешно перевёл {} бебр {}",
        PrettyDecimal::comma3dot(amount), 
        user.global_name.as_deref().unwrap_or_else(|| &user.name)
    )).await;

    if user_sbp.1 {
        let mut embed = serenity::CreateEmbed::default()
            .title(format!(
                "Получен перевод от {} суммой {} бебр.",
                ctx.author().global_name.as_deref().unwrap_or_else(|| &ctx.author().name),
                PrettyDecimal::comma3dot(amount)
            ))
            .colour(serenity::colours::branding::GREEN);

        if let Some(c) = comment {
            embed = embed.description(format!("Комментарий от отправителя: ```{c}```"));
        }

        let _ = user.dm(&ctx, serenity::CreateMessage::default().embed(embed)).await;
    }

    Ok(())
}

/// Отправить бебры пользователю с помощью СБП
#[poise::command(slash_command, ephemeral, rename = "transfer", check = "sbp_check", install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn transfer_slash_command(
    ctx: Context<'_>,
    #[description = "Кому переводить"] user: serenity::User,
    #[description = "Сумма в бебрах"] amount: f64,
    #[description = "Комментарий к переводу"] comment: Option<String>
) -> Result<(), Error> {
    transfer(ctx, user, amount, comment).await?;
    Ok(())
}

/// Отправить бебры пользователю с помощью СБП
#[poise::command(context_menu_command = "Перевод бебр", ephemeral, install_context = "User | Guild", interaction_context = "Guild | BotDm | PrivateChannel")]
pub async fn transfer_context_menu_command(
    ctx: Context<'_>,
    #[description = "Кому переводить"] user: serenity::User,
) -> Result<(), Error> {
    let poise::Context::Application(app_ctx) = ctx else {
        return Ok(());
    };

    let modal = CreateQuickModal::new(
        format!(
            "Перевод бебр пользователю {}",
            user.global_name.as_deref().unwrap_or_else(|| &user.name)
        ))
        .timeout(std::time::Duration::from_secs(300))
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Сумма перевода",
                ""
            )
        )
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Комментарий к переводу",
                ""
            ).max_length(50).required(false)
        );


    let response = app_ctx.interaction.quick_modal(ctx.serenity_context(), modal).await?;
    app_ctx.has_sent_initial_response.store(true, std::sync::atomic::Ordering::SeqCst);

    if let Some(response) = response {
        let Ok(amount) = response.inputs[0].parse::<f64>() else {
            response.interaction.create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::Acknowledge
            ).await?;
            ctx.say("Введите корректное число").await?;
            return Ok(());
        };

        let comment: Option<String> = Some(response.inputs[1].clone())
            .filter(|s| !s.is_empty());

        response.interaction.create_response(
            ctx.serenity_context(),
            serenity::CreateInteractionResponse::Acknowledge
        ).await?;
        transfer(ctx, user, amount, comment).await?;
    }

    Ok(())
}