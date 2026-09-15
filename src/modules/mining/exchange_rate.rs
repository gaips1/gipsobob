use crate::helpers::resolve_data_path;
use crate::types::*;
use rust_decimal::Decimal;
use std::fs;
use std::str::FromStr;
use std::sync::{OnceLock, RwLock};

const PATH: &str = "src/modules/mining/mining_exchange_rate";
static EXCHANGE_RATE: OnceLock<RwLock<Decimal>> = OnceLock::new();

pub struct ExchangeRate;
impl ExchangeRate {
    fn lock() -> &'static RwLock<Decimal> {
        EXCHANGE_RATE.get_or_init(|| {
            let file_content = fs::read_to_string(resolve_data_path(PATH))
                .expect("failed to read exchange rate file");
            let value = Decimal::from_str(&file_content).expect("failed to parse exchange rate");
            RwLock::new(value)
        })
    }

    pub fn get() -> Decimal {
        *Self::lock().read().unwrap_or_else(|e| e.into_inner())
    }

    pub fn set(value: Decimal) -> Result<(), Error> {
        fs::write(resolve_data_path(PATH), value.to_string())?;
        *Self::lock().write().unwrap_or_else(|e| e.into_inner()) = value;
        Ok(())
    }
}
