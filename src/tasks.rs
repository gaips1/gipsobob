use super::types::*;
use crate::modules;

pub fn run_tasks(ctx: &serenity::Context, pool: &sqlx::PgPool) {
    tokio::spawn(modules::giveaways::restore_giveaways(
        ctx.clone(),
        pool.clone(),
    ));
    tokio::spawn(modules::giveaways::run_giveaway_poller(
        ctx.clone(),
        pool.clone(),
    ));
    tokio::spawn(modules::giveaways::run_daily_giveaway_scheduler(
        ctx.clone(),
        pool.clone(),
        843475272107163648,
    ));
    tokio::spawn(modules::quests::helpers::run_random_quests_adder(
        ctx.clone(),
        pool.clone(),
    ));
    tokio::spawn(modules::quests::helpers::run_expired_quests_poller(
        ctx.clone(),
        pool.clone(),
    ));
    tokio::spawn(modules::quests::helpers::run_old_quests_cleaner(
        pool.clone(),
    ));
    tokio::spawn(modules::traits::run_today_spins_reset_task(pool.clone()));
}
