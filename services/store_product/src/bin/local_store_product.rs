use ::store_product::app;
use actix_web::{
    middleware, route,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use core::model::product::Product;
use core::model::DataTransferObject;
use serde_json::json;
use std::io;

const PORT: u16 = 8081;

// #[route("/{id}", method = "POST")]
// async fn store_product(body: web::Json<Product>) -> impl Responder {

#[route("/", method = "POST")]
async fn store_product(id: web::Path<String>, body: web::Json<Product>) -> impl Responder {
    let product = body.into_inner();
    let response = app(product.clone()).await.unwrap();

    HttpResponse::Ok().json(json!({
        "id": id.to_owned(),
        "is_valid": product.is_valid(),
        "key": product.get_key(),
        "meta_data": product.get_meta_data(),
        "response": response
    }))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server on port {}", PORT);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .service(store_product)
            .wrap(middleware::Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
