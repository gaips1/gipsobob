use indexmap::IndexMap;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use sqlx::types::Json;
use std::collections::HashMap;

use super::types::*;
use crate::types::*;

pub fn bar(current: f64, max: f64, width: usize) -> String {
    let filled = ((current / max) * width as f64).round() as usize;
    let filled = filled.min(width);
    format!("[{}{}]", "▰".repeat(filled), "▱".repeat(width - filled))
}

impl<'a> MiningUser<'a> {
    pub async fn get(pool: &sqlx::PgPool, user: &'a serenity::User) -> Option<Self> {
        let row: Option<(
            Decimal,
            String,
            Json<HashMap<String, u64>>,
            chrono::DateTime<chrono::Utc>,
        )> = sqlx::query_as(
            "SELECT balance, location, videocards, restarted_at FROM mining_users WHERE id = $1",
        )
        .bind(user.id.get() as i64)
        .fetch_optional(pool)
        .await
        .ok()?;

        let Some((balance, location, Json(user_videocards), restarted_at)) = row else {
            return None;
        };

        let all_videocards = super::get_videocards();
        let all_locations = super::get_locations();

        Some(MiningUser {
            balance: balance,
            location: all_locations
                .get(&location)
                .expect("unknown location in DB"),
            videocards: user_videocards
                .into_iter()
                .map(|(id, count)| {
                    (
                        all_videocards.get(&id).expect("unknown videocard in DB"),
                        count,
                    )
                })
                .collect(),
            restarted_at,
        })
    }

    pub fn videocards_power_sum(&self) -> u64 {
        self.videocards
            .iter()
            .map(|(card, &count)| card.power as u64 * count)
            .sum()
    }

    pub fn videocards_earn_per_second_sum(&self) -> Decimal {
        self.videocards
            .iter()
            .map(|(card, &count)| card.earn_per_second * Decimal::from(count))
            .sum()
    }

    pub fn earn_per_second(&self) -> Decimal {
        let watts = Decimal::from(self.videocards_power_sum());
        let kwh_per_second = watts / Decimal::from(1000) / Decimal::from(3600);
        let power_cost_per_second = kwh_per_second * self.location.price_per_kwh;

        self.videocards_earn_per_second_sum() - power_cost_per_second
    }
}

impl Location {
    pub fn index(&self, locations: &IndexMap<String, Location>) -> usize {
        locations
            .values()
            .position(|k| k.name == self.name)
            .unwrap_or(0)
    }
}
