use crate::{helpers::resolve_data_path, types::*};
use indexmap::IndexMap;
use std::sync::OnceLock;

pub mod buttons;
pub mod exchange_rate;
pub mod helpers;
mod main_menu;
pub mod tasks;
mod types;

static VIDEOCARDS: OnceLock<IndexMap<String, types::Videocard>> = OnceLock::new();
pub fn get_videocards() -> &'static IndexMap<String, types::Videocard> {
    VIDEOCARDS.get_or_init(|| {
        #[derive(serde::Deserialize)]
        struct MiningConfig {
            videocards: IndexMap<String, types::Videocard>,
        }

        let path = resolve_data_path("src/modules/mining/mining.json");
        let data = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read mining.json: {e}"));

        let config: MiningConfig =
            serde_json::from_str(&data).expect("failed to parse videocards from mining.json");

        config.videocards
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

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![main_menu::mining()]
}
