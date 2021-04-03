#![feature(proc_macro_hygiene, decl_macro)]
#![deny(warnings)]
#![feature(test)]

#[macro_use]
extern crate rocket;

mod string_utils;

use rocket_contrib::serve::StaticFiles;

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}", string_utils::capitalize(name.as_str()))
}

#[get("/hello?wave&<name>")]
fn wave(name: Option<String>) -> String {
    name.map(|name| format!("Hi, {}!", string_utils::capitalize(name.as_str())))
        .unwrap_or_else(|| "Hello!".into())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, wave])
        .mount("/res", StaticFiles::from("resources"))
        .launch();
}
