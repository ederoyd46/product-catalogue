use serde::Serialize;

pub mod inventory;
pub mod inventory_search;
pub mod product;
pub mod product_search;

pub trait DataTransferObject: Send + Sync + Serialize {
    fn is_valid(&self) -> bool;
    fn get_key(&self) -> String;
    fn get_metadata(&self) -> Vec<String>;
}
