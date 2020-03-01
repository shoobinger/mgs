#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::{self, routes};

mod routes;
mod models;

fn main() {
    rocket::ignite().mount("/", routes![routes::get_assets]).launch();
}
