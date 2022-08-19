use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::io;

use actix_web::{
    middleware, route,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

/// GraphQL endpoint
// #[route("/", method = "GET", method = "POST")]
// async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
//     let response_data = data.execute(&st, &()).await;
//     HttpResponse::Ok().json(response_data)
// }

#[derive(Deserialize, Serialize, Debug)]
struct AppState {
    id: i64,
    description: String,
}

#[route("/{id}", method = "POST")]
async fn store_product(id: web::Path<String>, body: web::Json<AppState>) -> impl Responder {
    web::Json(json!({ "id": id.to_owned(), "body": body.description }))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server on port 8081");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .service(store_product)
            .wrap(middleware::Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
