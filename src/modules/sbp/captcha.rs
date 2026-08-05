use ab_glyph::PxScale;
use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use poise::CreateReply;
use rand::RngExt;
use rand::distr::Alphanumeric;
use std::io::Cursor;

use crate::buttons::handle_button;
use crate::checks::sbp_check;
use crate::types::*;

const CAPTCHA_LEN: usize = 10;
const IMG_WIDTH: u32 = 200;
const IMG_HEIGHT: u32 = 50;

/// Пройти капчу и получить бебры
#[poise::command(
    slash_command,
    rename = "капча",
    check = "sbp_check",
    install_context = "User | Guild",
    interaction_context = "Guild | BotDm | PrivateChannel"
)]
pub async fn captcha(ctx: Context<'_>) -> Result<(), Error> {
    let pool = &ctx.data().pool;

    ctx.defer_ephemeral().await?;

    let image_text: String = {
        let mut rng = rand::rng();
        (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(CAPTCHA_LEN)
            .map(char::from)
            .collect()
    };

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(IMG_WIDTH, IMG_HEIGHT, Rgb([255u8, 255, 255]));

    let font_data = include_bytes!("../../../arial.ttf") as &[u8];
    let font = ab_glyph::FontArc::try_from_slice(font_data).unwrap();
    let scale = PxScale { x: 28.0, y: 28.0 };

    draw_text_mut(
        &mut image,
        Rgb([0u8, 0, 0]),
        10,
        10,
        scale,
        &font,
        &image_text,
    );

    let mut buf = Cursor::new(Vec::new());
    image
        .write_to(&mut buf, image::ImageFormat::Avif)
        .expect("image coding error");
    let image_bytes = buf.into_inner();

    let button_id = format!("{}:captcha", ctx.id());
    let mut buttons = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(button_id.clone()).label("Ввести капчу"),
    ])];

    let msg = ctx
        .send(
            CreateReply::default()
                .content("Привет!\nТвоя капча:")
                .attachment(serenity::CreateAttachment::bytes(
                    image_bytes,
                    "captcha.avif",
                ))
                .components(buttons.clone())
                .ephemeral(true),
        )
        .await?;

    handle_button(
        ctx,
        &button_id,
        300,
        move |press| {
            let image_text = image_text.clone();
            async move {
                let modal = serenity::CreateQuickModal::new("Ввод капчи")
                    .timeout(std::time::Duration::from_secs(300))
                    .field(serenity::CreateInputText::new(
                        serenity::InputTextStyle::Short,
                        "Капча",
                        "",
                    ));
                let response = press.quick_modal(ctx.serenity_context(), modal).await?;
                let Some(response) = response else {
                    return Ok(false);
                };

                let entered = response.inputs.first().map(|s| s.as_str()).unwrap_or("");
                if entered == image_text {
                    sqlx::query("UPDATE sbp_users SET balance = balance + 30 WHERE id = $1")
                        .bind::<i64>(press.user.id.into())
                        .execute(pool)
                        .await?;

                    let _ = add_user_quest_progress(
                        pool,
                        ctx.serenity_context(),
                        press.user.id.get(),
                        "captcha",
                        None,
                        None,
                    )
                    .await;

                    crate::create_edit_response!(
                        ctx,
                        response.interaction,
                        serenity::CreateInteractionResponseMessage::new()
                            .content("✅ Капча пройдена! Вы получили 30 бебр.")
                            .components(Vec::new())
                            .ephemeral(true)
                    );

                    return Ok(true);
                } else {
                    crate::create_edit_response!(
                        ctx,
                        response.interaction,
                        serenity::CreateInteractionResponseMessage::new()
                            .content("❌ Неверная капча. Попробуйте с новой капчой.")
                            .components(Vec::new())
                            .ephemeral(true)
                    );

                    return Ok(true);
                }
            }
        },
        move || {
            for row in &mut buttons {
                if let serenity::CreateActionRow::Buttons(btns) = row {
                    for button in btns {
                        *button = button.clone().disabled(true);
                    }
                }
            }
            async move {
                let _ = msg
                    .edit(
                        ctx,
                        CreateReply::default()
                            .content("Вы не успели ввести капчу!")
                            .components(buttons),
                    )
                    .await;
                Ok(())
            }
        },
    )
    .await?;

    Ok(())
}
