use serde_derive::{Deserialize, Serialize};

use super::{DataQueryObject, DataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductSearch {
    pub key: String,
}

impl DataQueryObject<String> for ProductSearch {
    fn new(key: String, _args: Option<Vec<String>>) -> Self {
        ProductSearch { key: key }
    }
}

impl DataTransferObject for ProductSearch {
    fn is_valid(&self) -> bool {
        !&self.key.is_empty()
    }

    fn get_key(&self) -> String {
        format!("{}{}", "PRODUCT:", self.key)
    }

    fn get_metadata(&self) -> Vec<String> {
        [self.key.to_string()].to_vec()
    }
}
