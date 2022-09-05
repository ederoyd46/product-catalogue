use serde::Serialize;

pub mod product;
pub mod inventory;

pub trait DataTransferObject: Send + Sync + Serialize {
    fn is_valid(&self) -> bool;
    fn get_key(&self) -> String;
    fn get_metadata(&self) -> Vec<String>;
}
