use rocket::{get, post, Request, response};
use rocket::http::Header;
use rocket::http::hyper::header::{AccessControlAllowOrigin, Headers};
use rocket::response::Responder;
use rocket_contrib::json::Json;

use serde::Serialize;

use mgs_back::db::models::asset::Asset;

use crate::models::AssetRepr;

#[derive(Responder)]
pub struct Response {
    body: Json<Page>,
    header: Header<'static>,
}

#[derive(Serialize)]
pub struct Page {
    elements: Vec<AssetRepr>,
    limit: i32,
    offset: i32,
}

const DEFAULT_LIMIT: i32 = 20;
const DEFAULT_OFFSET: i32 = 0;

#[get("/assets?<limit>&<offset>")]
pub fn get_assets(limit: Option<i32>, offset: Option<i32>) -> Response {
    let limit = limit.unwrap_or_else(|| DEFAULT_LIMIT);
    let offset = offset.unwrap_or_else(|| DEFAULT_OFFSET);
    let assets = Asset::read(1, limit, offset)
        .expect("Can't read assets");

    let elements: Vec<_> = assets.into_iter().map(|a| AssetRepr {
        id: a.id,
        created_at: a.created_at.timestamp_millis(),
        enabled: a.enabled,
        name: a.name,
        user_id: a.user_id,
    }).collect();

    Response {
        body: Json(Page {
            elements,
            limit,
            offset
        }),
        header: Header::new("Access-Control-Allow-Origin", "*"),
    }
}

#[post("/assets")]
pub fn add_asset() -> &'static str {
    "Hello, world!"
}