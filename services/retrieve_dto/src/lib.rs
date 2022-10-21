use core::config::build_config;
use core::database::retrieve_item;
use core::error_and_panic;
use core::model::{DataQueryObject, DataTransferObject};
use core::types::{ApplicationError, Config};
use serde_json::{json, Value};

pub async fn app<T: DataQueryObject<String> + DataTransferObject + serde::Serialize>(
    dto: T,
) -> Result<Value, ApplicationError> {
    if !dto.is_valid() {
        error_and_panic!("Invalid input, please use a string");
    }

    let config = build_config().await?;

    log::info!("Metadata {:?}", &dto.get_metadata());

    let response = retrieve_handler(&config, dto).await?;
    Ok(json!(response))
}

async fn retrieve_handler<T: DataQueryObject<String> + DataTransferObject>(
    config: &Config,
    data: T,
) -> Result<Value, ApplicationError> {
    if !data.is_valid() {
        error_and_panic!("No key specified");
    }

    let value = retrieve_item(&config.table_name, &data, &config.dynamodb).await?;

    log::info!("value: {:?}", value);

    Ok(value)
}
