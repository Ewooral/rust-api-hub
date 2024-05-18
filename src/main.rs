// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;
// extern crate rocket_cors;

// use rocket::http::Method;
// use rocket::response::content;
// use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[get("/greet/<greeting>/<name>")]
// fn greet(greeting: String, name: String) -> String {
//     format!("Good {}, {}", greeting, name)
// }

// fn main() {
//     let (allowed_origins, failed_origins) = AllowedOrigins::some(&[
//         "http://localhost:3000",
//         // "http://your_nextjs_app_origin",
//     ]);
//     assert!(failed_origins.is_empty());

//     let options = rocket_cors::Cors {
//         allowed_origins,
//         allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
//         allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
//         allow_credentials: true,
//         ..Default::default()
//     };

//     rocket::ignite()
//         .mount("/", routes![index, greet])
//         .attach(options)
//         .launch();
// }

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/greet/<greeting>/<name>")]
fn greet(greeting: String, name: String) -> String {
    format!("Good {}, {}", greeting, name)
}

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&[
        "http://localhost:3000",
        "http://ec2-16-171-1-163.eu-north-1.compute.amazonaws.com:8000",
        // "http://your_nextjs_app_origin",
    ]);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, greet])
        .attach(options)
        .launch();
}