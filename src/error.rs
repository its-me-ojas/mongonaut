use thiserror::Error;

#[derive(Debug,Error)]
pub enum AppError{
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("MongoDB error: {0}")]
    Mongo(#[from] mongodb::error::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T,AppError>;
