use std::{collections::HashMap, sync::OnceLock};
use rand::seq::IndexedRandom as _;

use super::types::*;
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
                user_quest.progress,
                quest.max_progress,
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

pub async fn add_quest_to_user(
    pool: &sqlx::PgPool,
    user_id: u64,
    quest: &Quest
) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO user_quests (user_id, quest_id, ends_at) VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id as i64)
    .bind(&quest.id)
    .bind(
        if let Some(ends) = quest.ends {
            Some(chrono::Utc::now() + chrono::Duration::hours(ends as i64))
        } else {
            None
        }
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn run_random_quests_adder(
    ctx: serenity::Context,
    pool: sqlx::PgPool
) {
    let quests: Vec<&Quest> = get_quests().values().collect();
    log::info!("random quests adder started");

    loop {
        let now = chrono::Local::now();
        let target_time = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();

        let mut next_run = now.date_naive().and_time(target_time);
        if now.naive_local() >= next_run {
            next_run += chrono::Duration::days(1);
        }

        let duration_until = next_run - now.naive_local();
        let std_duration = duration_until.to_std().unwrap_or(std::time::Duration::from_secs(0));

        tokio::time::sleep(std_duration).await;

        let users: Vec<(i64, bool)> = sqlx::query_as("SELECT id, quest_notifications FROM users")
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

        let (user_ids, quest_ids, ends_ats, notify_ids) = {
            let mut rng = rand::rng();

            let mut user_ids = Vec::with_capacity(users.len());
            let mut quest_ids = Vec::with_capacity(users.len());
            let mut ends_ats = Vec::with_capacity(users.len());
            let mut notify_ids = Vec::new();

            for (id, notify) in &users {
                let quest = quests.choose(&mut rng).unwrap();

                user_ids.push(*id);
                quest_ids.push(quest.id.clone());
                ends_ats.push(quest.ends.map(|h| chrono::Utc::now() + chrono::Duration::hours(h as i64)));

                if *notify {
                    notify_ids.push(*id);
                }
            }

            (user_ids, quest_ids, ends_ats, notify_ids)
        };

        let _ = sqlx::query(
            "INSERT INTO user_quests (user_id, quest_id, ends_at)
            SELECT * FROM UNNEST($1::bigint[], $2::text[], $3::timestamptz[])"
        )
        .bind(&user_ids)
        .bind(&quest_ids)
        .bind(&ends_ats)
        .execute(&pool)
        .await;

        for uid in notify_ids {
            let embed = serenity::CreateEmbed::new()
                .title("Новый квест!")
                .description("Вам был добавлен новый квест\nПодробнее в `/квесты`")
                .color(serenity::colours::branding::GREEN);

            let user_id = serenity::UserId::new(uid as u64);
            let _ = user_id.dm(&ctx, serenity::CreateMessage::new().embed(embed)).await;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

pub async fn run_quests_poller(
    ctx: serenity::Context,
    pool: sqlx::PgPool
) {
    log::info!("quests poller started");

    let mut ticker = tokio::time::interval(std::time::Duration::from_mins(1));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;
    
        let expired_quests: Vec<(i64, String, bool)> = sqlx::query_as(
            "DELETE FROM user_quests uq \
            USING users u \
            WHERE uq.user_id = u.id \
            AND uq.ends_at IS NOT NULL \
            AND ends_at <= NOW() \
            RETURNING uq.user_id, uq.quest_id, u.quest_notifications"
        )
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

        for quest in expired_quests {
            let embed = serenity::CreateEmbed::new()
                .title(format!("Квест {} истёк!", get_quests().get(&quest.1).cloned().unwrap_or_default().name))
                .description("Увы, время выполнения квеста истекло")
                .color(serenity::colours::branding::RED);

            let user_id = serenity::UserId::new(quest.0 as u64);
            let _ = user_id.dm(&ctx, serenity::CreateMessage::new().embed(embed)).await;
        }
    }
}