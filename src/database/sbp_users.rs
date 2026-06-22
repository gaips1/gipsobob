use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct SbpUser {
    pub id: i64,
    pub balance: Decimal,
    pub notifications: bool,
}