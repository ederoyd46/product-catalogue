use serde_derive::{Deserialize, Serialize};

use super::{DataQueryObject, DataTransferObject};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InventorySearch {
    pub key: String,
}

impl DataQueryObject<String> for InventorySearch {
    fn new(key: String, _args: Option<Vec<String>>) -> Self {
        InventorySearch { key }
    }
}

impl DataTransferObject for InventorySearch {
    fn is_valid(&self) -> bool {
        !self.key.is_empty()
    }

    fn get_key(&self) -> String {
        format!("{}{}", "INVENTORY:", self.key)
    }

    fn get_metadata(&self) -> Vec<String> {
        [self.key.to_string()].to_vec()
    }
}
