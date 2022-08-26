use dto::add_field;
use serde_derive::{Deserialize, Serialize};

use super::DataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    pub key: String,
    pub name: String,
    pub price: Option<Price>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Price {
    pub currency_code: String,
    pub amount: f64,
}

impl DataTransferObject for Product {
    fn is_valid(&self) -> bool {
        !&self.key.is_empty()
    }

    fn get_key(&self) -> String {
        format!("{}{}", "PRODUCT:", self.key)
    }

    fn get_meta_data(&self) -> Vec<String> {
        let mut metadata = [self.name.to_string()].to_vec();

        if let Some(description) = &self.description {
            metadata.push(description.to_string());
        }

        if let Some(price) = &self.price {
            metadata.push(price.currency_code.to_string());
        }

        metadata
    }
}
