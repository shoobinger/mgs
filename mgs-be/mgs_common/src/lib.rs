use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddAsset {
    #[serde(rename = "type")]
    pub asset_type: i8,
    pub name: String,
    pub description: String,
}
