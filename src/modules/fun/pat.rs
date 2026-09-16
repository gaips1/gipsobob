use std::{io::Cursor, sync::LazyLock};
use image::{AnimationDecoder as _, Frame, RgbaImage, codecs::gif::{GifDecoder, GifEncoder, Repeat}, imageops::{self, FilterType}};
use crate::types::*;

static PETPET_FRAMES: LazyLock<Vec<Frame>> = LazyLock::new(|| {
    let bytes = include_bytes!("petpet.gif");
    let cursor = Cursor::new(bytes);
    
    let decoder = GifDecoder::new(cursor)
        .expect("Не удалось прочитать petpet.gif");
        
    decoder
        .into_frames()
        .collect_frames()
        .expect("Не удалось декодировать кадры")
});

/// Погладить пользователя
#[poise::command(
    slash_command,
    rename = "погладить",
    context_menu_command = "Погладить",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn pat(
    ctx: Context<'_>,
    #[description = "Кого гладите"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(poise::CreateReply::default().content("Роботофил!").ephemeral(true))
            .await?;
        return Ok(());
    }

    if user.id == ctx.author().id {
        ctx.send(
            poise::CreateReply::default()
                .content("Ты чё гладить себя собираешься?")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    ctx.defer().await?;

    let _ = add_user_quest_progress(
        &ctx.data().pool,
        ctx.serenity_context(),
        ctx.author().id.get(),
        "pat",
        Some(user.id.get()),
        None,
    )
    .await;

    let response_bytes = reqwest::get(&user.face())
        .await?
        .bytes()
        .await?;

    let photo = image::load_from_memory(&response_bytes)?.to_rgba8();
    let resized_photo = imageops::resize(&photo, 190, 190, FilterType::Nearest);
    let mut new_frames = Vec::with_capacity(PETPET_FRAMES.len());

    for frame in PETPET_FRAMES.iter() {
        let mut canvas = RgbaImage::new(256, 256);
        imageops::overlay(&mut canvas, &resized_photo, 70, 80);

        let scaled_hand = imageops::resize(
            frame.buffer(),
            256,
            256,
            FilterType::Nearest,
        );

        imageops::overlay(&mut canvas, &scaled_hand, 0, 0);
        new_frames.push(Frame::from_parts(canvas, 0, 0, frame.delay()));
    }

    let mut gif_bytes = Vec::new();
    {
        let mut encoder = GifEncoder::new(Cursor::new(&mut gif_bytes));
        encoder.set_repeat(Repeat::Infinite)?;
        encoder.encode_frames(new_frames.into_iter())?; 
    }

    let attachment = serenity::CreateAttachment::bytes(gif_bytes, "petpet.gif");
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} погладил(а) {}",
            ctx.author().display_name(),
            user.display_name(),
        ))
        .image("attachment://petpet.gif")
        .colour(serenity::colours::branding::GREEN);

    ctx.send(poise::CreateReply::default().embed(embed).attachment(attachment)).await?;
    Ok(())
}
