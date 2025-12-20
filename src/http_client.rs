use crate::error::BotError;
use log::{debug, error};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_BOTSERVER_URL: &str = "https://localhost:8088";
const DEFAULT_TIMEOUT_SECS: u64 = 30;

#[derive(Clone)]
pub struct BotServerClient {
    client: Arc<reqwest::Client>,
    base_url: String,
}

impl BotServerClient {
    pub fn new(base_url: Option<String>) -> Self {
        Self::with_timeout(base_url, Duration::from_secs(DEFAULT_TIMEOUT_SECS))
    }

    pub fn with_timeout(base_url: Option<String>, timeout: Duration) -> Self {
        let url = base_url.unwrap_or_else(|| {
            std::env::var("BOTSERVER_URL").unwrap_or_else(|_| DEFAULT_BOTSERVER_URL.to_string())
        });

        let client = reqwest::Client::builder()
            .timeout(timeout)
            .user_agent(format!("BotLib/{}", env!("CARGO_PKG_VERSION")))
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client: Arc::new(client),
            base_url: url,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("GET {}", url);

        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    pub async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("POST {}", url);

        let response = self.client.post(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    pub async fn put<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("PUT {}", url);

        let response = self.client.put(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    pub async fn patch<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("PATCH {}", url);

        let response = self.client.patch(&url).json(body).send().await?;
        self.handle_response(response).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("DELETE {}", url);

        let response = self.client.delete(&url).send().await?;
        self.handle_response(response).await
    }

    pub async fn get_authorized<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        token: &str,
    ) -> Result<T, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("GET {} (authorized)", url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        self.handle_response(response).await
    }

    pub async fn post_authorized<T: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
        token: &str,
    ) -> Result<R, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("POST {} (authorized)", url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .json(body)
            .send()
            .await?;
        self.handle_response(response).await
    }

    pub async fn delete_authorized<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        token: &str,
    ) -> Result<T, BotError> {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("DELETE {} (authorized)", url);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        self.handle_response(response).await
    }

    pub async fn health_check(&self) -> bool {
        match self.get::<serde_json::Value>("/health").await {
            Ok(_) => true,
            Err(e) => {
                error!("Health check failed: {}", e);
                false
            }
        }
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, BotError> {
        let status = response.status();
        let status_code = status.as_u16();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("HTTP {} error: {}", status_code, error_text);
            return Err(BotError::http(status_code, error_text));
        }

        response.json().await.map_err(|e| {
            error!("Failed to parse response: {}", e);
            BotError::http(status_code, format!("Failed to parse response: {}", e))
        })
    }
}

impl std::fmt::Debug for BotServerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BotServerClient")
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = BotServerClient::new(Some("http://localhost:8080".to_string()));
        assert_eq!(client.base_url(), "http://localhost:8080");
    }

    #[test]
    fn test_client_default_url() {
        std::env::remove_var("BOTSERVER_URL");
        let client = BotServerClient::new(None);
        assert_eq!(client.base_url(), DEFAULT_BOTSERVER_URL);
    }

    #[test]
    fn test_client_with_timeout() {
        let client = BotServerClient::with_timeout(
            Some("http://test:9000".to_string()),
            Duration::from_secs(60),
        );
        assert_eq!(client.base_url(), "http://test:9000");
    }

    #[test]
    fn test_client_with_timeout_default_url() {
        std::env::remove_var("BOTSERVER_URL");
        let client = BotServerClient::with_timeout(None, Duration::from_secs(60));
        assert_eq!(client.base_url(), DEFAULT_BOTSERVER_URL);
    }

    #[test]
    fn test_client_debug() {
        let client = BotServerClient::new(Some("http://debug-test".to_string()));
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("BotServerClient"));
        assert!(debug_str.contains("http://debug-test"));
    }
}
