//! # XRPL.Sale Rust SDK
//!
//! Official Rust SDK for integrating with the XRPL.Sale platform - the native XRPL launchpad 
//! for token sales and project funding.
//!
//! ## Features
//!
//! - 🦀 **Modern Rust** - Built with idiomatic Rust patterns and zero-cost abstractions
//! - ⚡ **Full Async/Await** - Built on tokio with comprehensive async support  
//! - 🔐 **XRPL Wallet Authentication** - Wallet-based authentication support
//! - 📊 **Project Management** - Create, launch, and manage token sales
//! - 💰 **Investment Tracking** - Monitor investments and analytics
//! - 🔔 **Webhook Support** - Real-time event notifications with signature verification
//! - 📈 **Analytics & Reporting** - Comprehensive data insights
//! - 🛡️ **Type Safety** - Strongly typed API with comprehensive error handling
//! - 🔄 **Auto-retry Logic** - Resilient API calls with exponential backoff
//! - 🧩 **Framework Integration** - Optional integrations for Axum, Actix-web, and Warp
//! - 📝 **Rich Documentation** - Comprehensive docs with examples
//!
//! ## Quick Start
//!
//! ```rust
//! use xrplsale::{Client, Environment, CreateProjectRequest, ProjectTier};
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the client
//!     let client = Client::builder()
//!         .api_key("your-api-key")
//!         .environment(Environment::Production)
//!         .build()?;
//!
//!     // Create a new project
//!     let project = client.projects().create(CreateProjectRequest {
//!         name: "My DeFi Protocol".to_string(),
//!         description: "Revolutionary DeFi protocol on XRPL".to_string(),
//!         token_symbol: "MDP".to_string(),
//!         total_supply: "100000000".to_string(),
//!         tiers: vec![ProjectTier {
//!             tier: 1,
//!             price_per_token: "0.001".to_string(),
//!             total_tokens: "20000000".to_string(),
//!             ..Default::default()
//!         }],
//!         sale_start_date: chrono::Utc::now() + chrono::Duration::days(30),
//!         sale_end_date: chrono::Utc::now() + chrono::Duration::days(60),
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("Project created: {}", project.id);
//!     Ok(())
//! }
//! ```

use std::sync::Arc;

pub mod client;
pub mod error;
pub mod models;
pub mod services;
pub mod webhook;

#[cfg(feature = "axum-integration")]
pub mod integrations;

// Re-exports for convenience
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use models::*;
pub use webhook::{WebhookEvent, WebhookSignatureValidator};

/// XRPL.Sale API environments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    /// Production environment
    Production,
    /// Testnet environment for testing
    Testnet,
}

impl Environment {
    /// Get the base URL for this environment
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Production => "https://api.xrpl.sale/v1",
            Environment::Testnet => "https://api-testnet.xrpl.sale/v1",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Production
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Production => write!(f, "production"),
            Environment::Testnet => write!(f, "testnet"),
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Ok(Environment::Production),
            "testnet" | "test" => Ok(Environment::Testnet),
            _ => Err(Error::InvalidEnvironment(s.to_string())),
        }
    }
}

/// SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent string for API requests
pub fn user_agent() -> String {
    format!("XRPL.Sale-Rust-SDK/{}", VERSION)
}