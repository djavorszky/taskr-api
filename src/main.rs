#![feature(proc_macro_hygiene, decl_macro)]
#![deny(warnings)]

#[macro_use]
extern crate rocket;

mod string_utils;

use rocket_contrib::serve::StaticFiles;

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}", string_utils::capitalize(name.as_str()))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .mount("/res", StaticFiles::from("resources"))
        .launch();
}
