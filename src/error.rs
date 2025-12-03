//! Common error types for BotLib
//!
//! Provides unified error handling across botserver and botui.

use thiserror::Error;

/// Result type alias using BotError
pub type BotResult<T> = Result<T, BotError>;

/// Common error types across the bot ecosystem
#[derive(Error, Debug)]
pub enum BotError {
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Database errors
    #[error("Database error: {0}")]
    Database(String),

    /// HTTP/Network errors
    #[error("HTTP error: {0}")]
    Http(String),

    /// Authentication/Authorization errors
    #[error("Auth error: {0}")]
    Auth(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found errors
    #[error("{0} not found")]
    NotFound(String),

    /// Internal errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic error wrapper
    #[error("{0}")]
    Other(String),
}

impl BotError {
    /// Create a config error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a database error
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Create an HTTP error
    pub fn http(msg: impl Into<String>) -> Self {
        Self::Http(msg.into())
    }

    /// Create an auth error
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a not found error
    pub fn not_found(entity: impl Into<String>) -> Self {
        Self::NotFound(entity.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl From<anyhow::Error> for BotError {
    fn from(err: anyhow::Error) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<String> for BotError {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

impl From<&str> for BotError {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
}

#[cfg(feature = "http-client")]
impl From<reqwest::Error> for BotError {
    fn from(err: reqwest::Error) -> Self {
        Self::Http(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = BotError::config("missing API key");
        assert_eq!(err.to_string(), "Configuration error: missing API key");
    }

    #[test]
    fn test_not_found_error() {
        let err = BotError::not_found("User");
        assert_eq!(err.to_string(), "User not found");
    }
}
