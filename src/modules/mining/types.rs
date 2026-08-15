use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Videocard {
    pub name: String,
    pub earn_per_second: f64,
    pub price: u32,
    pub power: u32,
}

impl PartialEq for Videocard {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Videocard {}
impl std::hash::Hash for Videocard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub max_power: u32,
    pub price_per_kwh: f64,
    pub unlock_price: u32,
}

pub struct MiningUser<'a> {
    pub serenity_user: &'a serenity::User,
    pub balance: u64,
    pub location: &'a Location,
    pub videocards: HashMap<&'a Videocard, u64>,
}
