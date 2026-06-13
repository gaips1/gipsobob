use poise::serenity_prelude as serenity;
use poise::serenity_prelude::futures::StreamExt;
use crate::types::*;
use std::future::Future;

/// Обработчик нажатий на одну кнопка по её айди.
pub async fn handle_button<Fut>(
    ctx: Context<'_>,
    button_id: &str,
    timeout_secs: u64,
    mut on_click: impl FnMut(serenity::ComponentInteraction) -> Fut
) -> Result<(), Error>
where
    Fut: Future<Output = Result<(), Error>>,
{
    let filter_id = button_id.to_string();
    let mut stream = serenity::collector::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .filter(move |press| press.data.custom_id == filter_id)
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .stream();

    while let Some(press) = stream.next().await {
        on_click(press).await?;
    }

    Ok(())
}

/// Обработчик нажатий на кнопки по их префиксу.
pub async fn handle_buttons<Fut>(
    ctx: Context<'_>,
    prefix_id: &str,
    timeout_secs: u64,
    mut on_click: impl FnMut(serenity::ComponentInteraction, String) -> Fut
) -> Result<(), Error>
where
    Fut: Future<Output = Result<(), Error>>,
{
    let filter_prefix = prefix_id.to_string();
    let mut stream = serenity::collector::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .filter(move |press| press.data.custom_id.starts_with(&filter_prefix))
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .stream();

    while let Some(press) = stream.next().await {
        let relative_id = press.data.custom_id
            .strip_prefix(prefix_id)
            .unwrap_or(&press.data.custom_id)
            .to_string();
        on_click(press, relative_id).await?;
    }

    Ok(())
}