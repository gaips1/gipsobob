use rust_decimal::Decimal;
use sqlx::FromRow;

use crate::modules::dromland::display_class;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    pub pool: sqlx::Pool<sqlx::Postgres>
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct DlUser {
    pub id: i64,
    pub name: String,
    pub class: String,
    pub balance: Decimal,
    pub health: i32,
    pub mana: i32,
    pub damage: i32,
    pub in_game: bool
}

impl DlUser {
    pub fn display_class(&self) -> &str {
        display_class(self.class.as_str()).unwrap()
    }
}