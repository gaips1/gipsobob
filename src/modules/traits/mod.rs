use sqlx::prelude::FromRow;
use crate::{helpers::resolve_data_path, types::*};
use std::collections::HashMap;
use std::sync::OnceLock;

pub mod main_menu;

#[derive(Debug, FromRow, Clone)]
pub struct UserTrait {
    trait_id: String,
    slot_index: i16
}

static TRAITS: OnceLock<HashMap<String, String>> = OnceLock::new();
pub fn get_traits() -> &'static HashMap<String, String> {
    TRAITS.get_or_init(|| {
        let data =
            std::fs::read_to_string(resolve_data_path("src/modules/traits/traits.json")).unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&data)
            .expect("failed to parse traits.json");

        let traits_json = parsed.get("traits")
            .expect("not found 'traits' column");

        serde_json::from_value(traits_json.clone())
            .expect("failed to deserialize traits object")
    })
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![main_menu::traits()]
}