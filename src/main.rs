#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/greet/<greeting>/<name>")]
fn greet(greeting: String, name: String) -> String {
    format!("Good {}, {}", greeting, name)
}

fn main() {
    rocket::ignite().mount("/", routes![index, greet]).launch();
}