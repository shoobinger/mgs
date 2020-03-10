use serde::{Serialize, Deserialize};
use rusqlite::types::{FromSql, ValueRef, FromSqlError, ToSqlOutput};
use rusqlite::{ToSql, Error};
use std::ops::Deref;

#[derive(Debug, Deserialize)]
pub struct AddAsset {
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    pub name: String,
    pub description: String,
}

#[repr(i16)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AssetType {
    CURRENCY = 1,
    ETF,
    CRYPTO
}

impl From<i16> for AssetType {
    fn from(ordinal: i16) -> Self {
        unsafe { ::std::mem::transmute(ordinal)}
    }
}

impl Into<i16> for AssetType {
    fn into(self) -> i16 {
        unsafe { ::std::mem::transmute::<AssetType, i16>(self)}
    }
}

impl FromSql for AssetType {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        return Ok(Self::from(value.as_i64()? as i16))
    }
}

impl ToSql for AssetType {
    //noinspection ALL
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, Error> {
        let ordinal: i16 = (*self).into();
        return Ok(ToSqlOutput::from(ordinal))
    }
}
