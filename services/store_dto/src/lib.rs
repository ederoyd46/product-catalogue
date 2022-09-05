use core::database::store_database_item;
use core::error_and_panic;
use core::model::DataTransferObject;
use core::types::{ApplicationError, Config, ConfigBuilder};
use log::{error, info};
use once_cell::sync::OnceCell;
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
        let mut config = ConfigBuilder::new().table_name(env::var("DATABASE")?);
        if env::var("ENDPOINT_URL").is_ok() {
            config = config.endpoint_url(Some(env::var("ENDPOINT_URL").unwrap()))
        }
        CONFIG.set(config.build().await).unwrap();
        info!("Configuration loaded");
        CONFIG.get().unwrap()
    };

    info!("Metadata {:?}", &dto.get_metadata());

    let response = store_handler(&config, dto).await?;
    Ok(json!(response))
}

async fn store_handler<T: DataTransferObject>(
    config: &Config,
    data: T,
) -> Result<T, ApplicationError> {
    if !data.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&config.table_name, &data, &config.dynamodb).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(data)
}
