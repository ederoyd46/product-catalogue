mod config;
mod error;

pub use config::*;
pub use error::AppError;
use std::collections::HashMap;

/// Alias for a type-erased error type.
pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;

pub trait Storable<T>: Send + Sync {
    fn is_valid(&self) -> bool;
    fn get_pk(&self) -> T;
    fn get_metadata(&self) -> T;
    fn to_database_value(&self) -> T;
}

pub trait Retrievable<T, Y>: Send + Sync {
    fn from_database_value(data: HashMap<String, T>) -> Option<Y>;
}
