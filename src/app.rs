use std::u8;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// This struct represents state
pub struct AppState {
    pub app_name: String,
}
#[derive(Serialize)]
struct EchoResponse {
    message: String,
    app_name: String,
    index: Option<u8>,
}

#[derive(Deserialize, Serialize)]
pub struct RequestData {
    message: String,
    index: Option<u8>,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}
#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[get("/x")]
pub async fn index_x(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("x {app_name}!") // <- response with app_name
}

// #[post("/turnaround")]
// turnaround takes message from the request and returns it with some additional data
pub async fn turnaround(
    data: web::Json<RequestData>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let app_name = app_data.app_name.clone();
    // let prefix = String::from("mod_");
    // let mod_req = format!("{}{}", prefix, app_name);

    let message = data.message.clone();
    // let message = data.0.message.clone();
    let response = EchoResponse {
        message,
        app_name,
        index: data.0.index,
    };
    HttpResponse::Ok().json(response)
}

pub async fn echo(data: web::Json<RequestData>) -> impl Responder {
    HttpResponse::Ok().json(data.0)
}
