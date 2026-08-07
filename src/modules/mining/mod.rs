use std::{collections::HashMap, sync::OnceLock};
use crate::{helpers::resolve_data_path, types::*};

mod types;

static VIDEOCARDS: OnceLock<HashMap<String, types::Videocard>> = OnceLock::new();
pub fn get_videocards() -> &'static HashMap<String, types::Videocard> {
    VIDEOCARDS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/mining/mining.json")).unwrap();

        let parsed: serde_json::Value =
            serde_json::from_str(&data).expect("failed to parse mining.json");

        let json = parsed.get("videocards").expect("not found 'videocards' column");

        serde_json::from_value(json.clone()).expect("failed to deserialize object")
    })
}

static LOCATIONS: OnceLock<HashMap<String, types::Location>> = OnceLock::new();
pub fn get_locations() -> &'static HashMap<String, types::Location> {
    LOCATIONS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/mining/mining.json")).unwrap();

        let parsed: serde_json::Value =
            serde_json::from_str(&data).expect("failed to parse mining.json");

        let json = parsed.get("locations").expect("not found 'locations' column");

        serde_json::from_value(json.clone()).expect("failed to deserialize object")
    })
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![]
}
