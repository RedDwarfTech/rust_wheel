use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyCustomError {
    #[error("Custom error with code {0}: {1}")]
    CustomError(i32, String),
}