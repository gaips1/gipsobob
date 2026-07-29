use crate::helpers::resolve_data_path;
use crate::types::*;
use poise::serenity_prelude::Mentionable;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
use std::sync::LazyLock;
use tokio::sync::Mutex;

static COUNTER_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

async fn get_counter_file() -> Result<tokio::fs::File, Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(resolve_data_path("src/modules/counter/counter.txt"))
        .await?;

    Ok(file)
}

pub async fn error(ctx: &serenity::Context, msg: &serenity::Message) -> Result<(), Error> {
    let mut file = get_counter_file().await?;
    file.write_u64(0).await?;

    let _ = msg.react(ctx, serenity::ReactionType::Unicode("❌".to_string())).await;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let _ = msg.reply_ping(ctx, format!("{} успешно заруинил! Счет вернулся к 0.", msg.author.mention())).await;

    Ok(())
}

pub async fn handle_counter_messages(
    ctx: &serenity::Context,
    msg: &serenity::Message
) -> Result<(), Error> {
    let Ok(num) = msg.content.parse::<u64>() else {
        error(ctx, msg).await?;
        return Ok(());
    };

    let _guard = COUNTER_LOCK.lock().await;

    let mut file = get_counter_file().await?;
    file.seek(SeekFrom::Start(0)).await?;

    let current_num = match file.read_u64().await {
        Ok(n) => n,
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => 0,
        Err(e) => return Err(e.into()),
    };
    
    if current_num + 1 != num {
        error(ctx, msg).await?;
        return Ok(());
    }

    file.seek(SeekFrom::Start(0)).await?;
    file.write_u64(num).await?;

    let _ = msg.react(ctx, serenity::ReactionType::Unicode("✅".to_string())).await;

    Ok(())
}