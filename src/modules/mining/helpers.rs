use std::collections::HashMap;
use sqlx::types::Json;

use super::types::*;
use crate::types::*;

pub fn bar(current: f64, max: f64, width: usize) -> String {
    let filled = ((current / max) * width as f64).round() as usize;
    let filled = filled.min(width);
    format!("[{}{}]", "▰".repeat(filled), "▱".repeat(width - filled))
}

impl<'a> MiningUser<'a> {
    pub async fn get(pool: & sqlx::PgPool, user: &'a serenity::User) -> Option<Self> {
        let row: Option<(i64, String, Json<HashMap<String, u64>>)> =
            sqlx::query_as("SELECT balance, location, videocards FROM mining_users WHERE id = $1")
                .bind(user.id.get() as i64)
                .fetch_optional(pool)
                .await
                .ok()?;

        let Some((balance, location, Json(user_videocards))) = row else {
            return None;
        };

        let all_videocards = super::get_videocards();
        let all_locations = super::get_locations();

        Some(
            MiningUser {
                serenity_user: user,
                balance: balance as u64,
                location: all_locations.get(&location).expect("unknown location in DB"),
                videocards: user_videocards
                    .into_iter()
                    .map(|(id, count)| (all_videocards.get(&id).expect("unknown videocard in DB"), count))
                    .collect(),
            }
        )
    }
}