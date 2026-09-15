use crate::{
    modules::mining::{exchange_rate::ExchangeRate, get_videocards},
    types::*,
};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use tokio::time::{MissedTickBehavior, interval};
use std::{collections::HashMap, time::Duration};

const SECONDS_PER_MINUTE: Decimal = Decimal::from_parts(60, 0, 0, false, 0);

pub async fn run_mining_profit_task(pool: sqlx::PgPool) -> Result<(), Error> {
    log::info!("mining profit task started");

    let all_videocards = get_videocards();
    let locations = super::get_locations();

    let mut ticker = interval(Duration::from_secs(60));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    
    ticker.tick().await;

    loop {
        ticker.tick().await;

        let users: Vec<(i64, sqlx::types::Json<HashMap<String, u64>>, String)> =
            match sqlx::query_as(
                "SELECT id, videocards, location FROM mining_users \
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

        if users.is_empty() {
            continue;
        }

        let mut user_ids = Vec::with_capacity(users.len());
        let mut profits = Vec::with_capacity(users.len());

        'user_loop: for (user_id, sqlx::types::Json(user_cards), location_name) in users {
            let Some(location) = locations.get(&location_name) else {
                log::warn!("unknown location for user {user_id} [{location_name}]");
                continue;
            };

            let mut power_sum: u64 = 0;
            let mut profit_sum = Decimal::ZERO;

            for (name, count) in user_cards {
                let Some(vc) = all_videocards.get(&name) else {
                    log::warn!("unknown videocard for user {user_id} [{name}]");
                    continue 'user_loop;
                };

                let count_dec = Decimal::from(count);
                power_sum = power_sum.saturating_add(vc.power as u64 * count);
                profit_sum += (vc.earn_per_second * SECONDS_PER_MINUTE) * count_dec;
            }

            if power_sum > location.max_power as u64 {
                continue;
            }

            if !profit_sum.is_zero() {
                user_ids.push(user_id);
                profits.push(profit_sum);
            }
        }

        for (ids_chunk, profits_chunk) in user_ids
            .chunks(2000)
            .zip(profits.chunks(2000))
        {
            let res = sqlx::query(
                "UPDATE mining_users AS m \
                SET balance = m.balance + u.profit \
                FROM UNNEST($1::bigint[], $2::numeric[]) AS u(id, profit) \
                WHERE m.id = u.id",
            )
            .bind(ids_chunk)
            .bind(profits_chunk)
            .execute(&pool)
            .await;

            if let Err(e) = res {
                log::error!("failed to add balance to mining users: {e}");
            }
        }
    }
}

pub async fn run_mining_exchange_rate_randomizer_task() -> Result<(), Error> {
    log::info!("mining exchange rate randomizer task started");

    let mut ticker = tokio::time::interval(std::time::Duration::from_hours(6));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;
        let value = rand::random_range(0.008..0.012);
        let _ = ExchangeRate::set(Decimal::from_f64(value).unwrap().round_dp(3));
    }
}

pub async fn run_mining_trade_limit_reset_task(pool: sqlx::PgPool) -> Result<(), Error> {
    log::info!("mining trade limit reset task started");

    loop {
        let now = chrono::Local::now();

        let target_time = if !cfg!(debug_assertions) {
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        } else {
            (now + chrono::TimeDelta::seconds(60)).time()
        };

        let mut next_run = now.date_naive().and_time(target_time);
        if now.naive_local() >= next_run {
            next_run += chrono::Duration::days(1);
        }

        let duration_until = next_run - now.naive_local();
        let std_duration = duration_until
            .to_std()
            .unwrap_or(std::time::Duration::from_secs(0));

        tokio::time::sleep(std_duration).await;

        match sqlx::query("UPDATE mining_users SET traded_today = 0")
            .execute(&pool)
            .await
        {
            Ok(_) => {}
            Err(e) => log::error!("failed to reset traded_today in mining users: {e}"),
        }
    }
}
