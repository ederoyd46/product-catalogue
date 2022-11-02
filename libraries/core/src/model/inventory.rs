use serde_derive::{Deserialize, Serialize};

use super::DataTransferObject;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Inventory {
    pub key: String,
    pub inventory: u64,
}

impl DataTransferObject for Inventory {
    fn is_valid(&self) -> bool {
        !self.key.is_empty()
    }

    fn get_key(&self) -> String {
        format!("{}{}", "INVENTORY:", self.key)
    }

    fn get_metadata(&self) -> Vec<String> {
        [self.key.to_string(), self.inventory.to_string()].to_vec()
    }
}
