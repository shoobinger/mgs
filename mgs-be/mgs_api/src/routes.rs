use rocket::{get, post, Request, response};
use rocket::http::Header;
use rocket::http::hyper::header::{AccessControlAllowOrigin, Headers};
use rocket::response::Responder;
use rocket_contrib::json::Json;

use mgs_back::db::models::asset::Asset;
use mgs_back::establish_connection;

use crate::models::AssetRepr;

#[derive(Responder)]
pub struct Response {
    body: Json<Vec<AssetRepr>>,
    header: Header<'static>,
}

#[get("/assets")]
pub fn get_assets() -> Response {
    let conn = establish_connection();
    let assets = Asset::read_all(1, &conn).expect("Can't read assets");

    println!("{}", assets.len());

    Response {
        body: Json(
            assets.into_iter().map(|a| AssetRepr {
                id: a.id,
                created_at: a.created_at.timestamp_millis(),
                enabled: a.enabled,
                name: a.name,
                user_id: a.user_id,
            }).collect()),
        header: Header::new("Access-Control-Allow-Origin", "*"),
    }
}

#[post("/assets")]
pub fn add_asset() -> &'static str {
    "Hello, world!"
}