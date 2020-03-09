use rocket::{get, post, Request, response};
use rocket::http::Header;
use rocket::http::hyper::header::{AccessControlAllowOrigin, Headers};
use rocket::response::Responder;
use rocket_contrib::json::Json;

use serde::Serialize;

use mgs_back::db::models::asset::{Asset};

use crate::models::{AssetRepr};
use std::thread;
use std::time::Duration;
use mgs_common::AddAsset;
use std::ops::Deref;

#[derive(Responder)]
pub struct Response {
    body: Json<Page>,
    header: Header<'static>,
}

#[derive(Responder)]
pub struct AddResponse {
    body: Json<AssetRepr>,
    header: Header<'static>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    elements: Vec<AssetRepr>,
    limit: i32,
    offset: i32
}

const DEFAULT_LIMIT: i32 = 20;
const DEFAULT_OFFSET: i32 = 0;

#[get("/assets?<currency_id>&<limit>&<offset>")]
pub fn get_assets(currency_id: Option<i16>, limit: Option<i32>, offset: Option<i32>) -> Response {
    let limit = limit.unwrap_or_else(|| DEFAULT_LIMIT);
    let offset = offset.unwrap_or_else(|| DEFAULT_OFFSET);
    //let currency_id = currency_id.unwrap_or_else(|| User::get_default_currency());
    //let user_id =
    //TODO implement user

    let user_id = 1;
    let currency_id = currency_id.unwrap_or_else(|| 1);

    let assets = Asset::read(user_id, currency_id, limit, offset)
        .expect("Can't read assets");

    let elements: Vec<AssetRepr> = assets.into_iter().map(|a| AssetRepr::from(a)).collect();

    Response {
        body: Json(Page { elements, limit, offset }),
        header: Header::new("Access-Control-Allow-Origin", "*"),
    }
}

#[post("/assets", data = "<add_asset>")]
pub fn add_asset(add_asset: Json<AddAsset>) -> AddResponse {
    let resp = AssetRepr::from(Asset::save(1, add_asset.deref()).unwrap());

    AddResponse {
        body: Json(resp),
        header: Header::new("Access-Control-Allow-Origin", "*"),
    }
}