use rust_decimal::Decimal;
use tokio::time::Instant;

use crate::types::*;

#[derive(Debug, Clone)]
pub enum MinesAction {
    Tile(u8),
    Cashout,
}

#[derive(Debug, Clone)]
pub struct MinesGame {
    pub user_id: serenity::UserId,
    pub action: MinesAction,
    pub bet: Decimal,
    pub bombs: u8,
    pub seed: String,
    pub mask: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum MinesError {
    #[error("Некорректный префикс или формат custom_id")]
    InvalidFormat,
    #[error("Ошибка парсинга параметров: {0}")]
    ParseError(String),
    #[error("Подпись не совпадает (попытка подделки данных)")]
    InvalidSignature,
    #[error("Клетка {0} уже была открыта ранее")]
    AlreadyOpened(u8),
    #[error("Недопустимый индекс клетки: {0}")]
    InvalidTileIndex(u8),
    #[error("Недопустимое количество мин")]
    InvalidBombCount,
    #[error("Ставка должна быть больше нуля")]
    InvalidBet,
}

pub struct ActiveSession {
    pub current_win: Decimal,
    pub last_activity: Instant,
}
