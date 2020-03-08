use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetRepr {
    pub id: i32,
    pub name: String,
    pub created_at: i64,
    pub enabled: bool,
}

