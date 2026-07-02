use poise::serenity_prelude as serenity;
use poise::serenity_prelude::futures::StreamExt;
use crate::types::*;
use std::future::Future;
use std::pin::Pin;

/// Слушает нажатия на кнопку с конкретным `custom_id` в течение `timeout_secs` секунд.
///
/// При каждом нажатии вызывается `on_click`. Если он возвращает `Ok(true)`,
/// прослушивание немедленно прекращается (например, когда условие выполнено
/// и дальше ждать нечего). Если `Ok(false)` — коллектор продолжает ждать
/// следующих нажатий.
///
/// Если за отведённое время не пришло ни одного нажатия, удовлетворившего
/// `on_click` (т.е. коллектор завершился по таймауту), вызывается `on_timeout`.
///
/// # Параметры
/// - `ctx` — контекст команды.
/// - `button_id` — точный `custom_id` кнопки, нажатия на которую нужно слушать.
/// - `timeout_secs` — сколько секунд ждать нажатие, прежде чем сработает таймаут.
/// - `on_click` — вызывается при каждом нажатии на кнопку. Верните `Ok(true)`,
///   чтобы остановить прослушивание, `Ok(false)` — чтобы продолжить ждать.
/// - `on_timeout` — вызывается один раз, если время истекло, а `on_click`
///   так и не вернул `Ok(true)`.
///
/// # Возвращает
/// `Ok(true)`, если прослушивание завершилось из-за нажатия (т.е. `on_click`
/// вернул `true`), и `Ok(false)`, если завершилось по таймауту.
pub async fn handle_button<Fut, FutTimeout>(
    ctx: Context<'_>,
    button_id: &str,
    timeout_secs: u64,
    mut on_click: impl FnMut(serenity::ComponentInteraction) -> Fut,
    on_timeout: impl FnOnce() -> FutTimeout,
) -> Result<bool, Error>
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
    let mut active_click: Option<Pin<Box<Fut>>> = None;

    loop {
        tokio::select! {
            maybe_press = stream.next() => {
                if let Some(press) = maybe_press {
                    active_click = Some(Box::pin(on_click(press)));
                } else {
                    break;
                }
            }
            res = async {
                match &mut active_click {
                    Some(fut) => fut.as_mut().await,
                    None => std::future::pending().await,
                }
            } => {
                active_click = None;
                if res? {
                    is_broken = true;
                    break;
                }
            }
        }
    }

    if !is_broken {
        on_timeout().await?;
    }

    Ok(is_broken)
}

/// Слушает нажатия на все кнопки, `custom_id` которых начинается с `prefix`,
/// в течение `timeout_secs` секунд.
///
/// При каждом нажатии вызывается `on_click`, куда вторым аргументом передаётся
/// часть `custom_id` после `prefix` (удобно для различения нескольких кнопок
/// с общим префиксом, например `"rps:choice:rock"` → `"rock"`).
/// Если `on_click` возвращает `Ok(true)`, прослушивание немедленно
/// прекращается. Если `Ok(false)` — коллектор продолжает ждать следующих нажатий.
///
/// Если за отведённое время ни одно нажатие не привело к `Ok(true)`
/// (коллектор завершился по таймауту), вызывается `on_timeout`.
///
/// # Параметры
/// - `ctx` — контекст команды.
/// - `prefix` — префикс `custom_id`, по которому отбираются нажатия кнопок.
/// - `timeout_secs` — сколько секунд ждать нажатия, прежде чем сработает таймаут.
/// - `on_click` — вызывается при каждом подходящем нажатии; вторым аргументом
///   приходит `custom_id` без префикса. Верните `Ok(true)`, чтобы остановить
///   прослушивание, `Ok(false)` — чтобы продолжить ждать.
/// - `on_timeout` — вызывается один раз, если время истекло, а `on_click`
///   так и не вернул `Ok(true)`.
pub async fn handle_buttons<Fut, FutTimeout>(
    ctx: Context<'_>,
    prefix: &str,
    timeout_secs: u64,
    mut on_click: impl FnMut(serenity::ComponentInteraction, String) -> Fut,
    on_timeout: impl FnOnce() -> FutTimeout,
) -> Result<bool, Error>
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
    let mut active_click: Option<Pin<Box<Fut>>> = None;

    loop {
        tokio::select! {
            maybe_press = stream.next() => {
                if let Some(press) = maybe_press {
                    let relative_id = press.data.custom_id
                        .strip_prefix(prefix)
                        .unwrap_or(&press.data.custom_id)
                        .to_string();
                        
                    active_click = Some(Box::pin(on_click(press, relative_id)));
                } else {
                    break;
                }
            }
            res = async {
                match &mut active_click {
                    Some(fut) => fut.as_mut().await,
                    None => std::future::pending().await,
                }
            } => {
                active_click = None;
                if res? {
                    is_broken = true;
                    break;
                }
            }
        }
    }

    if !is_broken {
        on_timeout().await?;
    }

    Ok(is_broken)
}