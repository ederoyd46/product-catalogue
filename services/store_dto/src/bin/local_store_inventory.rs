use ::store_dto::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use core::local_http;
use core::model::inventory::Inventory;
use std::io;

#[route("/", method = "POST")]
async fn route(body: web::Json<Inventory>) -> impl Responder {
    let response_data = app(body.into_inner()).await.unwrap();
    HttpResponse::Ok().json(response_data)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    local_http!(8082, route)
}
