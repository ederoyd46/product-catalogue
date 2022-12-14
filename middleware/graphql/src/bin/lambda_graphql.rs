use actix_web::web::Buf;
use juniper::http::GraphQLRequest;
use lambda_http::{service_fn, Request};
use lambda_runtime::Error;
use serde_json::Value;
use std::sync::Arc;

use product_catalogue_graphql::schema::{create_schema, Schema};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let schema = Arc::new(create_schema());
    log::info!("Startup");
    lambda_http::run(service_fn(move |event| {
        lambda_handler(event, schema.clone())
    }))
    .await?;
    log::info!("Shutting down now");
    Ok(())
}

async fn lambda_handler(event: Request, schema: Arc<Schema>) -> Result<Value, Error> {
    let data: GraphQLRequest = serde_json::from_reader(event.body().reader())?;
    let response_data = data.execute(&schema, &()).await;
    Ok(serde_json::to_value(&response_data).unwrap())
}
