# XRPL.Sale Rust SDK

Official Rust SDK for integrating with the XRPL.Sale platform - the native XRPL launchpad for token sales and project funding.

[![Crates.io](https://img.shields.io/crates/v/xrplsale.svg)](https://crates.io/crates/xrplsale)
[![Documentation](https://docs.rs/xrplsale/badge.svg)](https://docs.rs/xrplsale)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🦀 **Modern Rust** - Built with idiomatic Rust patterns and zero-cost abstractions
- ⚡ **Full Async/Await** - Built on tokio with comprehensive async support  
- 🔐 **XRPL Wallet Authentication** - Wallet-based authentication support
- 📊 **Project Management** - Create, launch, and manage token sales
- 💰 **Investment Tracking** - Monitor investments and analytics
- 🔔 **Webhook Support** - Real-time event notifications with signature verification
- 📈 **Analytics & Reporting** - Comprehensive data insights
- 🛡️ **Type Safety** - Strongly typed API with comprehensive error handling
- 🔄 **Auto-retry Logic** - Resilient API calls with exponential backoff
- 🧩 **Framework Integration** - Optional integrations for Axum, Actix-web, and Warp
- 📝 **Rich Documentation** - Comprehensive docs with examples
- 🚀 **High Performance** - Zero-cost abstractions and efficient HTTP client
- 🔧 **Flexible Configuration** - Builder pattern with sensible defaults

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xrplsale = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

For web framework integrations:

```toml
[dependencies]
xrplsale = { version = "1.0", features = ["axum-integration"] }
# or
xrplsale = { version = "1.0", features = ["actix-integration"] }
# or
xrplsale = { version = "1.0", features = ["warp-integration"] }
```

## Quick Start

### Basic Usage

```rust
use xrplsale::{Client, Environment, CreateProjectRequest, ProjectTier};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = Client::builder()
        .api_key("your-api-key")
        .environment(Environment::Production)
        .build()?;

    // Create a new project
    let project = client.projects().create(CreateProjectRequest {
        name: "My DeFi Protocol".to_string(),
        description: "Revolutionary DeFi protocol on XRPL".to_string(),
        token_symbol: "MDP".to_string(),
        total_supply: "100000000".to_string(),
        tiers: vec![ProjectTier {
            tier: 1,
            price_per_token: "0.001".to_string(),
            total_tokens: "20000000".to_string(),
            ..Default::default()
        }],
        sale_start_date: chrono::Utc::now() + chrono::Duration::days(30),
        sale_end_date: chrono::Utc::now() + chrono::Duration::days(60),
        ..Default::default()
    }).await?;

    println!("Project created: {}", project.id);
    Ok(())
}
```

### Advanced Configuration

```rust
use xrplsale::{Client, Environment};
use std::time::Duration;

let client = Client::builder()
    .api_key("your-api-key")
    .environment(Environment::Production)
    .timeout(Duration::from_secs(60))
    .max_retries(5)
    .retry_delay(Duration::from_secs(2))
    .webhook_secret("your-webhook-secret")
    .debug(true)
    .build()?;
```

## Authentication

### XRPL Wallet Authentication

```rust
// Generate authentication challenge
let challenge = client.auth()
    .generate_challenge("rYourWalletAddress...")
    .await?;

// Sign the challenge with your wallet
// (implementation depends on your wallet library)
let signature = sign_message(&challenge.challenge)?;

// Authenticate
let auth_response = client.auth().authenticate(
    "rYourWalletAddress...",
    &signature,
    challenge.timestamp
).await?;

println!("Authentication successful: {}", auth_response.token);

// Set the auth token for subsequent requests
client.set_auth_token(Some(auth_response.token)).await;
```

## Core Services

### Projects Service

```rust
// List active projects
let projects = client.projects()
    .active(Some(1), Some(10))
    .await?;

// Get project details
let project = client.projects()
    .get("proj_abc123")
    .await?;

// Launch a project
client.projects()
    .launch("proj_abc123")
    .await?;

// Get project statistics
let stats = client.projects()
    .stats("proj_abc123")
    .await?;
println!("Total raised: {} XRP", stats.total_raised_xrp);

// Search projects
let results = client.projects()
    .search("DeFi", Some("active"), Some(1), Some(10))
    .await?;

// Get trending projects
let trending = client.projects()
    .trending(Some("24h"), Some(5))
    .await?;

// Stream all projects with automatic pagination
use futures::StreamExt;

let mut stream = client.projects().stream_all(Some("active"));
while let Some(project) = stream.next().await {
    match project {
        Ok(project) => println!("Project: {}", project.name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Investments Service

```rust
use xrplsale::CreateInvestmentRequest;

// Create an investment
let investment = client.investments().create(CreateInvestmentRequest {
    project_id: "proj_abc123".to_string(),
    amount_xrp: "100".to_string(),
    investor_account: "rInvestorAddress...".to_string(),
    ..Default::default()
}).await?;

// List investments for a project
let investments = client.investments()
    .get_by_project("proj_abc123", Some(1), Some(10))
    .await?;

// Get investor summary
let summary = client.investments()
    .get_investor_summary("rInvestorAddress...")
    .await?;

// Simulate an investment
let simulation = client.investments()
    .simulate("proj_abc123", "100")
    .await?;
println!("Expected tokens: {}", simulation.token_amount);
```

### Analytics Service

```rust
use chrono::{Utc, Duration};

// Get platform analytics
let analytics = client.analytics()
    .get_platform_analytics()
    .await?;
println!("Total raised: {} XRP", analytics.total_raised_xrp);

// Get project-specific analytics
let start_date = Utc::now() - Duration::days(30);
let end_date = Utc::now();

let project_analytics = client.analytics()
    .get_project_analytics("proj_abc123", Some(start_date), Some(end_date))
    .await?;

// Get market trends
let trends = client.analytics()
    .get_market_trends("30d")
    .await?;

// Export data
let export = client.analytics()
    .export("projects", "csv", Some(start_date), Some(end_date))
    .await?;
println!("Download URL: {}", export.download_url);
```

## Webhook Integration

### Basic Webhook Handling

```rust
use xrplsale::WebhookEvent;

// Verify and parse webhook
fn handle_webhook(payload: &str, signature: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Verify signature
    let validator = xrplsale::WebhookSignatureValidator::new(secret.to_string());
    if !validator.verify(payload, signature) {
        return Err("Invalid signature".into());
    }

    // Parse event
    let event: WebhookEvent = serde_json::from_str(payload)?;

    match event.event_type.as_str() {
        "investment.created" => handle_investment_created(event.data)?,
        "project.launched" => handle_project_launched(event.data)?,
        "tier.completed" => handle_tier_completed(event.data)?,
        _ => println!("Unknown event type: {}", event.event_type),
    }

    Ok(())
}

fn handle_investment_created(data: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("New investment: {}", data);
    // Process the investment
    Ok(())
}
```

### Axum Integration

```rust
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use xrplsale::{Client, WebhookEvent};

#[derive(Clone)]
struct AppState {
    xrpl_client: Client,
}

async fn webhook_handler(
    State(state): State<AppState>,
    payload: String,
) -> Result<StatusCode, StatusCode> {
    // Verify webhook signature
    if let Some(validator) = state.xrpl_client.webhook_validator() {
        // Get signature from headers in real implementation
        let signature = ""; // Extract from X-XRPL-Sale-Signature header
        
        if !validator.verify(&payload, signature) {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // Parse webhook event
    let event: WebhookEvent = serde_json::from_str(&payload)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Handle event
    match event.event_type.as_str() {
        "investment.created" => {
            println!("New investment: {:?}", event.data);
            // Process investment
        }
        "project.launched" => {
            println!("Project launched: {:?}", event.data);
            // Handle project launch
        }
        _ => {
            println!("Unknown event: {}", event.event_type);
        }
    }

    Ok(StatusCode::OK)
}

async fn get_projects(State(state): State<AppState>) -> Result<Json<Vec<xrplsale::Project>>, StatusCode> {
    let projects = state.xrpl_client
        .projects()
        .active(Some(1), Some(10))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(projects.data.unwrap_or_default()))
}

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .api_key("your-api-key")
        .environment(xrplsale::Environment::Production)
        .build()
        .expect("Failed to create client");

    let app_state = AppState {
        xrpl_client: client,
    };

    let app = Router::new()
        .route("/projects", get(get_projects))
        .route("/webhooks/xrplsale", post(webhook_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Actix-web Integration

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use xrplsale::{Client, WebhookEvent};

struct AppState {
    xrpl_client: Client,
}

async fn webhook_handler(
    data: web::Data<AppState>,
    payload: String,
) -> Result<HttpResponse> {
    // Verify and process webhook
    if let Some(validator) = data.xrpl_client.webhook_validator() {
        // Signature verification logic here
    }

    let event: WebhookEvent = serde_json::from_str(&payload)?;
    
    match event.event_type.as_str() {
        "investment.created" => {
            // Handle investment created
        }
        "project.launched" => {
            // Handle project launched
        }
        _ => {}
    }

    Ok(HttpResponse::Ok().finish())
}

async fn get_projects(data: web::Data<AppState>) -> Result<HttpResponse> {
    let projects = data.xrpl_client
        .projects()
        .active(Some(1), Some(10))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(projects.data.unwrap_or_default()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::builder()
        .api_key("your-api-key")
        .environment(xrplsale::Environment::Production)
        .build()
        .expect("Failed to create client");

    let app_state = web::Data::new(AppState {
        xrpl_client: client,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/projects", web::get().to(get_projects))
            .route("/webhooks/xrplsale", web::post().to(webhook_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Error Handling

```rust
use xrplsale::{Error, Result};

async fn example() -> Result<()> {
    let client = Client::builder()
        .api_key("your-api-key")
        .build()?;

    match client.projects().get("invalid-id").await {
        Ok(project) => println!("Found project: {}", project.name),
        Err(Error::NotFound(_)) => println!("Project not found"),
        Err(Error::Unauthorized(_)) => println!("Authentication failed"),
        Err(Error::BadRequest(msg)) => println!("Validation error: {}", msg),
        Err(Error::RateLimit { retry_after, .. }) => {
            if let Some(retry_after) = retry_after {
                println!("Rate limit exceeded. Retry after: {} seconds", retry_after);
            }
        }
        Err(e) => println!("Other error: {}", e),
    }

    Ok(())
}
```

## Configuration

### Environment Variables

```bash
export XRPLSALE_API_KEY="your-api-key"
export XRPLSALE_ENVIRONMENT="production"
export XRPLSALE_WEBHOOK_SECRET="your-webhook-secret"
```

### Configuration File Support

Enable the `config-support` feature:

```toml
[dependencies]
xrplsale = { version = "1.0", features = ["config-support"] }
```

```rust
use xrplsale::ClientConfig;

// Load from config file
let config: ClientConfig = config::Config::builder()
    .add_source(config::File::with_name("xrplsale"))
    .add_source(config::Environment::with_prefix("XRPLSALE"))
    .build()?
    .try_deserialize()?;

let client = Client::with_config(config)?;
```

## Async Streaming

```rust
use futures::StreamExt;

// Stream all active projects
let mut stream = client.projects().stream_all(Some("active"));

while let Some(result) = stream.next().await {
    match result {
        Ok(project) => {
            println!("Processing project: {}", project.name);
            // Process project asynchronously
        }
        Err(e) => {
            eprintln!("Error fetching project: {}", e);
            break;
        }
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::builder()
            .api_key("test-key")
            .environment(Environment::Testnet)
            .build()
            .expect("Failed to create client");

        assert_eq!(client.base_url(), "https://api-testnet.xrpl.sale/v1");
    }

    #[tokio::test]
    async fn test_projects_list() {
        // Use mockito or wiremock for HTTP mocking
        let client = Client::builder()
            .api_key("test-key")
            .base_url("http://localhost:1234")
            .build()
            .unwrap();

        // Test with mocked server
    }
}
```

## Examples

Check out the [examples directory](https://github.com/xrplsale/rust-sdk/tree/main/examples) for complete sample applications:

- **Basic Usage** - Simple CLI integration
- **Axum Web Server** - REST API with webhook handling
- **Actix-web Application** - High-performance web service
- **Tokio Console App** - Background processing with streaming
- **Webhook Server** - Dedicated webhook processing service

## Building and Testing

```bash
# Build the library
cargo build

# Run tests
cargo test

# Run tests with all features
cargo test --all-features

# Build documentation
cargo doc --open

# Run examples
cargo run --example basic_usage
cargo run --example axum_webhook --features axum-integration

# Run with different TLS backends
cargo build --features native-tls --no-default-features
```

## Performance

The SDK is designed for high performance:

- **Zero-cost abstractions** - Minimal runtime overhead
- **Efficient HTTP client** - Based on reqwest with connection pooling
- **Async streaming** - Memory-efficient pagination with async streams
- **Configurable timeouts** - Fine-tuned request timeouts and retries
- **Connection reuse** - HTTP/2 and connection pooling for optimal performance

## Benchmarks

```bash
cargo bench
```

## Support

- 📖 [Documentation](https://docs.rs/xrplsale)
- 💬 [Discord Community](https://discord.gg/xrpl-sale)
- 🐛 [Issue Tracker](https://github.com/xrplsale/rust-sdk/issues)
- 📧 [Email Support](mailto:developers@xrpl.sale)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- [XRPL.Sale Platform](https://xrpl.sale)
- [API Documentation](https://xrpl.sale/docs/api)
- [Other SDKs](https://xrpl.sale/docs/developers/sdk-downloads)
- [GitHub Organization](https://github.com/xrplsale)

---

Made with ❤️ by the XRPL.Sale team