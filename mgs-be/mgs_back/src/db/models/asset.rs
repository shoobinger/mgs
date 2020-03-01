use chrono::{NaiveDate, NaiveDateTime};
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::db::schema::assets;


#[derive(Debug, Queryable)]
pub struct Asset {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub enabled: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "assets"]
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
    pub(crate) fn save(&self, conn: &SqliteConnection) -> Result<(), String> {
        diesel::insert_into(assets::table)
            .values(self)
            .execute(conn)
            .map(|_| () ).map_err(|e| format!("{:?}", e) )
    }
}

impl Asset {
    pub fn read_all(user_id: i32, conn: &SqliteConnection) -> Result<Vec<Self>, String> {
        assets::table
            .filter(assets::user_id.eq(user_id))
            .load::<Self>(conn).map_err(|e| format!("{:?}", e) )
    }
}