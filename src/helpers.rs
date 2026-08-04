use super::types::Error;
pub use poise::serenity_prelude as serenity;

#[macro_export]
macro_rules! create_response {
    ($ctx:expr, $interaction:expr, $message:expr) => {
        $interaction
            .create_response(
                $ctx,
                $crate::helpers::serenity::CreateInteractionResponse::Message($message),
            )
            .await?
    };
}

#[macro_export]
macro_rules! create_edit_response {
    ($ctx:expr, $interaction:expr, $message:expr) => {
        $interaction
            .create_response(
                $ctx,
                $crate::helpers::serenity::CreateInteractionResponse::UpdateMessage($message),
            )
            .await?
    };
}

pub fn resolve_data_path(relative: &str) -> std::path::PathBuf {
    if cfg!(debug_assertions) {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        std::path::Path::new(manifest_dir).join(relative)
    } else {
        let exe_dir = std::env::current_exe()
            .expect("failed to get current exe path")
            .parent()
            .unwrap()
            .join("bot_data")
            .to_path_buf();

        let file_name = std::path::Path::new(relative)
            .file_name()
            .expect("relative path has no file name");

        exe_dir.join(file_name)
    }
}

pub async fn check_user_flag(pool: &sqlx::PgPool, user_id: u64, flag: &str) -> Result<bool, Error> {
    let result: bool = sqlx::query_scalar("SELECT $2 = ANY(flags) FROM users WHERE id = $1")
        .bind(user_id as i64)
        .bind(flag)
        .fetch_optional(pool)
        .await?
        .unwrap_or(false);

    Ok(result)
}

pub async fn set_user_flag(pool: &sqlx::PgPool, user_id: u64, flag: &str) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    sqlx::query(
        "UPDATE users SET flags = array_append(flags, $2) \
        WHERE id = $1 AND NOT (flags @> ARRAY[$2]::text[])"
    )
    .bind(user_id as i64)
    .bind(flag)
    .execute(pool)
    .await
}
