use rand::seq::IndexedRandom as _;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::{collections::HashMap, sync::OnceLock};

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

pub fn get_notifications_button(value: bool) -> Vec<serenity::CreateActionRow> {
    vec![serenity::CreateActionRow::Buttons(vec![if value {
        serenity::CreateButton::new("quests:notifications:on")
            .label("Включить уведомления")
            .style(serenity::ButtonStyle::Success)
    } else {
        serenity::CreateButton::new("quests:notifications:off")
            .label("Выключить уведомления")
            .style(serenity::ButtonStyle::Danger)
    }])]
}

pub async fn get_user_quests(
    pool: &sqlx::PgPool,
    user_id: u64,
    quest_status: Status,
) -> Result<Vec<UserQuest>, Error> {
    let user_quests: Vec<UserQuest> = sqlx::query_as(
        "SELECT user_id, quest_id, progress, users, ends_at, status \
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

pub async fn add_user_quest_progress(
    pool: &sqlx::PgPool,
    ctx: &serenity::Context,
    user_id: u64,
    action: &str,
    target_user_id: Option<u64>,
    value: Option<i32>,
) -> Result<(), Error> {
    let quests: Vec<_> = get_quests()
        .values()
        .filter(|q| &q.action == action)
        .map(|q| q.id.clone())
        .collect();

    if quests.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;

    let quest_notifications: bool =
        sqlx::query_scalar("SELECT quest_notifications FROM users WHERE id = $1")
            .bind(user_id as i64)
            .fetch_one(&mut *tx)
            .await
            .unwrap_or(false);

    let active_quests: Vec<UserQuest> = sqlx::query_as(
        "SELECT user_id, quest_id, progress, users, ends_at, status \
        FROM user_quests \
        WHERE user_id = $1 \
        AND quest_id = ANY($2) \
        AND status = 'active' \
        FOR UPDATE",
    )
    .bind(user_id as i64)
    .bind(&quests)
    .fetch_all(&mut *tx)
    .await?;

    for active_quest in active_quests {
        let Some(q) = active_quest.to_quest() else {
            sqlx::query(
                "DELETE FROM user_quests WHERE quest_id = $1"
            )
            .bind(active_quest.quest_id)
            .execute(&mut *tx)
            .await?;
        
            continue;
        };
        let mut should_update = true;
        let mut new_users = active_quest.users.clone();

        if q.users_required_type == Some(UsersRequiredType::Different) {
            if let Some(target_id) = target_user_id {
                if active_quest.users.contains(&(target_id as i64)) {
                    should_update = false;
                } else {
                    new_users.push(target_id as i64);
                }
            } else {
                should_update = false;
            }
        } else {
            if let Some(target_id) = target_user_id {
                new_users.push(target_id as i64);
            }
        }

        if should_update {
            let new_progress = active_quest.progress + value.unwrap_or(1);
            let mut is_completed = false;
            if new_progress >= q.max_progress as i32 {
                is_completed = true;
            }

            if is_completed {
                sqlx::query(
                    "UPDATE user_quests \
                    SET progress = $1, users = $2, status = 'completed' \
                    WHERE user_id = $3 AND quest_id = $4 AND status = 'active'",
                )
                .bind(new_progress)
                .bind(new_users)
                .bind(user_id as i64)
                .bind(&active_quest.quest_id)
                .execute(&mut *tx)
                .await?;

                sqlx::query(
                    "UPDATE sbp_users \
                    SET balance = balance + $1 \
                    WHERE id = $2",
                )
                .bind(Decimal::from_u32(q.reward).unwrap())
                .bind(user_id as i64)
                .execute(&mut *tx)
                .await?;

                if quest_notifications {
                    let embed = serenity::CreateEmbed::new()
                        .title(format!("Квест {} выполнен!", q.name))
                        .description(format!(
                            "Вы успешно выполнили квест **{}**!\nНаграда: {} бебр",
                            q.name, q.reward
                        ))
                        .color(serenity::colours::branding::GREEN);

                    let dm_user_id = serenity::UserId::new(user_id);
                    let _ = dm_user_id
                        .dm(
                            ctx,
                            serenity::CreateMessage::new()
                                .embed(embed)
                                .components(get_notifications_button(false)),
                        )
                        .await;

                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                }
            } else {
                sqlx::query(
                    "UPDATE user_quests \
                    SET progress = $1, users = $2 \
                    WHERE user_id = $3 AND quest_id = $4 AND status = 'active'",
                )
                .bind(new_progress)
                .bind(new_users)
                .bind(user_id as i64)
                .bind(&active_quest.quest_id)
                .execute(&mut *tx)
                .await?;
            }
        }
    }

    tx.commit().await?;

    Ok(())
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
    quest: &Quest,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO user_quests (user_id, quest_id, ends_at) VALUES ($1, $2, $3)")
        .bind(user_id as i64)
        .bind(&quest.id)
        .bind(if let Some(ends) = quest.ends {
            Some(chrono::Utc::now() + chrono::Duration::hours(ends as i64))
        } else {
            None
        })
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn run_random_quests_adder(ctx: serenity::Context, pool: sqlx::PgPool) {
    let quests: Vec<&Quest> = get_quests()
        .values()
        .filter(|q| &q.id != "first_q")
        .collect();
    log::info!("random quests adder started");

    loop {
        let now = chrono::Local::now();

        let target_time = if !cfg!(debug_assertions) {
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        } else {
            (now + chrono::TimeDelta::seconds(10)).time()
        };

        let mut next_run = now.date_naive().and_time(target_time);
        if now.naive_local() >= next_run {
            next_run += chrono::Duration::days(1);
        }

        let duration_until = next_run - now.naive_local();
        let std_duration = duration_until
            .to_std()
            .unwrap_or(std::time::Duration::from_secs(0));

        tokio::time::sleep(std_duration).await;

        let users: Vec<(i64, bool, i64)> = sqlx::query_as(
            "SELECT u.id, u.quest_notifications, COALESCE(uq.cnt, 0)
            FROM users u
            LEFT JOIN (
                SELECT user_id, COUNT(*) AS cnt
                FROM user_quests
                WHERE status = 'active'
                GROUP BY user_id
            ) uq ON uq.user_id = u.id
            WHERE COALESCE(uq.cnt, 0) < 6",
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let (user_ids, quest_ids, ends_ats, notify_entries) = {
            let mut rng = rand::rng();

            let mut user_ids = Vec::with_capacity(users.len() * 2);
            let mut quest_ids = Vec::with_capacity(users.len() * 2);
            let mut ends_ats = Vec::with_capacity(users.len() * 2);
            let mut notify_entries: Vec<(i64, Vec<&Quest>)> = Vec::new();

            for (id, notify, cnt) in &users {
                let to_add = if *cnt <= 4 { 2 } else { 1 };

                let selected: Vec<&Quest> = quests
                    .sample(&mut rng, to_add)
                    .copied()
                    .collect();

                let mut user_notify_quests = Vec::new();

                for quest in selected {
                    user_ids.push(*id);
                    quest_ids.push(quest.id.clone());
                    ends_ats.push(
                        quest
                            .ends
                            .map(|h| chrono::Utc::now() + chrono::Duration::hours(h as i64)),
                    );

                    if *notify {
                        user_notify_quests.push(quest);
                    }
                }

                if *notify && !user_notify_quests.is_empty() {
                    notify_entries.push((*id, user_notify_quests));
                }
            }

            (user_ids, quest_ids, ends_ats, notify_entries)
        };

        match sqlx::query(
            "INSERT INTO user_quests (user_id, quest_id, ends_at)
            SELECT * FROM UNNEST($1::bigint[], $2::text[], $3::timestamptz[])",
        )
        .bind(&user_ids)
        .bind(&quest_ids)
        .bind(&ends_ats)
        .execute(&pool)
        .await
        {
            Ok(res) => log::info!("inserted {} quests", res.rows_affected()),
            Err(e) => log::error!("failed to insert quests: {e}"),
        };

        for (uid, quests) in notify_entries {
            let mut embed = serenity::CreateEmbed::new()
                .title(if quests.len() == 1 {
                    "Новый квест!"
                } else {
                    "Новые квесты!"
                })
                .color(serenity::colours::branding::GREEN);

            for quest in &quests {
                embed = embed.field("Новый квест:", &quest.name, false);
            }

            embed = embed.description("Подробнее в `/квесты`");

            let user_id = serenity::UserId::new(uid as u64);
            match user_id
                .dm(
                    &ctx,
                    serenity::CreateMessage::new()
                        .embed(embed)
                        .components(get_notifications_button(false)),
                )
                .await
            {
                Ok(_) => log::info!("dm sent to {uid}"),
                Err(e) => log::warn!("failed to dm {uid}: {e:?}"),
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

pub async fn run_expired_quests_poller(ctx: serenity::Context, pool: sqlx::PgPool) {
    log::info!("quests poller started");

    let mut ticker = tokio::time::interval(std::time::Duration::from_mins(1));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;

        let expired_quests: Vec<(i64, String, bool)> = sqlx::query_as(
            "UPDATE user_quests uq \
            SET status = 'expired' \
            FROM users u \
            WHERE uq.user_id = u.id \
            AND uq.ends_at IS NOT NULL \
            AND uq.ends_at <= NOW() \
            AND uq.status = 'active' \
            RETURNING uq.user_id, uq.quest_id, u.quest_notifications",
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        for quest in expired_quests.iter().filter(|q| q.2) {
            let embed = serenity::CreateEmbed::new()
                .title(format!(
                    "Квест {} истёк!",
                    get_quests().get(&quest.1).cloned().unwrap_or_default().name
                ))
                .description("Увы, время выполнения квеста истекло")
                .color(serenity::colours::branding::RED);

            let user_id = serenity::UserId::new(quest.0 as u64);
            let _ = user_id
                .dm(
                    &ctx,
                    serenity::CreateMessage::new()
                        .embed(embed)
                        .components(get_notifications_button(false)),
                )
                .await;

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }
    }
}

pub async fn run_old_quests_cleaner(pool: sqlx::PgPool) {
    log::info!("old quests cleaner started");

    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(60 * 60 * 24));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;

        let result = sqlx::query(
            "DELETE FROM user_quests \
            WHERE status IN ('expired', 'completed') \
            AND ends_at IS NOT NULL \
            AND ends_at <= NOW() - INTERVAL '7 days'",
        )
        .execute(&pool)
        .await;

        match result {
            Ok(r) => log::info!("old quests cleaner: deleted {} rows", r.rows_affected()),
            Err(e) => log::error!("old quests cleaner error: {:?}", e),
        }
    }
}
