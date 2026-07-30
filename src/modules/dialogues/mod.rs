use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;

use crate::helpers::resolve_data_path;
use crate::types::*;

pub mod buttons;
pub mod types;

use types::*;

static DIALOGUES_CACHE: OnceLock<HashMap<String, RawDialogue>> = OnceLock::new();

pub struct DialoguesManager {}

impl<'a> DialoguesManager {
    pub fn new(data_path: &'a str) -> Result<(), Error> {
        if DIALOGUES_CACHE.get().is_some() {
            return Err("DialoguesManager error: Диалоги уже были загружены ранее!".into());
        }
        let file_content = fs::read_to_string(resolve_data_path(data_path))?;
        let parsed: DialoguesFile = serde_json::from_str(&file_content)?;

        DIALOGUES_CACHE
            .set(parsed.dialogues)
            .map_err(|_| "Не удалось записать данные в кеш (уже инициализировано)")?;

        Ok(())
    }

    pub fn get_dialogue(id: &str) -> Option<Dialogue<'static>> {
        let cache = DIALOGUES_CACHE.get()?;
        let raw_dialogue = cache.get(id)?;
        let selected_content = raw_dialogue.content.choose()?;

        let buttons = raw_dialogue
            .buttons
            .iter()
            .map(|b| {
                let style = match b.style {
                    ButtonStyle::Primary => serenity::ButtonStyle::Primary,
                    ButtonStyle::Secondary => serenity::ButtonStyle::Secondary,
                    ButtonStyle::Danger => serenity::ButtonStyle::Danger,
                    ButtonStyle::Success => serenity::ButtonStyle::Success,
                };

                serenity::CreateButton::new(&b.custom_id)
                    .label(&b.label)
                    .style(style)
            })
            .collect();

        Some(Dialogue {
            content: selected_content,
            buttons: vec![serenity::CreateActionRow::Buttons(buttons)],
        })
    }
}
