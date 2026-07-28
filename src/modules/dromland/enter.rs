use crate::modules::dromland::game::get_main_menu_buttons;
use crate::types::*;
use rand::seq::IndexedRandom as _;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::{Instant, sleep};

use super::types::*;

const CACHE_TTL: Duration = Duration::from_secs(600);
static MONSTERS_CACHE: RwLock<Option<(Instant, Vec<DlMonster>)>> = RwLock::const_new(None);

async fn get_monsters(pool: &sqlx::PgPool) -> Result<Vec<DlMonster>, Error> {
    if let Some((updated_at, monsters)) = &*MONSTERS_CACHE.read().await {
        if updated_at.elapsed() < CACHE_TTL {
            return Ok(monsters.clone());
        }
    }

    let mut guard = MONSTERS_CACHE.write().await;
    if let Some((updated_at, monsters)) = &*guard {
        if updated_at.elapsed() < CACHE_TTL {
            return Ok(monsters.clone());
        }
    }

    let fresh = sqlx::query_as::<_, DlMonster>(
        "SELECT name, health, reward, damage, image_url FROM dl_monsters",
    )
    .fetch_all(pool)
    .await?;

    *guard = Some((Instant::now(), fresh.clone()));
    Ok(fresh)
}

struct InGameGuard {
    pool: sqlx::PgPool,
    user_id: i64,
    armed: bool,
}

impl InGameGuard {
    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for InGameGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let pool = self.pool.clone();
        let user_id = self.user_id;
        tokio::spawn(async move {
            let _ = sqlx::query("UPDATE dl_users SET in_game = false WHERE id = $1")
                .bind(user_id)
                .execute(&pool)
                .await;
        });
    }
}

pub async fn handle_enter_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
    dl_user: DlUser,
) -> Result<(), Error> {
    let monsters = get_monsters(&data.pool).await?;
    let user_id = press.user.id.get() as i64;

    sqlx::query("UPDATE dl_users SET in_game = true WHERE id = $1")
        .bind(user_id)
        .execute(&data.pool)
        .await?;

    let mut in_game_guard = InGameGuard {
        pool: data.pool.clone(),
        user_id,
        armed: true,
    };

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content("Вы вошли в лабиринт.")
            .embeds(Vec::new())
            .components(Vec::new())
    );

    sleep(Duration::from_millis(1_500)).await;

    let monster = {
        let mut rng = rand::rng();
        monsters.choose(&mut rng).ok_or("dl_monsters пуста")?
    };

    let embed = serenity::CreateEmbed::new()
        .title("Лабиринт")
        .description(format!("**Вы наткнулись на {}!**", monster.name))
        .image(&monster.image_url)
        .color(serenity::colours::branding::RED);

    press
        .edit_response(
            ctx,
            serenity::EditInteractionResponse::new()
                .content("")
                .embed(embed),
        )
        .await?;

    let mut user_health = dl_user.health;
    let mut monster_health = monster.health;

    let loose: Option<i32> = loop {
        monster_health -= dl_user.damage as i16;

        sleep(Duration::from_secs(2)).await;

        let embed = serenity::CreateEmbed::new()
            .title(&monster.name)
            .description("Вы атакуете...")
            .color(serenity::colours::branding::BLURPLE)
            .image(&monster.image_url);

        press
            .edit_response(ctx, serenity::EditInteractionResponse::new().embed(embed))
            .await?;

        sleep(Duration::from_secs(2)).await;

        if monster_health <= 0 {
            sqlx::query("UPDATE dl_users SET balance = balance + $1 WHERE id = $2")
                .bind(monster.reward)
                .bind(user_id)
                .execute(&data.pool)
                .await?;

            break None;
        } else {
            let embed = serenity::CreateEmbed::new()
                .title(&monster.name)
                .description(format!(
                    "**У {} осталось {} хп!**",
                    monster.name, monster_health
                ))
                .color(serenity::colours::branding::BLURPLE)
                .image(&monster.image_url);

            press
                .edit_response(ctx, serenity::EditInteractionResponse::new().embed(embed))
                .await?;
        }

        sleep(Duration::from_secs(2)).await;

        let embed = serenity::CreateEmbed::new()
            .title(&monster.name)
            .description(format!("**{} атакует!**", monster.name))
            .color(serenity::colours::branding::BLURPLE)
            .image(&monster.image_url);

        press
            .edit_response(ctx, serenity::EditInteractionResponse::new().embed(embed))
            .await?;

        sleep(Duration::from_secs(2)).await;

        user_health -= monster.damage as i32;

        if user_health <= 0 {
            let loose = rand::random_range(1..=100);
            sqlx::query("UPDATE dl_users SET balance = balance - $1 WHERE id = $2")
                .bind(loose)
                .bind(user_id)
                .execute(&data.pool)
                .await?;
            break Some(loose);
        } else {
            let embed = serenity::CreateEmbed::new()
                .title(&monster.name)
                .description(format!("**У Вас осталось {} хп!**", user_health))
                .color(serenity::colours::branding::BLURPLE)
                .image(&monster.image_url);

            press
                .edit_response(ctx, serenity::EditInteractionResponse::new().embed(embed))
                .await?;
        }
    };

    sqlx::query("UPDATE dl_users SET in_game = false WHERE id = $1")
        .bind(user_id)
        .execute(&data.pool)
        .await?;
    in_game_guard.disarm();

    if loose.is_none() {
        let _ = add_user_quest_progress(&data.pool, ctx, user_id as u64, "dromlyandia", None, None).await;

        let embed = serenity::CreateEmbed::new()
            .title(&monster.name)
            .description(format!(
                "**Ты победил! За победу тебе выдали `{}` монет!**",
                monster.reward
            ))
            .color(serenity::colours::branding::BLURPLE)
            .image("");

        press
            .edit_response(
                ctx,
                serenity::EditInteractionResponse::new()
                    .embed(embed)
                    .components(get_main_menu_buttons()),
            )
            .await?;
    } else {
        let embed = serenity::CreateEmbed::new()
            .title(&monster.name)
            .description(format!(
                "**Вы проиграли :(**\n{} спиздил у вас {} монет",
                monster.name,
                loose.unwrap()
            ))
            .color(serenity::colours::branding::RED)
            .image("");

        press
            .edit_response(
                ctx,
                serenity::EditInteractionResponse::new()
                    .embed(embed)
                    .components(get_main_menu_buttons()),
            )
            .await?;
    }

    Ok(())
}
