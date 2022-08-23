use ::store_dto::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpServer, Responder,
};
use core::local_http;
use core::model::inventory::Inventory;
use log::info;
use std::io;

#[route("/", method = "POST")]
async fn route(body: web::Json<Inventory>) -> impl Responder {
    let response = app(body.into_inner()).await.unwrap();
    web::Json(response)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    local_http!(8082, route)
}
