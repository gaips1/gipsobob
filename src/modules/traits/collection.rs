use crate::types::*;
use std::fmt::Write;

pub async fn handle_traits_collection_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let user_spinned_traits: Vec<String> = sqlx::query_scalar(
        "SELECT spinned_traits FROM traits_users WHERE id = $1"
    )
    .bind(press.user.id.get() as i64)
    .fetch_one(&data.pool)
    .await?;

    let mut text = String::from(
        "Добро пожаловать!\n\
        Редкости мутаций:\n\
        🟡 - **Легендарный**,\n\
        🔵 - **Редкий**,\n\
        🟢 - **Необычный**,\n\
        ⚪ - **Бесполезный** (ничего не даёт)\n\n\
        Мутации, которые вы однажды вкололи себе:\n"
    );

    let traits_map = super::get_traits();
    let traits_list = user_spinned_traits
        .iter()
        .filter_map(|t| traits_map.get(t))
        .map(|t| t.split(":").next().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let _ = write!(text, "{traits_list}");

    let embed = serenity::CreateEmbed::new()
        .title("Коллекция Ваших мутаций")
        .description(text)
        .colour(serenity::colours::branding::YELLOW);

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(vec![
                serenity::CreateActionRow::Buttons(vec![
                    serenity::CreateButton::new("traits:mm")
                        .label("Назад")
                        .style(serenity::ButtonStyle::Primary)
                ])
            ])
    );

    Ok(())
}