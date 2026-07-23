use std::{collections::HashMap, sync::OnceLock};

use super::types::{Quest, Status, UserQuest};
use crate::helpers::resolve_data_path;
use crate::types::*;

static QUESTS: OnceLock<HashMap<String, Quest>> = OnceLock::new();
pub fn get_quests() -> &'static HashMap<String, Quest> {
    QUESTS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/quests/quests.json")).unwrap();
        let list: Vec<Quest> = serde_json::from_str(&data).expect("failed to parse quests.json");
        list.into_iter().map(|q| (q.id.clone(), q)).collect()
    })
}

pub async fn get_user_quests(
    pool: &sqlx::PgPool,
    user_id: u64,
    quest_status: Status,
) -> Result<Vec<UserQuest>, Error> {
    let user_quests: Vec<UserQuest> = sqlx::query_as(
        "SELECT quest_id, progress, ends_at, status \
        FROM user_quests \
        WHERE user_id = $1 \
        AND status = $2",
    )
    .bind(user_id as i64)
    .bind(quest_status)
    .fetch_all(pool)
    .await?;

    Ok(user_quests)
}

pub fn create_quests_embed(user_quests: &Vec<UserQuest>) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::new()
        .title("Квесты")
        .colour(serenity::colours::branding::BLURPLE);

    if user_quests.len() == 0 {
        embed = embed.description("У вас нет доступных квестов выбранного статуса");
        return embed;
    }

    for user_quest in user_quests {
        let quest = user_quest.to_quest().unwrap();
        embed = embed.field(
            &quest.name,
            format!(
                "{}\nВыполнено: {}/{}\nНаграда: {}\nИстекает: {}",
                &quest.description,
                user_quest.progess,
                quest.max_progess,
                quest.reward,
                if user_quest.ends_at.is_none() {
                    "Никогда".to_string()
                } else {
                    format!("<t:{}:R>", user_quest.ends_at.unwrap().timestamp())
                }
            ),
            false,
        )
    }

    embed
}

pub fn quests_select_menu(selected: Status) -> Vec<serenity::CreateActionRow> {
    let options = vec![
        serenity::CreateSelectMenuOption::new("⌛ Активные ⌛", "active")
            .default_selection(selected == Status::Active),
        serenity::CreateSelectMenuOption::new("✅ Выполненные ✅", "completed")
            .default_selection(selected == Status::Completed),
        serenity::CreateSelectMenuOption::new("❌ Просроченные ❌", "expired")
            .default_selection(selected == Status::Expired),
    ];

    let menu = serenity::CreateSelectMenu::new(
        "quest_status_select",
        serenity::CreateSelectMenuKind::String { options },
    )
    .placeholder("👉 Выбрать статус отображаемых квестов 👈")
    .min_values(1)
    .max_values(1);

    vec![serenity::CreateActionRow::SelectMenu(menu)]
}
