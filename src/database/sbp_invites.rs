use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct SbpInvite {
    pub user_id: i64,
    pub invited_user_id: i64,
}