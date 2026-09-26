use sqlx::Row;
use crate::types::*;
use std::time::Duration;

const QUERY: &str = "\
    WITH expired AS ( \
        DELETE FROM active_games \
        WHERE created_at < NOW() - INTERVAL '5 minutes' \
        RETURNING user_id, bet \
    ) \
    UPDATE sbp_users u \
    SET balance = u.balance + e.bet \
    FROM expired e \
    WHERE u.id = e.user_id \
    RETURNING e.user_id, e.bet;\
";

pub async fn run_mines_cleanup_task(pool: sqlx::PgPool) -> Result<(), Error> {
    log::info!("mines cleanup task started");

    let mut ticker = tokio::time::interval(Duration::from_secs(60));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;

        match sqlx::query(QUERY).fetch_all(&pool).await {
            Ok(refunded_rows) => {
                for row in refunded_rows {
                    let user_id: i64 = row.get("user_id");
                    let user_id = serenity::UserId::new(user_id as u64);
                    super::ACTIVE_GAMES.remove(&user_id);
                }
            }
            Err(err) => {
                log::error!("mines_cleanup_task error: {:?}", err);
            }
        }
    }
}
