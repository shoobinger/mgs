use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{params, named_params, Connection, Error};
use std::io;
use std::io::Write;
use crate::conn;
use mgs_common::{AddAsset, AssetType};
use std::ops::Deref;
use rusqlite::types::{FromSql, FromSqlError, ValueRef};
use std::collections::HashMap;

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
    pub asset_type: AssetType,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub enabled: bool,
    pub quantity: Option<Amount>,
}


impl Asset {
    pub fn read(user_id: i32, currency_id: i16, limit: i32, offset: i32) -> Result<Vec<Self>, String> {
        let rates: HashMap<&str, f64> = vec![ ("USDRUB", 72.0 ) ].into_iter().collect();

        conn().prepare(r#"
            SELECT a.id, a.user_id, a.type, a.name, a.description, a.created_at, m.default_symbol_id, SUM(p.quantity)
            FROM assets a
            LEFT JOIN asset_parts p ON p.asset_id = a.id
            LEFT JOIN asset_market_info m ON m.asset_id = a.id
            WHERE a.user_id=:user_id AND a.enabled=1
            GROUP BY a.id
            ORDER BY a.created_at DESC
            LIMIT :limit OFFSET :offset"#
        ).and_then(|mut stmt|
            stmt.query_map_named(named_params! {
             ":user_id": user_id,
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
                    quantity: row.get(6)?,
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