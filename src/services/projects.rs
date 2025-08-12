//! Projects service for managing token sale projects

use crate::{
    client::Client,
    error::Result,
    models::{
        CreateProjectRequest, Investment, PaginatedResponse, Project, ProjectStats, ProjectTier,
        UpdateProjectRequest,
    },
};
use std::collections::HashMap;

/// Service for managing token sale projects
///
/// This service provides methods for creating, updating, launching, and managing
/// token sale projects on the XRPL.Sale platform. It also includes functionality
/// for retrieving project statistics, investors, and tier information.
#[derive(Debug, Clone)]
pub struct ProjectsService {
    client: Client,
}

impl ProjectsService {
    /// Create a new projects service
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// List all projects with optional filtering and pagination
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by project status
    /// * `page` - Page number (1-based)
    /// * `limit` - Number of items per page
    /// * `sort_by` - Field to sort by
    /// * `sort_order` - Sort order (asc or desc)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use xrplsale::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().api_key("test").build()?;
    /// let projects = client.projects().list(
    ///     Some("active"),
    ///     Some(1),
    ///     Some(10),
    ///     Some("created_at"),
    ///     Some("desc")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        status: Option<&str>,
        page: Option<u32>,
        limit: Option<u32>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<PaginatedResponse<Project>> {
        let mut query = HashMap::new();

        if let Some(status) = status {
            query.insert("status".to_string(), status.to_string());
        }
        if let Some(page) = page {
            query.insert("page".to_string(), page.to_string());
        }
        if let Some(limit) = limit {
            query.insert("limit".to_string(), limit.to_string());
        }
        if let Some(sort_by) = sort_by {
            query.insert("sort_by".to_string(), sort_by.to_string());
        }
        if let Some(sort_order) = sort_order {
            query.insert("sort_order".to_string(), sort_order.to_string());
        }

        let query = if query.is_empty() { None } else { Some(&query) };
        self.client.get("/projects", query).await
    }

    /// Get active projects
    pub async fn active(&self, page: Option<u32>, limit: Option<u32>) -> Result<PaginatedResponse<Project>> {
        self.list(Some("active"), page, limit, None, None).await
    }

    /// Get upcoming projects
    pub async fn upcoming(&self, page: Option<u32>, limit: Option<u32>) -> Result<PaginatedResponse<Project>> {
        self.list(Some("upcoming"), page, limit, None, None).await
    }

    /// Get completed projects
    pub async fn completed(&self, page: Option<u32>, limit: Option<u32>) -> Result<PaginatedResponse<Project>> {
        self.list(Some("completed"), page, limit, None, None).await
    }

    /// Get a specific project by ID
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    ///
    /// # Example
    ///
    /// ```rust
    /// # use xrplsale::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().api_key("test").build()?;
    /// let project = client.projects().get("proj_abc123").await?;
    /// println!("Project: {}", project.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, project_id: &str) -> Result<Project> {
        self.client.get(&format!("/projects/{}", project_id), None).await
    }

    /// Create a new project
    ///
    /// # Arguments
    ///
    /// * `request` - Project creation data
    ///
    /// # Example
    ///
    /// ```rust
    /// # use xrplsale::{Client, CreateProjectRequest, ProjectTier};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().api_key("test").build()?;
    /// let project = client.projects().create(CreateProjectRequest {
    ///     name: "My DeFi Protocol".to_string(),
    ///     description: "Revolutionary DeFi protocol on XRPL".to_string(),
    ///     token_symbol: "MDP".to_string(),
    ///     total_supply: "100000000".to_string(),
    ///     tiers: vec![ProjectTier {
    ///         tier: 1,
    ///         price_per_token: "0.001".to_string(),
    ///         total_tokens: "20000000".to_string(),
    ///         ..Default::default()
    ///     }],
    ///     sale_start_date: chrono::Utc::now() + chrono::Duration::days(30),
    ///     sale_end_date: chrono::Utc::now() + chrono::Duration::days(60),
    ///     ..Default::default()
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: CreateProjectRequest) -> Result<Project> {
        self.client.post("/projects", Some(&request)).await
    }

    /// Update an existing project
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    /// * `request` - Project update data
    pub async fn update(&self, project_id: &str, request: UpdateProjectRequest) -> Result<Project> {
        self.client.patch(&format!("/projects/{}", project_id), Some(&request)).await
    }

    /// Launch a project (make it active)
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    pub async fn launch(&self, project_id: &str) -> Result<Project> {
        self.client.post(&format!("/projects/{}/launch", project_id), None::<&()>).await
    }

    /// Pause a project
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    pub async fn pause(&self, project_id: &str) -> Result<Project> {
        self.client.post(&format!("/projects/{}/pause", project_id), None::<&()>).await
    }

    /// Resume a paused project
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    pub async fn resume(&self, project_id: &str) -> Result<Project> {
        self.client.post(&format!("/projects/{}/resume", project_id), None::<&()>).await
    }

    /// Cancel a project
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    pub async fn cancel(&self, project_id: &str) -> Result<Project> {
        self.client.post(&format!("/projects/{}/cancel", project_id), None::<&()>).await
    }

    /// Get project statistics
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    pub async fn stats(&self, project_id: &str) -> Result<ProjectStats> {
        self.client.get(&format!("/projects/{}/stats", project_id), None).await
    }

    /// Get project investors
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    /// * `page` - Page number (1-based)
    /// * `limit` - Number of items per page
    pub async fn investors(
        &self,
        project_id: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<Investment>> {
        let mut query = HashMap::new();

        if let Some(page) = page {
            query.insert("page".to_string(), page.to_string());
        }
        if let Some(limit) = limit {
            query.insert("limit".to_string(), limit.to_string());
        }

        let query = if query.is_empty() { None } else { Some(&query) };
        self.client.get(&format!("/projects/{}/investors", project_id), query).await
    }

    /// Get project tiers
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    pub async fn tiers(&self, project_id: &str) -> Result<Vec<ProjectTier>> {
        self.client.get(&format!("/projects/{}/tiers", project_id), None).await
    }

    /// Update project tiers
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID
    /// * `tiers` - New tier configuration
    pub async fn update_tiers(&self, project_id: &str, tiers: Vec<ProjectTier>) -> Result<Vec<ProjectTier>> {
        let body = serde_json::json!({ "tiers": tiers });
        self.client.put(&format!("/projects/{}/tiers", project_id), Some(&body)).await
    }

    /// Search projects
    ///
    /// # Arguments
    ///
    /// * `query` - Search query
    /// * `status` - Filter by status
    /// * `page` - Page number (1-based)
    /// * `limit` - Number of items per page
    pub async fn search(
        &self,
        query: &str,
        status: Option<&str>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<Project>> {
        let mut params = HashMap::new();
        params.insert("q".to_string(), query.to_string());

        if let Some(status) = status {
            params.insert("status".to_string(), status.to_string());
        }
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(limit) = limit {
            params.insert("limit".to_string(), limit.to_string());
        }

        self.client.get("/projects/search", Some(&params)).await
    }

    /// Get featured projects
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of projects to return
    pub async fn featured(&self, limit: Option<u32>) -> Result<Vec<Project>> {
        let mut query = HashMap::new();

        if let Some(limit) = limit {
            query.insert("limit".to_string(), limit.to_string());
        }

        let query = if query.is_empty() { None } else { Some(&query) };
        let response: PaginatedResponse<Project> = self.client.get("/projects/featured", query).await?;
        Ok(response.data.unwrap_or_default())
    }

    /// Get trending projects
    ///
    /// # Arguments
    ///
    /// * `period` - Time period (24h, 7d, 30d)
    /// * `limit` - Maximum number of projects to return
    pub async fn trending(&self, period: Option<&str>, limit: Option<u32>) -> Result<Vec<Project>> {
        let mut query = HashMap::new();

        if let Some(period) = period {
            query.insert("period".to_string(), period.to_string());
        }
        if let Some(limit) = limit {
            query.insert("limit".to_string(), limit.to_string());
        }

        let query = if query.is_empty() { None } else { Some(&query) };
        let response: PaginatedResponse<Project> = self.client.get("/projects/trending", query).await?;
        Ok(response.data.unwrap_or_default())
    }

    /// Get all projects with automatic pagination
    ///
    /// This method automatically handles pagination and returns an async stream
    /// of all projects matching the given criteria.
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by project status
    ///
    /// # Example
    ///
    /// ```rust
    /// # use xrplsale::Client;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder().api_key("test").build()?;
    /// let mut stream = client.projects().stream_all(Some("active"));
    /// 
    /// while let Some(project) = stream.next().await {
    ///     match project {
    ///         Ok(project) => println!("Project: {}", project.name),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn stream_all(&self, status: Option<&str>) -> impl futures::Stream<Item = Result<Project>> + '_ {
        use futures::stream::{self, StreamExt, TryStreamExt};

        let status = status.map(|s| s.to_string());
        
        stream::unfold((1u32, false), move |mut state| async move {
            let (page, done) = state;
            
            if done {
                return None;
            }

            let result = self.list(status.as_deref(), Some(page), Some(50), None, None).await;
            
            match result {
                Ok(response) => {
                    let has_more = response.pagination.as_ref()
                        .map(|p| p.page < p.total_pages)
                        .unwrap_or(false);
                    
                    state.0 = page + 1;
                    state.1 = !has_more;
                    
                    let projects = response.data.unwrap_or_default();
                    Some((stream::iter(projects.into_iter().map(Ok)), state))
                }
                Err(e) => {
                    state.1 = true; // Stop on error
                    Some((stream::iter(vec![Err(e)]), state))
                }
            }
        })
        .flat_map(|s| s)
    }
}