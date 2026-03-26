use thiserror::Error;
use tonic::Status;

/// A specialized Result type for Xray API operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur while interacting with the Xray API.
#[derive(Error, Debug)]
pub enum Error {
    /// Errors originating from the underlying gRPC transport (e.g., connection failures).
    #[error("connection error: {0}")]
    Connection(#[from] tonic::transport::Error),

    /// gRPC status errors returned by the Xray-core API.
    #[error("gRPC status: {0}")]
    Status(#[from] Status),

    /// Errors related to configuration (e.g., invalid JSON or missing parameters).
    #[error("config error: {0}")]
    Config(String),

    /// Other unknown or unexpected errors.
    #[error("unknown error: {0}")]
    Unknown(String),
}
