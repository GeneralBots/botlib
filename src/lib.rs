//! BotLib - Shared library for General Bots
//!
//! This crate provides common types, utilities, and abstractions
//! shared between botserver and botui.
//!
//! # Features
//! - `database` - Database connection utilities (diesel)
//! - `http-client` - HTTP client for API calls
//! - `validation` - Request validation derive macros

pub mod branding;
pub mod error;
#[cfg(feature = "http-client")]
pub mod http_client;
pub mod message_types;
pub mod models;
pub mod version;

// Re-exports for convenience
pub use branding::{branding, init_branding, is_white_label, platform_name, platform_short, BrandingConfig};
pub use error::{BotError, BotResult};
pub use message_types::MessageType;
pub use models::{ApiResponse, BotResponse, Session, Suggestion, UserMessage};
pub use version::{
    get_botserver_version, init_version_registry, register_component, version_string,
    ComponentSource, ComponentStatus, ComponentVersion, VersionRegistry, BOTSERVER_VERSION,
};

#[cfg(feature = "http-client")]
pub use http_client::BotServerClient;
