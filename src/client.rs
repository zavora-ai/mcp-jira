use base64::Engine;
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct JiraClient {
    http: Client,
    base_url: String,
    auth_header: String,
}

impl JiraClient {
    pub fn new(base_url: String, email: String, api_token: String) -> Self {
        let creds = format!("{email}:{api_token}");
        let auth_header = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(creds)
        );
        Self {
            http: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            auth_header,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    async fn get(&self, path: &str) -> anyhow::Result<Value> {
        let resp = self
            .http
            .get(self.url(path))
            .header("Authorization", &self.auth_header)
            .header("Accept", "application/json")
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }

    async fn post(&self, path: &str, body: &Value) -> anyhow::Result<Value> {
        let resp = self
            .http
            .post(self.url(path))
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        let text = resp.text().await?;
        if text.is_empty() {
            Ok(Value::Null)
        } else {
            Ok(serde_json::from_str(&text)?)
        }
    }

    async fn put(&self, path: &str, body: &Value) -> anyhow::Result<Value> {
        let resp = self
            .http
            .put(self.url(path))
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        let text = resp.text().await?;
        if text.is_empty() {
            Ok(Value::Null)
        } else {
            Ok(serde_json::from_str(&text)?)
        }
    }

    pub async fn list_projects(&self) -> anyhow::Result<Value> {
        self.get("/rest/api/3/project").await
    }

    pub async fn get_project(&self, key: &str) -> anyhow::Result<Value> {
        self.get(&format!("/rest/api/3/project/{key}")).await
    }

    pub async fn search_issues(&self, jql: &str, max_results: u32) -> anyhow::Result<Value> {
        // Jira deprecated POST /search. New endpoint is GET /search/jql.
        // reqwest re-encodes commas in query params, so we use Command for exact URL control.
        let url = format!("{}/rest/api/3/search/jql?jql={}&maxResults={}&fields=summary,status,assignee,priority,issuetype",
            self.base_url,
            jql.replace(' ', "%20").replace('=', "%3D"),
            max_results);
        let output = tokio::process::Command::new("curl")
            .args(["-s", "-H", &format!("Authorization: {}", self.auth_header), &url])
            .output().await?;
        if output.status.success() {
            Ok(serde_json::from_slice(&output.stdout)?)
        } else {
            anyhow::bail!("Jira search failed: {}", String::from_utf8_lossy(&output.stderr))
        }
    }

    pub async fn get_issue(&self, key: &str) -> anyhow::Result<Value> {
        self.get(&format!("/rest/api/3/issue/{key}")).await
    }

    pub async fn create_issue(&self, project_key: &str, summary: &str, issue_type: &str, description: Option<&str>) -> anyhow::Result<Value> {
        let mut fields = serde_json::json!({
            "project": { "key": project_key },
            "summary": summary,
            "issuetype": { "name": issue_type }
        });
        if let Some(desc) = description {
            fields["description"] = serde_json::json!({
                "type": "doc",
                "version": 1,
                "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": desc }] }]
            });
        }
        self.post("/rest/api/3/issue", &serde_json::json!({ "fields": fields })).await
    }

    pub async fn update_issue(&self, key: &str, fields: &Value) -> anyhow::Result<Value> {
        self.put(&format!("/rest/api/3/issue/{key}"), &serde_json::json!({ "fields": fields })).await
    }

    pub async fn transition_issue(&self, key: &str, transition_id: &str) -> anyhow::Result<Value> {
        let body = serde_json::json!({ "transition": { "id": transition_id } });
        self.post(&format!("/rest/api/3/issue/{key}/transitions"), &body).await
    }

    pub async fn assign_issue(&self, key: &str, account_id: &str) -> anyhow::Result<Value> {
        let body = serde_json::json!({ "accountId": account_id });
        self.put(&format!("/rest/api/3/issue/{key}/assignee"), &body).await
    }

    pub async fn add_comment(&self, key: &str, body_text: &str) -> anyhow::Result<Value> {
        let body = serde_json::json!({
            "body": {
                "type": "doc",
                "version": 1,
                "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": body_text }] }]
            }
        });
        self.post(&format!("/rest/api/3/issue/{key}/comment"), &body).await
    }

    pub async fn list_comments(&self, key: &str) -> anyhow::Result<Value> {
        self.get(&format!("/rest/api/3/issue/{key}/comment")).await
    }

    pub async fn get_transitions(&self, key: &str) -> anyhow::Result<Value> {
        self.get(&format!("/rest/api/3/issue/{key}/transitions")).await
    }

    pub async fn list_sprints(&self, board_id: u64) -> anyhow::Result<Value> {
        self.get(&format!("/rest/agile/1.0/board/{board_id}/sprint")).await
    }

    pub async fn create_sprint(&self, board_id: u64, name: &str, start_date: Option<&str>, end_date: Option<&str>) -> anyhow::Result<Value> {
        let mut body = serde_json::json!({ "name": name, "originBoardId": board_id });
        if let Some(s) = start_date { body["startDate"] = Value::String(s.to_string()); }
        if let Some(e) = end_date { body["endDate"] = Value::String(e.to_string()); }
        self.post("/rest/agile/1.0/sprint", &body).await
    }

    pub async fn add_to_sprint(&self, sprint_id: u64, issue_keys: &[String]) -> anyhow::Result<Value> {
        let body = serde_json::json!({ "issues": issue_keys });
        self.post(&format!("/rest/agile/1.0/sprint/{sprint_id}/issue"), &body).await
    }

    pub async fn get_board(&self, board_id: u64) -> anyhow::Result<Value> {
        self.get(&format!("/rest/agile/1.0/board/{board_id}")).await
    }

    pub async fn health_check(&self) -> bool {
        self.get("/rest/api/3/myself").await.is_ok()
    }
}
