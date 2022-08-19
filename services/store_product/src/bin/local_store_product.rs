use serde_json::{json, Value};
use std::io;

use actix_web::{
    middleware, route,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

const PORT: u16 = 8081;

#[route("/{id}", method = "POST")]
async fn store_product(id: web::Path<String>, body: web::Json<Value>) -> impl Responder {
    HttpResponse::Created().json(json!({ "id": id.to_owned(), "body": body }))
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
