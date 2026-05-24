mod client;
mod server;

use client::JiraClient;
use rmcp::{ServiceExt, transport::stdio};
use server::JiraServer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("info".parse()?),
        )
        .init();

    let base_url = std::env::var("JIRA_URL").expect("JIRA_URL required");
    let email = std::env::var("JIRA_EMAIL").expect("JIRA_EMAIL required");
    let api_token = std::env::var("JIRA_API_TOKEN").expect("JIRA_API_TOKEN required");

    let client = Arc::new(JiraClient::new(base_url, email, api_token));
    let server = JiraServer { client };

    tracing::info!("mcp-jira starting on stdio");
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
