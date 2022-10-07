use crate::types::{ApplicationError, Config, ConfigBuilder};
use once_cell::sync::OnceCell;
use std::env;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub async fn build_config<'a>() -> Result<&'a Config, ApplicationError> {
    let config = if let Some(config) = CONFIG.get() {
        config
    } else {
        let mut config = ConfigBuilder::new().table_name(env::var("DATABASE")?);
        if env::var("LOCALSTACK_HOSTNAME").is_ok() {
            let localstack_hostname = env::var("LOCALSTACK_HOSTNAME").unwrap();
            let localstack_url = format!("http://{}:4566", localstack_hostname);
            config = config.endpoint_url(Some(localstack_url))
        }
        if env::var("ENDPOINT_URL").is_ok() {
            let endpoint_url = env::var("ENDPOINT_URL").unwrap();
            config = config.endpoint_url(Some(endpoint_url))
        }

        CONFIG.set(config.build().await).unwrap();
        log::info!("Configuration loaded");
        CONFIG.get().unwrap()
    };

    Ok(config)
}
