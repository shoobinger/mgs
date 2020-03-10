#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::{self, routes};

mod routes;
mod models;

pub fn start() {
    rocket::ignite().mount("/api", routes![
            routes::get_assets,
            routes::add_asset
    ]).launch();
}
