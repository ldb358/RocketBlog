#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod users; 

fn main() {
    rocket::ignite().mount("/", routes![users::index, users::files, users::login, users::user_page]).launch();
}
