use serde_derive::{Deserialize, Serialize};

use super::DataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    pub key: String,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

impl DataTransferObject for Product {
    fn is_valid(&self) -> bool {
        !&self.key.is_empty()
    }

    fn get_key(&self) -> String {
        format!("{}{}", "PRODUCT:", self.key)
    }

    fn get_meta_data(&self) -> Vec<String> {
        [
            self.key.to_string(),
            self.price.to_string(),
            self.name.to_string(),
        ]
        .to_vec()
    }
}
