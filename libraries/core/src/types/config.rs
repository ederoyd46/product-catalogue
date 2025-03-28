use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_types::SdkConfig;

#[derive(Debug)]
pub struct Config {
    pub table_name: String,
    pub aws_sdk_config: SdkConfig,
    pub dynamodb: Client,
}
#[derive(Debug)]
pub struct ConfigBuilder {
    table_name: String,
    endpoint_url: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            table_name: "default_table".to_string(),
            endpoint_url: None,
        }
    }

    pub fn table_name(mut self, name: String) -> Self {
        self.table_name = name;
        self
    }

    pub fn endpoint_url(mut self, endpoint_url: Option<String>) -> Self {
        self.endpoint_url = endpoint_url;
        self
    }

    pub async fn build(self) -> Config {
        log::info!("ConfigBuilder: {:?}", &self);

        let aws_sdk_config = match self.endpoint_url {
            Some(url) => {
                aws_config::defaults(BehaviorVersion::v2025_01_17())
                    .endpoint_url(url)
                    .load()
                    .await
            }

            None => aws_config::load_defaults(BehaviorVersion::v2025_01_17()).await,
        };

        Config {
            table_name: self.table_name,
            dynamodb: Client::new(&aws_sdk_config),
            aws_sdk_config,
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}
