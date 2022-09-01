pub mod product;
pub mod inventory;

pub trait DataTransferObject: Send + Sync {
    fn is_valid(&self) -> bool;
    fn get_key(&self) -> String;
    fn get_metadata(&self) -> Vec<String>;
}
