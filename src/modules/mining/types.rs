use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, PartialEq, Eq, serde::Deserialize, Clone, Hash)]
pub struct Videocard {
    pub name: String,
    pub earn_per_second: u16,
    pub price: u32,
    pub power: u32,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub max_power: u32,
    pub price_per_kwh: u16,
    pub unlock_price: u32,
}

pub struct MiningUser<'a> {
    pub serenity_user: &'a serenity::User,
    pub balance: u64,
    pub location: &'a Location,
    pub videocards: HashMap<&'a Videocard, u64>,
}
