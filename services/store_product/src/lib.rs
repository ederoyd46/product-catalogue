use core::database::store_database_item;
use core::error_and_panic;
use core::types::{Config, Storable, ConfigBuilder, CustomValue};
use std::env;
use lambda_http::{tower::BoxError, Request};
use log::{error, info, LevelFilter, SetLoggerError};
use serde_json::Value;

pub fn extract_key_from_request(event: Request) -> String {
    match event.uri().path().rsplit_once('/') {
        Some((_prefix, path_name)) => path_name.to_string(),
        None => error_and_panic!("Could not find Path from the environment"),
    }
}

pub async fn app(value: Value) -> Result<String, BoxError> {
    let config = ConfigBuilder::new()
        .table_name(env::var("DATABASE").unwrap())
        .build()
        .await;

    let data = CustomValue { key: "1".to_owned(), value };
    store_handler(&config, data).await
}

pub async fn store_handler<T: Storable>(config: &Config, data: T) -> Result<String, BoxError> {
    if !data.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&config.table_name, &data, &config.dynamodb).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(format!("Stored, {}!", data.get_pk().as_s().unwrap()))
}

pub fn initialise_logger() -> Result<(), SetLoggerError> {
    env_logger::builder()
        .filter(Some("lib::database"), LevelFilter::Debug)
        .filter_level(LevelFilter::Info)
        .init();
    Ok(())
}
