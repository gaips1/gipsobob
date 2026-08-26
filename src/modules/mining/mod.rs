use crate::{helpers::resolve_data_path, types::*};
use indexmap::IndexMap;
use std::sync::OnceLock;

pub mod buttons;
pub mod helpers;
mod main_menu;
mod types;

static VIDEOCARDS: OnceLock<IndexMap<String, types::Videocard>> = OnceLock::new();
pub fn get_videocards() -> &'static IndexMap<String, types::Videocard> {
    VIDEOCARDS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/mining/mining.json")).unwrap();

        let parsed: serde_json::Value =
            serde_json::from_str(&data).expect("failed to parse mining.json");

        let json = parsed
            .get("videocards")
            .expect("not found 'videocards' column");

        serde_json::from_value(json.clone()).expect("failed to deserialize object")
    })
}

#[derive(serde::Deserialize)]
struct MiningConfig {
    locations: IndexMap<String, types::Location>,
}
static LOCATIONS: OnceLock<IndexMap<String, types::Location>> = OnceLock::new();
pub fn get_locations() -> &'static IndexMap<String, types::Location> {
    LOCATIONS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/mining/mining.json")).unwrap();
        let config: MiningConfig =
            serde_json::from_str(&data).expect("failed to parse mining.json");
        config.locations
    })
}

pub async fn run_mining_profit_poller(ctx: serenity::Context, pool: sqlx::PgPool) {
    log::info!("mining profit poller started");

    let mut ticker = tokio::time::interval(std::time::Duration::from_mins(1));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;
    }
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![main_menu::mining()]
}
