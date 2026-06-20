use poise::serenity_prelude as serenity;
use poise::serenity_prelude::futures::StreamExt;
use crate::types::*;
use std::future::Future;

/// Обработчик нажатий на одну кнопку по её айди.
/// В on_click вернуть false, чтобы продолжить обработку нажатий,
/// и true, чтобы прекратить.
/// Если время ожидания истекло, вызывается on_timeout.
pub async fn handle_button<Fut, FutTimeout>(
    ctx: Context<'_>,
    button_id: &str,
    timeout_secs: u64,
    mut on_click: impl FnMut(serenity::ComponentInteraction) -> Fut,
    on_timeout: impl FnOnce() -> FutTimeout,
) -> Result<(), Error>
where
    Fut: Future<Output = Result<bool, Error>>,
    FutTimeout: Future<Output = Result<(), Error>>,
{
    let filter_id = button_id.to_string();
    let mut stream = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id == filter_id)
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .stream();

    let mut is_broken = false;
    while let Some(press) = stream.next().await {
        let break_updates = on_click(press).await?;
        if break_updates {
            is_broken = true;
            break;
        }
    }

    if !is_broken {
        on_timeout().await?;
    }

    Ok(())
}

/// Обработчик нажатий на кнопки по их префиксу.
/// В on_click вернуть false, чтобы продолжить обработку нажатий,
/// и true, чтобы прекратить.
/// Если время ожидания истекло, вызывается on_timeout.
pub async fn handle_buttons<Fut, FutTimeout>(
    ctx: Context<'_>,
    prefix: &str,
    timeout_secs: u64,
    mut on_click: impl FnMut(serenity::ComponentInteraction, String) -> Fut,
    on_timeout: impl FnOnce() -> FutTimeout,
) -> Result<(), Error>
where
    Fut: Future<Output = Result<bool, Error>>,
    FutTimeout: Future<Output = Result<(), Error>>,
{
    let filter_prefix = prefix.to_string();
    let mut stream = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&filter_prefix))
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .stream();

    let mut is_broken = false;
    while let Some(press) = stream.next().await {
        let relative_id = press.data.custom_id
            .strip_prefix(prefix)
            .unwrap_or(&press.data.custom_id)
            .to_string();

        let break_updates = on_click(press, relative_id).await?;
        if break_updates {
            is_broken = true;
            break;
        }
    }

    if !is_broken {
        on_timeout().await?;
    }

    Ok(())
}