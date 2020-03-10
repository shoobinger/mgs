use serde::{Serialize, Deserialize};
use mgs_back::db::models::asset::{Asset};
use std::iter::FromIterator;
use std::ops::Deref;
use mgs_common::AssetType;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetRepr {
    pub id: i32,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub value: Option<f64>
}

impl From<Asset> for AssetRepr {
    fn from(a: Asset) -> Self {
        Self {
            id: a.id,
            asset_type: a.asset_type,
            name: a.name,
            description: a.description,
            created_at: a.created_at.timestamp_millis(),
            value: a.value.map(|v| v.0)
        }
    }
}
