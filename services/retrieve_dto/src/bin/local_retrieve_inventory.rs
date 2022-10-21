use ::retrieve_dto::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use core::{local_http, model::inventory_search::InventorySearch, model::DataQueryObject};
use std::io;

#[route("/{key}", method = "GET")]
async fn route(key: web::Path<String>) -> impl Responder {
    let response_data = app(InventorySearch::new(key.into_inner(), None))
        .await
        .unwrap();
    HttpResponse::Ok().json(response_data)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    local_http!(8182, route)
}
