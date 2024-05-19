#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_cors;
extern crate log;
// extern crate diesel;


use std::sync::Mutex;
use std::collections::HashMap;

use log::info;
use log::error;
use rocket::http::Status;
use rocket::response::status;

use rocket::State;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket_contrib::json::Json;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;



#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    id: String,
    content: String,
}

type MessageMap = Mutex<HashMap<String, String>>;

#[get("/")]
fn index() -> Result<&'static str, status::Custom<String>> {
    Ok("This is Elijah, my first api project in rust using the rocket framework!")
}

#[get("/greet/<greeting>/<name>")]
fn greet(greeting: String, name: String) -> Result<String, status::Custom<String>> {
    if greeting.is_empty() || name.is_empty() {
        Err(status::Custom(Status::BadRequest, "Greeting or name cannot be empty".into()))
    } else {
        Ok(format!("Good {}, {}", greeting, name))
    }
}

#[post("/message", format = "json", data = "<message>")]
fn post_message(message: Json<Message>, map: State<'_, MessageMap>) -> Result<JsonValue, status::Custom<String>> {
    let mut map = map.lock().map_err(|e| {
        error!("Failed to acquire lock: {:?}", e);
        status::Custom(Status::InternalServerError, "Failed to acquire lock".into())
    })?;
    map.insert(message.id.clone(), message.content.clone());
    let success_message = "Post request is successful";
    info!("{}", success_message);
    Ok(JsonValue(json!({"status": "success", "message": success_message}).into()))
}

#[put("/message/<id>", format = "json", data = "<message>")]
fn update_message(id: String, message: Json<Message>, map: State<'_, MessageMap>) -> Result<Option<Json<Message>>, status::Custom<String>> {
    let mut map = map.lock().map_err(|_| status::Custom(Status::InternalServerError, "Failed to acquire lock".into()))?;
    map.insert(id.clone(), message.content.clone());
    Ok(Some(message))
}

#[delete("/message/<id>")]
fn delete_message(id: String, map: State<'_, MessageMap>) -> Result<JsonValue, status::Custom<String>> {
    let mut map = map.lock().map_err(|_| status::Custom(Status::InternalServerError, "Failed to acquire lock".into()))?;
    map.remove(&id);
    Ok(JsonValue(json!({"status": "success"}).into()))
}

#[get("/message/<id>")]
fn get_message(id: String, map: State<'_, MessageMap>) -> Result<Option<Json<Message>>, status::Custom<String>> {
    let hashmap = map.lock().map_err(|_| status::Custom(Status::InternalServerError, "Failed to acquire lock".into()))?;
    let message = hashmap.get(&id).map(|content| {
        Json(Message {
            id: id.clone(),
            content: content.clone()
        })
    });
    Ok(message)
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
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![index, greet, post_message, update_message, delete_message, get_message])
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .attach(options)
        .launch();
}