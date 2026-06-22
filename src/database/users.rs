use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: i64,
    pub is_banned: bool,
    pub ended_quests_notifications: bool,
    pub new_quests_notifications: bool,
    pub harem_id: Option<i64>,
}