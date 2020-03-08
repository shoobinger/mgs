use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{params, Connection, Error};
use std::io;
use std::io::Write;
use crate::conn;

#[derive(Debug)]
pub struct Asset {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct AssetIns {
    pub user_id: i32,
    pub name: String,
    pub enabled: bool,
}

pub struct AssetSnapshot {
    pub id: i32,
    pub asset_id: i32,
    pub quantity: i32,
    pub date: NaiveDate,
    pub created_at: NaiveDateTime,
}

impl AssetIns {
//    pub(crate) fn save(&self, conn: &SqliteConnection) -> Result<(), String> {
//        diesel::insert_into(assets::table)
//            .values(self)
//            .execute(conn)
//            .map(|_| () ).map_err(|e| format!("{:?}", e) )
//    }
}


impl Asset {
    pub fn read(user_id: i32, limit: i32, offset: i32) -> Result<Vec<Self>, String> {
        conn().prepare(r#"
            SELECT id, user_id, name, created_at
            FROM assets
            WHERE user_id=?1 AND enabled=1
            ORDER BY created_at DESC
            LIMIT ?2 OFFSET ?3"#
        ).and_then(|mut stmt|
            stmt.query_map(params![user_id, limit, offset], |row| {
                Ok(Asset {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    created_at: row.get(3)?,
                    enabled: true,
                })
            })?.collect()
        ).map_err(|e| e.to_string())
    }
}