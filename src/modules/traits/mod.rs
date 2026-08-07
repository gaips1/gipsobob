use crate::{helpers::resolve_data_path, types::*};
use chrono::{Duration, Utc};
use sqlx::prelude::FromRow;
use std::collections::HashMap;
use std::sync::OnceLock;

mod collection;
pub mod main_menu;
mod spin;
mod upgrade;

#[derive(Debug, FromRow, Clone)]
pub struct UserTrait {
    trait_id: String,
    slot_index: i16,
}

static TRAITS: OnceLock<HashMap<String, String>> = OnceLock::new();
pub fn get_traits() -> &'static HashMap<String, String> {
    TRAITS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/traits/traits.json")).unwrap();

        let parsed: serde_json::Value =
            serde_json::from_str(&data).expect("failed to parse traits.json");

        let traits_json = parsed.get("traits").expect("not found 'traits' column");

        serde_json::from_value(traits_json.clone()).expect("failed to deserialize traits object")
    })
}

pub async fn run_today_spins_reset_task(pool: sqlx::PgPool) {
    log::info!("today spins reset task started");

    loop {
        let now = Utc::now();
        let next_midnight = (now + Duration::days(1))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        let sleep_dur = (next_midnight - now)
            .to_std()
            .unwrap_or(std::time::Duration::from_secs(1));

        tokio::time::sleep(sleep_dur).await;

        let result = sqlx::query("UPDATE traits_users SET spins_today = 0 WHERE spins_today != 0")
            .execute(&pool)
            .await;

        match result {
            Ok(r) => log::info!("spins reset, rows affected: {}", r.rows_affected()),
            Err(e) => log::error!("failed to reset spins_today: {:?}", e),
        }
    }
}

pub async fn get_user_traits(
    pool: &sqlx::PgPool,
    user_id: u64,
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>("SELECT trait_id FROM user_traits WHERE user_id = $1")
        .bind(user_id as i64)
        .fetch_all(pool)
        .await
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![main_menu::traits()]
}
