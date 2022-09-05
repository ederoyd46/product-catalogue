use core::database::store_database_item;
use core::error_and_panic;
use core::model::DataTransferObject;
use core::types::{ApplicationError, Config, ConfigBuilder, CustomValue, Storable};
use log::{error, info};
use once_cell::sync::OnceCell;
use serde_json::value::to_value;
use serde_json::{json, Value};
use std::env;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub async fn app<T: DataTransferObject + serde::Serialize>(
    dto: T,
) -> Result<Value, ApplicationError> {
    if !dto.is_valid() {
        error_and_panic!("Invalid input, please use a string");
    }

    let config = if let Some(config) = CONFIG.get() {
        config
    } else {
        let config = ConfigBuilder::new()
            .table_name(env::var("DATABASE")?)
            .endpoint_url(Some("http://localhost:4566".to_string()))
            .build()
            .await;
        CONFIG.set(config).unwrap();
        info!("Configuration loaded");
        CONFIG.get().unwrap()
    };

    info!("Metadata {:?}", &dto.get_metadata());

    let data = CustomValue {
        key: dto.get_key().to_string(),
        value: to_value(&dto)?,
        metadata: to_value(&dto.get_metadata())?,
    };
    let response = store_handler(&config, data).await?;
    Ok(json!(response))
}

async fn store_handler<T: Storable>(config: &Config, data: T) -> Result<T, ApplicationError> {
    if !data.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&config.table_name, &data, &config.dynamodb).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(data)
}
