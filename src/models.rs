//! Common models shared across bot ecosystem
//!
//! Contains DTOs, API response types, and common structures.

use crate::message_types::MessageType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Standard API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Create a success response with data
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    /// Create a success response with message
    pub fn success_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message.into()),
        }
    }

    /// Create an error response
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
            message: None,
        }
    }
}

impl<T: Default> Default for ApiResponse<T> {
    fn default() -> Self {
        Self::success(T::default())
    }
}

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bot_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

impl Session {
    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            Utc::now() > expires
        } else {
            false
        }
    }
}

/// User message sent to bot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub bot_id: String,
    pub user_id: String,
    pub session_id: String,
    pub channel: String,
    pub content: String,
    pub message_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_name: Option<String>,
}

impl UserMessage {
    /// Create a new text message
    pub fn text(
        bot_id: impl Into<String>,
        user_id: impl Into<String>,
        session_id: impl Into<String>,
        channel: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            bot_id: bot_id.into(),
            user_id: user_id.into(),
            session_id: session_id.into(),
            channel: channel.into(),
            content: content.into(),
            message_type: MessageType::USER,
            media_url: None,
            timestamp: Utc::now(),
            context_name: None,
        }
    }
}

/// Suggestion for user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl Suggestion {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            context: None,
        }
    }

    pub fn with_context(text: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            context: Some(context.into()),
        }
    }
}

/// Bot response to user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotResponse {
    pub bot_id: String,
    pub user_id: String,
    pub session_id: String,
    pub channel: String,
    pub content: String,
    pub message_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_token: Option<String>,
    pub is_complete: bool,
    #[serde(default)]
    pub suggestions: Vec<Suggestion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_name: Option<String>,
    #[serde(default)]
    pub context_length: usize,
    #[serde(default)]
    pub context_max_length: usize,
}

impl BotResponse {
    /// Create a new bot response
    pub fn new(
        bot_id: impl Into<String>,
        session_id: impl Into<String>,
        user_id: impl Into<String>,
        content: impl Into<String>,
        channel: impl Into<String>,
    ) -> Self {
        Self {
            bot_id: bot_id.into(),
            user_id: user_id.into(),
            session_id: session_id.into(),
            channel: channel.into(),
            content: content.into(),
            message_type: MessageType::BOT_RESPONSE,
            stream_token: None,
            is_complete: true,
            suggestions: Vec::new(),
            context_name: None,
            context_length: 0,
            context_max_length: 0,
        }
    }

    /// Create a streaming response
    pub fn streaming(
        bot_id: impl Into<String>,
        session_id: impl Into<String>,
        user_id: impl Into<String>,
        channel: impl Into<String>,
        stream_token: impl Into<String>,
    ) -> Self {
        Self {
            bot_id: bot_id.into(),
            user_id: user_id.into(),
            session_id: session_id.into(),
            channel: channel.into(),
            content: String::new(),
            message_type: MessageType::BOT_RESPONSE,
            stream_token: Some(stream_token.into()),
            is_complete: false,
            suggestions: Vec::new(),
            context_name: None,
            context_length: 0,
            context_max_length: 0,
        }
    }

    /// Add suggestions to response
    pub fn with_suggestions(mut self, suggestions: Vec<Suggestion>) -> Self {
        self.suggestions = suggestions;
        self
    }
}

/// Attachment for media files in messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Type of attachment (image, audio, video, file, etc.)
    pub attachment_type: String,
    /// URL or path to the attachment
    pub url: String,
    /// MIME type of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File name if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response: ApiResponse<String> = ApiResponse::success("test".to_string());
        assert!(response.success);
        assert_eq!(response.data, Some("test".to_string()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<String> = ApiResponse::error("something went wrong");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("something went wrong".to_string()));
    }

    #[test]
    fn test_user_message_creation() {
        let msg = UserMessage::text("bot1", "user1", "sess1", "web", "Hello!");
        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.message_type, MessageType::USER);
    }

    #[test]
    fn test_bot_response_creation() {
        let response = BotResponse::new("bot1", "sess1", "user1", "Hi there!", "web");
        assert!(response.is_complete);
        assert_eq!(response.message_type, MessageType::BOT_RESPONSE);
    }
}
