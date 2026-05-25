# Jira MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-jira.svg)](https://crates.io/crates/mcp-jira)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Give your AI agents full Jira access — issues, projects, sprints, boards, comments, transitions, and JQL search. 15 tools over the Jira Cloud REST API v3 with enterprise governance.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-jira/main/docs/assets/architecture.svg" alt="MCP Jira Architecture" width="800"/>
</p>

## Tools (15)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `list_projects` | List all Jira projects | Read-only |
| `get_project` | Get project details | Read-only |
| `search_issues` | JQL search with fields | Read-only |
| `get_issue` | Get full issue details | Read-only |
| `list_comments` | List comments on an issue | Read-only |
| `get_transitions` | Get available workflow transitions | Read-only |
| `list_sprints` | List sprints for a board | Read-only |
| `get_board` | Get board details (Scrum/Kanban) | Read-only |
| `create_issue` | Create issue (story, bug, task, epic) | Internal write |
| `update_issue` | Update fields (summary, description, priority) | Internal write |
| `transition_issue` | Move issue through workflow | Internal write |
| `assign_issue` | Assign issue to a user | Internal write |
| `add_comment` | Add comment to an issue | Internal write |
| `create_sprint` | Create a new sprint | Internal write |
| `add_to_sprint` | Move issues into a sprint | Internal write |

## Installation

```bash
cargo install mcp-jira
```

Or build from source:

```bash
git clone https://github.com/zavora-ai/mcp-jira
cd mcp-jira
cargo build --release
```

## Configuration

### Environment Variables

| Variable | Required | Description |
|----------|:---:|-------------|
| `JIRA_URL` | ✅ | Jira Cloud URL (e.g. `https://yourorg.atlassian.net`) |
| `JIRA_EMAIL` | ✅ | Your Atlassian account email |
| `JIRA_API_TOKEN` | ✅ | API token from https://id.atlassian.com/manage-profile/security/api-tokens |

### Getting an API Token

1. Go to https://id.atlassian.com/manage-profile/security/api-tokens
2. Click **Create API token**
3. Name it "MCP Jira"
4. Copy the token

## Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "jira": {
      "command": "mcp-jira",
      "args": [],
      "env": {
        "JIRA_URL": "https://yourorg.atlassian.net",
        "JIRA_EMAIL": "you@company.com",
        "JIRA_API_TOKEN": "your-api-token"
      }
    }
  }
}
```

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "jira": {
      "command": "mcp-jira",
      "args": [],
      "env": {
        "JIRA_URL": "https://yourorg.atlassian.net",
        "JIRA_EMAIL": "you@company.com",
        "JIRA_API_TOKEN": "your-api-token"
      }
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "jira": {
      "command": "mcp-jira",
      "args": [],
      "env": {
        "JIRA_URL": "https://yourorg.atlassian.net",
        "JIRA_EMAIL": "you@company.com",
        "JIRA_API_TOKEN": "your-api-token"
      }
    }
  }
}
```

### Windsurf

Add to `~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "jira": {
      "command": "mcp-jira",
      "args": [],
      "env": {
        "JIRA_URL": "https://yourorg.atlassian.net",
        "JIRA_EMAIL": "you@company.com",
        "JIRA_API_TOKEN": "your-api-token"
      }
    }
  }
}
```

## Usage Examples

### Search issues
```
"Find all bugs in the BACKEND project that are unresolved"
→ calls search_issues with jql: "project = BACKEND AND type = Bug AND resolution = Unresolved"
```

### Create and assign
```
"Create a task in SCRUM to fix the login timeout, assign it to James"
→ calls create_issue → assign_issue
```

### Move through workflow
```
"Move SCRUM-5 to In Progress"
→ calls get_transitions → transition_issue
```

### Sprint management
```
"Add SCRUM-5 and SCRUM-6 to the current sprint"
→ calls list_sprints → add_to_sprint
```

### Comment on issue
```
"Add a comment to SCRUM-5 saying the fix is deployed"
→ calls add_comment
```

## MCP Server Manifest

```toml
server_id = "mcp_jira"
display_name = "Jira"
version = "1.0.0"
domain = "developer-tools"
risk_level = "medium"
writes_allowed = "gated"
transports = ["stdio"]
credentials = ["vault://jira-api-token"]
```

## Registry Compliance

This server implements the [ADK MCP SDK](https://crates.io/crates/adk-mcp-sdk) contract:

- **HealthCheck** — async health probe for registry monitoring
- **mcp-server.toml** — manifest declaring tools, risk classes, and credentials
- **Structured tracing** — `RUST_LOG` env-filter for observability

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)
