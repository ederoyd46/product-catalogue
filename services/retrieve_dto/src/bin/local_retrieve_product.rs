use ::retrieve_dto::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use core::{local_http, model::product_search::ProductSearch};
use log::info;
use std::io;

#[route("/{key}", method = "GET")]
async fn route(key: web::Path<String>) -> impl Responder {
    let response_data = app(ProductSearch { key: key.clone() }).await.unwrap();
    info!("Key {:?}", response_data);
    HttpResponse::Ok().json(response_data)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    local_http!(8181, route)
}
