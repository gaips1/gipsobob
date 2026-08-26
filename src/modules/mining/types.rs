use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Videocard {
    pub name: String,
    pub earn_per_second: Decimal,
    pub price: u32,
    pub power: u32,
}

#[derive(Debug, PartialEq, serde::Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub max_power: u32,
    pub price_per_kwh: Decimal,
    pub price: u32,
}

pub struct MiningUser<'a> {
    pub balance: Decimal,
    pub location: &'a Location,
    pub videocards: HashMap<&'a Videocard, u64>,
    pub restarted_at: chrono::DateTime<chrono::Utc>,
}
