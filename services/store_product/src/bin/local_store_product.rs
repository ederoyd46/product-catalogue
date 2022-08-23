use ::store_product::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpServer, Responder,
};
use core::local_http;
use core::model::product::Product;
use log::info;
use std::io;

#[route("/", method = "POST")]
async fn store_product(body: web::Json<Product>) -> impl Responder {
    let response = app(body.into_inner()).await.unwrap();
    web::Json(response)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    local_http!(8081, store_product)
}
