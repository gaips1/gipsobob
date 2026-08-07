#[derive(Debug, PartialEq, Eq, serde::Deserialize, Clone)]
pub struct Videocard {
    name: String,
    earn_per_second: u16,
    price: u32,
    power: u32
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, Clone)]
pub struct Location {
    name: String,
    description: String,
    max_power: u32,
    price_per_kwh: u16,
    unlock_price: u32
}