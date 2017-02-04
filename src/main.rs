#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod users; 
mod models;

fn main() {
    rocket::ignite().mount("/", routes![users::index, users::files, users::login, users::user_page]).launch();
}
