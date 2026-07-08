use crate::{
    modules::dromland::{display_class, game::get_main_menu_buttons},
    types::*,
};
use poise::serenity_prelude::{self as serenity};

const CLASSES: [(&str, [i32; 3]); 3] = [
    ("mage", [59, 200, 140]),
    ("warrior", [100, 10, 100]),
    ("heavy", [159, 0, 110]),
];

pub async fn handle_char_create_button(
    ctx: &serenity::Context,
    press: &serenity::ComponentInteraction,
    data: &Data,
) -> Result<(), Error> {
    let modal = serenity::CreateQuickModal::new("Создание персонажа")
        .timeout(std::time::Duration::from_secs(600))
        .field(
            serenity::CreateInputText::new(serenity::InputTextStyle::Short, "Имя персонажа", "")
                .max_length(35)
                .min_length(3),
        )
        .field(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Short,
                "Класс персонажа (маг/воин/танк)",
                "",
            )
            .max_length(15)
            .min_length(2),
        );

    let response = press.quick_modal(ctx, modal).await?;

    let Some(response) = response else {
        return Ok(());
    };
    let press = response.interaction;

    let char_name = response.inputs[0].trim();
    let char_class = response.inputs[1].trim();

    if char_name.len() <= 4 {
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Имя вашего персонажа слишком короткое")
                .ephemeral(true)
        );
        return Ok(());
    }

    let char_class = match char_class {
        "маг" => CLASSES.iter().find(|&&(name, _)| name == "mage").unwrap(),
        "воин" => CLASSES
            .iter()
            .find(|&&(name, _)| name == "warrior")
            .unwrap(),
        "танк" => CLASSES.iter().find(|&&(name, _)| name == "heavy").unwrap(),

        _ => {
            crate::create_response!(
                ctx,
                press,
                serenity::CreateInteractionResponseMessage::new()
                    .content("Неизвестный класс. Доступные классы: воин | маг | танк")
                    .ephemeral(true)
            );
            return Ok(());
        }
    };

    let result = sqlx::query(
        "INSERT INTO dl_users (id, name, class, health, mana, damage) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING"
    )
        .bind(press.user.id.get() as i64)
        .bind(char_name)
        .bind(char_class.0)
        .bind(char_class.1[0])
        .bind(char_class.1[1])
        .bind(char_class.1[2])
        .execute(&data.pool)
        .await?;

    if result.rows_affected() == 0 {
        crate::create_response!(
            ctx,
            press,
            serenity::CreateInteractionResponseMessage::new()
                .content("Вы уже создали своего персонажа")
                .ephemeral(true)
        );
        return Ok(());
    }

    crate::create_edit_response!(
        ctx,
        press,
        serenity::CreateInteractionResponseMessage::new()
            .content(format!(
                "Добро пожаловать, {} {}",
                display_class(char_class.0).unwrap(),
                char_name
            ))
            .components(get_main_menu_buttons())
            .embeds(Vec::new())
            .ephemeral(true)
    );

    Ok(())
}
