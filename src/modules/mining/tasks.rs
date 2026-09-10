use std::collections::HashMap;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use crate::{modules::mining::{exchange_rate::ExchangeRate, get_videocards, types::Videocard}, types::*};

pub async fn run_mining_profit_task(pool: sqlx::PgPool) -> Result<(), Error> {
    log::info!("mining profit task started");

    let all_videocards = get_videocards();
    let mut ticker = tokio::time::interval(std::time::Duration::from_mins(1));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;

        let users: Vec<(i64, sqlx::types::Json<HashMap<String, i64>>, Decimal)> =
            match sqlx::query_as(
                "SELECT id, videocards, balance FROM mining_users \
                WHERE NOW() - restarted_at < INTERVAL '1 day'",
            )
            .fetch_all(&pool)
            .await
            {
                Ok(rows) => rows,
                Err(e) => {
                    log::error!("mining profit poller: failed to fetch users: {e}");
                    continue;
                }
            };

        let users: Vec<(i64, HashMap<&Videocard, i64>, Decimal)> = users
            .iter()
            .filter_map(|u| {
                let mut videocards = HashMap::with_capacity(u.1.len());

                for (name, count) in u.1.iter() {
                    match all_videocards.get(name) {
                        Some(vc) => {
                            videocards.insert(vc, *count);
                        }
                        None => {
                            log::warn!("unknown videocard for user {} [{name}]", u.0);
                            return None;
                        }
                    }
                }

                Some((u.0, videocards, u.2))
            })
            .collect();

        if users.is_empty() {
            continue;
        }

        let (user_ids, balances) = {
            let mut user_ids = Vec::with_capacity(users.len() * 2);
            let mut balances: Vec<Decimal> = Vec::with_capacity(users.len() * 2);

            for (id, videocards, mut balance) in users {
                for (videocard, count) in videocards {
                    balance +=
                        (videocard.earn_per_second * Decimal::from(60)) * Decimal::from(count);
                }

                user_ids.push(id);
                balances.push(balance);
            }

            (user_ids, balances)
        };

        match sqlx::query(
            "UPDATE mining_users AS m \
            SET balance = u.balance \
            FROM UNNEST($1::bigint[], $2::numeric[]) AS u(id, balance) \
            WHERE m.id = u.id",
        )
        .bind(&user_ids)
        .bind(&balances)
        .execute(&pool)
        .await
        {
            Ok(_) => {}
            Err(e) => log::error!("failed to add balance to mining users: {e}"),
        };
    }
}

pub async fn run_exchange_rate_randomizer_task() -> Result<(), Error> {
    log::info!("mining exchange rate randomizer task started");

    let mut ticker = tokio::time::interval(std::time::Duration::from_hours(6));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        let value = rand::random_range(0.008..0.012);
        let _ = ExchangeRate::set(Decimal::from_f64(value).unwrap());
        ticker.tick().await;
    }
}