use rust_decimal::Decimal;
use sqlx::prelude::FromRow;

use crate::modules::dromland::display_class;

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct DlUser {
    pub id: i64,
    pub name: String,
    pub class: String,
    pub balance: Decimal,
    pub health: i32,
    pub mana: i32,
    pub damage: i32,
    pub in_game: bool,
}

impl DlUser {
    pub fn display_class(&self) -> &str {
        display_class(self.class.as_str()).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct DlMonster {
    pub name: String,
    pub health: i16,
    pub reward: i16,
    pub damage: i16,
    pub image_url: String,
}
