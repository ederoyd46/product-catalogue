mod config;
mod error;

pub use config::*;
pub use error::AppError;

/// Alias for a type-erased error type.
pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;
