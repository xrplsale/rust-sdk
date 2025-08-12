//! HTTP client for the XRPL.Sale API

use crate::{
    error::{Error, Result},
    services::{AnalyticsService, AuthService, InvestmentsService, ProjectsService, WebhooksService},
    webhook::WebhookSignatureValidator,
    Environment,
};
use reqwest::{header::HeaderMap, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use url::Url;

/// Configuration for the XRPL.Sale client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API key for authentication
    pub api_key: String,
    /// API environment
    pub environment: Environment,
    /// Custom base URL (optional)
    pub base_url: Option<String>,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: usize,
    /// Base delay between retries
    pub retry_delay: Duration,
    /// Webhook secret for signature verification
    pub webhook_secret: Option<String>,
    /// Enable debug logging
    pub debug: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            environment: Environment::Production,
            base_url: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            webhook_secret: None,
            debug: false,
        }
    }
}

/// Builder for creating a XRPL.Sale client
#[derive(Debug, Default)]
pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API key
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.config.api_key = api_key.into();
        self
    }

    /// Set the environment
    pub fn environment(mut self, environment: Environment) -> Self {
        self.config.environment = environment;
        self
    }

    /// Set a custom base URL
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.config.base_url = Some(base_url.into());
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the maximum retry attempts
    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    /// Set the retry delay
    pub fn retry_delay(mut self, retry_delay: Duration) -> Self {
        self.config.retry_delay = retry_delay;
        self
    }

    /// Set the webhook secret
    pub fn webhook_secret<S: Into<String>>(mut self, webhook_secret: S) -> Self {
        self.config.webhook_secret = Some(webhook_secret.into());
        self
    }

    /// Enable debug logging
    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    }

    /// Build the client
    pub fn build(self) -> Result<Client> {
        if self.config.api_key.is_empty() {
            return Err(Error::Configuration("API key is required".to_string()));
        }

        Client::with_config(self.config)
    }
}

/// Main client for interacting with the XRPL.Sale API
///
/// The client provides access to all platform services including projects,
/// investments, analytics, webhooks, and authentication.
#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<ClientConfig>,
    http_client: reqwest::Client,
    auth_token: Arc<tokio::sync::RwLock<Option<String>>>,
}

impl Client {
    /// Create a new client with the builder pattern
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Create a client with the given configuration
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("User-Agent", crate::user_agent().parse().unwrap());

        let http_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()
            .map_err(|e| Error::HttpClient(e.to_string()))?;

        Ok(Self {
            config: Arc::new(config),
            http_client,
            auth_token: Arc::new(tokio::sync::RwLock::new(None)),
        })
    }

    /// Get the base URL for API requests
    pub fn base_url(&self) -> &str {
        self.config
            .base_url
            .as_deref()
            .unwrap_or_else(|| self.config.environment.base_url())
    }

    /// Set the authentication token
    pub async fn set_auth_token<S: Into<String>>(&self, token: Option<S>) {
        let mut auth_token = self.auth_token.write().await;
        *auth_token = token.map(|t| t.into());
    }

    /// Get the authentication token
    pub async fn get_auth_token(&self) -> Option<String> {
        self.auth_token.read().await.clone()
    }

    /// Get the projects service
    pub fn projects(&self) -> ProjectsService {
        ProjectsService::new(self.clone())
    }

    /// Get the investments service
    pub fn investments(&self) -> InvestmentsService {
        InvestmentsService::new(self.clone())
    }

    /// Get the analytics service
    pub fn analytics(&self) -> AnalyticsService {
        AnalyticsService::new(self.clone())
    }

    /// Get the webhooks service
    pub fn webhooks(&self) -> WebhooksService {
        WebhooksService::new(self.clone())
    }

    /// Get the auth service
    pub fn auth(&self) -> AuthService {
        AuthService::new(self.clone())
    }

    /// Create a webhook signature validator
    pub fn webhook_validator(&self) -> Option<WebhookSignatureValidator> {
        self.config
            .webhook_secret
            .as_ref()
            .map(|secret| WebhookSignatureValidator::new(secret.clone()))
    }

    /// Make a GET request
    pub async fn get<T>(&self, path: &str, query: Option<&HashMap<String, String>>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut url = self.build_url(path)?;
        
        if let Some(query_params) = query {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in query_params {
                query_pairs.append_pair(key, value);
            }
        }

        let request = self.http_client.get(url);
        self.execute_request(request).await
    }

    /// Make a POST request
    pub async fn post<T, B>(&self, path: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.build_url(path)?;
        let mut request = self.http_client.post(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.execute_request(request).await
    }

    /// Make a PUT request
    pub async fn put<T, B>(&self, path: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.build_url(path)?;
        let mut request = self.http_client.put(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.execute_request(request).await
    }

    /// Make a PATCH request
    pub async fn patch<T, B>(&self, path: &str, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let url = self.build_url(path)?;
        let mut request = self.http_client.patch(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.execute_request(request).await
    }

    /// Make a DELETE request
    pub async fn delete<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path)?;
        let request = self.http_client.delete(url);
        self.execute_request(request).await
    }

    /// Execute an HTTP request with retry logic
    async fn execute_request<T>(&self, mut request: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // Add authentication headers
        if let Some(token) = self.get_auth_token().await {
            request = request.bearer_auth(token);
        } else {
            request = request.header("X-API-Key", &self.config.api_key);
        }

        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            let req = request
                .try_clone()
                .ok_or_else(|| Error::HttpClient("Failed to clone request".to_string()))?;

            match req.send().await {
                Ok(response) => {
                    if self.config.debug {
                        log::debug!(
                            "HTTP {} {} -> {}",
                            response.request().map(|r| r.method()).unwrap_or(&Method::GET),
                            response.request().map(|r| r.url()).unwrap().as_str(),
                            response.status()
                        );
                    }

                    return self.handle_response(response).await;
                }
                Err(e) => {
                    last_error = Some(Error::HttpClient(e.to_string()));

                    if attempt < self.config.max_retries {
                        let delay = self.config.retry_delay * 2_u32.pow(attempt as u32);
                        if self.config.debug {
                            log::debug!("Request failed, retrying in {:?}: {}", delay, e);
                        }
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| Error::HttpClient("Unknown error".to_string())))
    }

    /// Handle HTTP response
    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let url = response.url().clone();

        if status.is_success() {
            let text = response.text().await.map_err(|e| Error::HttpClient(e.to_string()))?;
            
            if text.is_empty() {
                // Handle empty responses for endpoints that return no content
                return serde_json::from_str("null").map_err(|e| Error::Parse(e.to_string()));
            }

            serde_json::from_str(&text).map_err(|e| {
                if self.config.debug {
                    log::debug!("Failed to parse response: {}", text);
                }
                Error::Parse(e.to_string())
            })
        } else {
            let text = response.text().await.unwrap_or_default();
            
            match status.as_u16() {
                400 => Err(Error::BadRequest(text)),
                401 => Err(Error::Unauthorized(text)),
                404 => Err(Error::NotFound(text)),
                429 => {
                    let retry_after = response
                        .headers()
                        .get("retry-after")
                        .and_then(|h| h.to_str().ok())
                        .and_then(|s| s.parse().ok());
                    
                    Err(Error::RateLimit { 
                        message: text, 
                        retry_after 
                    })
                }
                _ => Err(Error::Api {
                    status: status.as_u16(),
                    message: text,
                    url: url.to_string(),
                }),
            }
        }
    }

    /// Build a full URL from a path
    fn build_url(&self, path: &str) -> Result<Url> {
        let base = Url::parse(self.base_url())
            .map_err(|e| Error::Configuration(format!("Invalid base URL: {}", e)))?;
        
        base.join(path.trim_start_matches('/'))
            .map_err(|e| Error::Configuration(format!("Invalid path: {}", e)))
    }
}