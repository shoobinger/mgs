use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AssetRepr {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: i64,
    pub enabled: bool,
}

