use crate::types::*;
use rand::seq::IndexedRandom as _;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Success,
    Danger,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Single(String),
    Multiple(Vec<String>),
}

impl Content {
    pub fn choose(&self) -> Option<&str> {
        match self {
            Content::Single(text) => Some(text.as_str()),
            Content::Multiple(list) => {
                let mut rng = rand::rng();
                list.choose(&mut rng).map(|s| s.as_str())
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Button {
    pub custom_id: String,
    pub label: String,
    pub style: ButtonStyle,
}

#[derive(Debug, Deserialize)]
pub struct RawDialogue {
    pub content: Content,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Deserialize)]
pub struct DialoguesFile {
    pub dialogues: HashMap<String, RawDialogue>,
}

#[derive(Debug)]
pub struct Dialogue<'a> {
    pub content: &'a str,
    pub buttons: Vec<serenity::CreateActionRow>,
}
