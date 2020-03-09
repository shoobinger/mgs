use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{params, named_params, Connection, Error};
use std::io;
use std::io::Write;
use crate::conn;
use mgs_common::AddAsset;
use std::ops::Deref;
use rusqlite::types::{FromSql, FromSqlError, ValueRef};

#[derive(Debug)]
pub struct Amount(pub f64);

impl Amount {
    pub fn zero() -> Self {
        return Amount(0.0);
    }
}

impl Deref for Amount {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

const FRACTION: f64 = 100.0;

impl FromSql for Amount {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        return Ok(Amount(value.as_i64()? as f64 / FRACTION))
    }
}

#[derive(Debug)]
pub struct Asset {
    pub id: i32,
    pub user_id: i32,
    pub asset_type: i8,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub enabled: bool,
    pub value: Option<Amount>,
}

#[derive(Debug)]
pub struct AssetSnapshot {
    pub id: i32,
    pub asset_id: i32,
    pub quantity: i32,
    pub date: NaiveDate,
    pub created_at: NaiveDateTime,
}

impl Asset {
    pub fn read(user_id: i32, currency_id: i16, limit: i32, offset: i32) -> Result<Vec<Self>, String> {
        conn().prepare(r#"
            SELECT a.id, a.user_id, a.type, a.name, a.description, a.created_at, v.value
            FROM assets a
            LEFT JOIN snapshot_values v ON v.snapshot_id = a.last_snapshot_id AND v.currency_id=:currency_id
            WHERE a.user_id=:user_id AND a.enabled=1
            ORDER BY a.created_at DESC
            LIMIT :limit OFFSET :offset"#
        ).and_then(|mut stmt|
            stmt.query_map_named(named_params! {
             ":user_id": user_id,
             ":currency_id": currency_id,
             ":limit": limit,
             ":offset": offset
            }, |row| {
                Ok(Asset {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    asset_type: row.get(2)?,
                    name: row.get(3)?,
                    description: row.get(4)?,
                    created_at: row.get(5)?,
                    enabled: true,
                    value: row.get(6)?,
                })
            })?.collect()
        ).map_err(|e| e.to_string())
    }

    pub fn save(user_id: i32, add_asset: &AddAsset) -> Result<Self, String> {
        let conn = conn();
        conn.execute(r#"
            INSERT INTO assets (user_id, type, name, description)
            VALUES (?1, ?2, ?3, ?4)
        "#, params![user_id, add_asset.asset_type, add_asset.name, add_asset.description]);

        let id = conn.last_insert_rowid() as i32;
        let asset = conn.query_row(r#"
            SELECT id, user_id, type, name, description, created_at
            FROM assets
            WHERE id = ?1
        "#, params![id], |row|
            Ok(Asset {
                id: row.get(0)?,
                user_id: row.get(1)?,
                asset_type: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                created_at: row.get(5)?,
                enabled: true,
                value: Some(Amount::zero()),
            }),
        ).unwrap();

        Ok(asset)
    }
}