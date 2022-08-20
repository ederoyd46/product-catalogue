use core::database::store_database_item;
use core::error_and_panic;
use core::model::DataTransferObject;
use core::types::{Config, ConfigBuilder, CustomValue, Storable, ApplicationError};
use log::{error, info, LevelFilter, SetLoggerError};
use std::env;

pub async fn app<T: DataTransferObject + serde::Serialize>(dto: T) -> Result<String, ApplicationError> {
    if !dto.is_valid() {
        error_and_panic!("Invalid input, please use a string");
    }

    let config = ConfigBuilder::new()
        .table_name(env::var("DATABASE").unwrap())
        .build()
        .await;

    let value = serde_json::value::to_value(&dto).unwrap();
    println!("{:?}", &dto.get_meta_data());

    let data = CustomValue {
        key: dto.get_key().to_string(),
        value,
    };
    store_handler(&config, data).await
}

async fn store_handler<T: Storable>(config: &Config, data: T) -> Result<String, ApplicationError> {
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
