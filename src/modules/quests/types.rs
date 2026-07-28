use sqlx::prelude::FromRow;

#[derive(Debug, PartialEq, Eq, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum UsersRequiredType {
    None,
    Different,
    Any,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, Default)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub action: String,
    pub reward: u32,
    pub users_required_type: Option<UsersRequiredType>,
    pub ends: Option<u16>,
    pub max_progress: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "quest_status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Completed,
    Expired,
}

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct UserQuest {
    pub user_id: i64,
    pub quest_id: String,
    pub progress: i32,
    pub users: Vec<i64>,
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Status,
}

impl UserQuest {
    pub fn to_quest(&self) -> Option<&'static Quest> {
        super::helpers::get_quests().get(&self.quest_id)
    }
}
