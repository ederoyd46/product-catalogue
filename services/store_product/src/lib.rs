use core::database::store_database_item;
use core::error_and_panic;
use core::model::DataTransferObject;
use core::types::{ApplicationError, Config, ConfigBuilder, CustomValue, Storable};
use log::{error, info};
use serde_json::{json, Value};
use std::env;

use once_cell::sync::OnceCell;

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
            .table_name(env::var("DATABASE").unwrap())
            .build()
            .await;
        CONFIG.set(config).unwrap();
        info!("Configuration loaded");
        CONFIG.get().unwrap()
    };

    let value = serde_json::value::to_value(&dto).unwrap();
    info!("{:?}", &dto.get_meta_data());

    let data = CustomValue {
        key: dto.get_key().to_string(),
        value,
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
