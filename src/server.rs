use crate::client::JiraClient;
use adk_mcp_sdk::{HealthCheck, HealthStatus};
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;
use std::sync::Arc;

// --- Input types ---

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProjectKeyInput {
    /// Project key (e.g. "PROJ")
    pub key: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchIssuesInput {
    /// JQL query string
    pub jql: String,
    /// Max results (default 50)
    #[serde(default = "default_50")]
    pub max_results: u32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct IssueKeyInput {
    /// Issue key (e.g. "PROJ-123")
    pub key: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateIssueInput {
    /// Project key
    pub project_key: String,
    /// Issue summary
    pub summary: String,
    /// Issue type: Story, Bug, Task, Epic
    pub issue_type: String,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateIssueInput {
    /// Issue key (e.g. "PROJ-123")
    pub key: String,
    /// Fields to update as JSON (e.g. {"summary": "New title", "priority": {"name": "High"}})
    pub fields: serde_json::Value,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TransitionIssueInput {
    /// Issue key
    pub key: String,
    /// Transition ID (get from get_transitions)
    pub transition_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AssignIssueInput {
    /// Issue key
    pub key: String,
    /// Atlassian account ID of the assignee
    pub account_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddCommentInput {
    /// Issue key
    pub key: String,
    /// Comment body text
    pub body: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BoardIdInput {
    /// Board ID
    pub board_id: u64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateSprintInput {
    /// Board ID
    pub board_id: u64,
    /// Sprint name
    pub name: String,
    /// Start date (ISO 8601, e.g. "2024-01-15T00:00:00.000Z")
    #[serde(default)]
    pub start_date: Option<String>,
    /// End date (ISO 8601)
    #[serde(default)]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AddToSprintInput {
    /// Sprint ID
    pub sprint_id: u64,
    /// Issue keys to move into the sprint
    pub issue_keys: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}

fn default_50() -> u32 { 50 }

// --- Server ---

#[derive(Clone)]
pub struct JiraServer {
    pub client: Arc<JiraClient>,
}

#[tool_router(server_handler)]
impl JiraServer {
    #[tool(description = "List all Jira projects")]
    async fn list_projects(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match self.client.list_projects().await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get project details by key")]
    async fn get_project(&self, Parameters(i): Parameters<ProjectKeyInput>) -> String {
        match self.client.get_project(&i.key).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Search issues using JQL")]
    async fn search_issues(&self, Parameters(i): Parameters<SearchIssuesInput>) -> String {
        match self.client.search_issues(&i.jql, i.max_results).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get issue details by key")]
    async fn get_issue(&self, Parameters(i): Parameters<IssueKeyInput>) -> String {
        match self.client.get_issue(&i.key).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a new issue (Story, Bug, Task, or Epic)")]
    async fn create_issue(&self, Parameters(i): Parameters<CreateIssueInput>) -> String {
        match self.client.create_issue(&i.project_key, &i.summary, &i.issue_type, i.description.as_deref()).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Update issue fields (summary, description, priority, labels)")]
    async fn update_issue(&self, Parameters(i): Parameters<UpdateIssueInput>) -> String {
        match self.client.update_issue(&i.key, &i.fields).await {
            Ok(_) => format!("Issue {} updated", i.key),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Transition an issue through workflow states (To Do → In Progress → Done)")]
    async fn transition_issue(&self, Parameters(i): Parameters<TransitionIssueInput>) -> String {
        match self.client.transition_issue(&i.key, &i.transition_id).await {
            Ok(_) => format!("Issue {} transitioned", i.key),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Assign an issue to a user")]
    async fn assign_issue(&self, Parameters(i): Parameters<AssignIssueInput>) -> String {
        match self.client.assign_issue(&i.key, &i.account_id).await {
            Ok(_) => format!("Issue {} assigned", i.key),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Add a comment to an issue")]
    async fn add_comment(&self, Parameters(i): Parameters<AddCommentInput>) -> String {
        match self.client.add_comment(&i.key, &i.body).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List comments on an issue")]
    async fn list_comments(&self, Parameters(i): Parameters<IssueKeyInput>) -> String {
        match self.client.list_comments(&i.key).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get available workflow transitions for an issue")]
    async fn get_transitions(&self, Parameters(i): Parameters<IssueKeyInput>) -> String {
        match self.client.get_transitions(&i.key).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "List sprints for a board")]
    async fn list_sprints(&self, Parameters(i): Parameters<BoardIdInput>) -> String {
        match self.client.list_sprints(i.board_id).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Create a new sprint on a board")]
    async fn create_sprint(&self, Parameters(i): Parameters<CreateSprintInput>) -> String {
        match self.client.create_sprint(i.board_id, &i.name, i.start_date.as_deref(), i.end_date.as_deref()).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Move issues to a sprint")]
    async fn add_to_sprint(&self, Parameters(i): Parameters<AddToSprintInput>) -> String {
        match self.client.add_to_sprint(i.sprint_id, &i.issue_keys).await {
            Ok(_) => format!("Issues added to sprint {}", i.sprint_id),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[tool(description = "Get board details (Scrum/Kanban)")]
    async fn get_board(&self, Parameters(i): Parameters<BoardIdInput>) -> String {
        match self.client.get_board(i.board_id).await {
            Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
            Err(e) => format!("Error: {e}"),
        }
    }
}

#[async_trait::async_trait]
impl HealthCheck for JiraServer {
    async fn check_health(&self) -> HealthStatus {
        let start = std::time::Instant::now();
        let healthy = self.client.health_check().await;
        HealthStatus {
            healthy,
            message: if healthy { None } else { Some("Jira API unreachable".into()) },
            latency_ms: Some(start.elapsed().as_millis() as u64),
        }
    }
}
