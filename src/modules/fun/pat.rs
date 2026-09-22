use crate::types::*;
use image::{
    AnimationDecoder as _, Frame, RgbaImage,
    codecs::gif::{GifDecoder, GifEncoder, Repeat},
    imageops::{self, FilterType},
};
use std::{io::Cursor, sync::LazyLock};

const CANVAS_SIZE: u32 = 128;
const AVATAR_SIZE: u32 = 95;
const AVATAR_X: i64 = 40;
const AVATAR_Y: i64 = 45;

struct PetFrame {
    hand: RgbaImage,
    delay: image::Delay,
}
static PETPET_FRAMES: LazyLock<Vec<PetFrame>> = LazyLock::new(|| {
    let bytes = include_bytes!("petpet.gif");
    let decoder = GifDecoder::new(Cursor::new(bytes)).expect("Не удалось прочитать petpet.gif");

    decoder
        .into_frames()
        .collect_frames()
        .expect("Не удалось декодировать кадры")
        .into_iter()
        .map(|frame| {
            let delay = frame.delay();
            let hand = imageops::resize(
                frame.buffer(),
                CANVAS_SIZE,
                CANVAS_SIZE,
                FilterType::Nearest,
            );
            PetFrame { hand, delay }
        })
        .collect()
});

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
        ctx.send(
            poise::CreateReply::default()
                .content("Роботофил!")
                .ephemeral(true),
        )
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

    let avatar_url = user.face().replace("?size=1024", "?size=128");
    let response_bytes = reqwest::get(&avatar_url).await?.bytes().await?;

    let photo = image::load_from_memory(&response_bytes)?.to_rgba8();
    let resized_photo = imageops::resize(&photo, AVATAR_SIZE, AVATAR_SIZE, FilterType::Nearest);

    let gif_bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>, Error> {
        let mut new_frames = Vec::with_capacity(PETPET_FRAMES.len());
        let mut base_canvas = RgbaImage::new(CANVAS_SIZE, CANVAS_SIZE);
        imageops::overlay(&mut base_canvas, &resized_photo, AVATAR_X, AVATAR_Y);

        for pf in PETPET_FRAMES.iter() {
            let mut canvas = base_canvas.clone();
            imageops::overlay(&mut canvas, &pf.hand, 0, 0);
            new_frames.push(Frame::from_parts(canvas, 0, 0, pf.delay));
        }

        let mut gif_bytes = Vec::with_capacity(64 * 1024);
        {
            let mut encoder = GifEncoder::new_with_speed(Cursor::new(&mut gif_bytes), 30);
            encoder.set_repeat(Repeat::Infinite)?;
            encoder.encode_frames(new_frames.into_iter())?;
        }

        Ok(gif_bytes)
    })
    .await??;

    let attachment = serenity::CreateAttachment::bytes(gif_bytes, "petpet.gif");
    let embed = serenity::CreateEmbed::default()
        .title(format!(
            "{} погладил(а) {}",
            ctx.author().display_name(),
            user.display_name(),
        ))
        .image("attachment://petpet.gif")
        .colour(serenity::colours::branding::GREEN);

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .attachment(attachment),
    )
    .await?;
    Ok(())
}
