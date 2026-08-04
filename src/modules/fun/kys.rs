use crate::{helpers::resolve_data_path, modules::traits::get_user_traits, types::*};
use poise::CreateReply;
use rand::prelude::*;
use std::sync::OnceLock;

static KYS_LIST: OnceLock<Vec<String>> = OnceLock::new();
fn get_kys_list() -> &'static [String] {
    KYS_LIST.get_or_init(|| {
        let data = std::fs::read_to_string(resolve_data_path("src/modules/fun/kys.json")).unwrap();
        serde_json::from_str(&data).expect("Failed to parse kys.json")
    })
}
/// KEEP YOURSELF SAFE
#[poise::command(
    slash_command,
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn kys(ctx: Context<'_>) -> Result<(), Error> {
    let list = get_kys_list();
    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    // 🟡 phoenix: 0.1% шанс выжить и отнять 500 бебр у Смерти
    let user_traits = get_user_traits(&ctx.data().pool, ctx.author().id.get()).await?;
    if user_traits.contains(&"phoenix".to_string())
        && rand::random_bool(0.001)
    {
        let _ = sqlx::query("UPDATE sbp_users SET balance = balance + 500 WHERE id = $1")
            .bind::<i64>(ctx.author().id.into())
            .execute(&ctx.data().pool)
            .await;

        ctx.send(
            CreateReply::default()
                .content("Вы восстали из пепла прямо перед Смертью и отняли у неё 500 бебр! Феникс не умирает просто так.")
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    ctx.send(
        CreateReply::default()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("kys_btn")
                    .label("KYS")
                    .emoji('☠')
                    .style(serenity::ButtonStyle::Danger),
            ])])
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

pub async fn handle_kys_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let list = get_kys_list();

    let choice = {
        let mut rng = rand::rng();
        list.choose(&mut rng).unwrap()
    };

    // 🟡 phoenix: 0.1% шанс выжить и отнять 500 бебр у Смерти
    let user_traits = get_user_traits(&data.pool, interaction.user.id.get()).await?;
    if user_traits.contains(&"phoenix".to_string())
        && rand::random_bool(0.001)
    {
        let _ = sqlx::query("UPDATE sbp_users SET balance = balance + 500 WHERE id = $1")
            .bind::<i64>(interaction.user.id.into())
            .execute(&data.pool)
            .await;

        crate::create_edit_response!(
            ctx,
            interaction,
            serenity::CreateInteractionResponseMessage::default()
                .content("Вы восстали из пепла прямо перед Смертью и отняли у неё 500 бебр! Феникс не умирает просто так.")
                .components(Vec::new())
        );
        return Ok(());
    }

    crate::create_edit_response!(
        ctx,
        interaction,
        serenity::CreateInteractionResponseMessage::default()
            .content(format!("Вы {}. Поздравляю со смертью!", choice))
            .components(vec![serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new("kys_btn")
                    .label("KYS")
                    .emoji('☠')
                    .style(serenity::ButtonStyle::Danger),
            ])])
    );
    Ok(())
}
