use ::store_product::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpServer, Responder,
};
use core::local_http;
use core::model::product::Product;
use std::io;

// #[route("/{id}", method = "POST")]
// async fn store_product(id: web::Path<String>, body: web::Json<Product>) -> impl Responder {

#[route("/", method = "POST")]
async fn store_product(body: web::Json<Product>) -> impl Responder {
    web::Json(app(body.into_inner()).await.unwrap())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    local_http!(8081, store_product)
}
