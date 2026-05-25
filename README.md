# mcp-jira

Jira MCP Server — issues, projects, sprints, boards, comments, transitions, and JQL search for AI agents.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-jira/main/docs/assets/architecture.svg" alt="MCP Jira Architecture" width="800"/>
</p>

## Setup

```bash
export JIRA_URL="https://yourorg.atlassian.net"
export JIRA_EMAIL="you@example.com"
export JIRA_API_TOKEN="your-api-token"
```

Generate an API token at https://id.atlassian.com/manage-profile/security/api-tokens

## Tools (15)

| Tool | Description | Risk |
|------|-------------|------|
| list_projects | List all projects | read_only |
| get_project | Get project details | read_only |
| search_issues | JQL search | read_only |
| get_issue | Get issue details | read_only |
| create_issue | Create issue (Story, Bug, Task, Epic) | internal_write |
| update_issue | Update fields (summary, description, priority, labels) | internal_write |
| transition_issue | Move issue through workflow | internal_write |
| assign_issue | Assign to user | internal_write |
| add_comment | Add comment to issue | internal_write |
| list_comments | List comments on issue | read_only |
| get_transitions | Get available transitions | read_only |
| list_sprints | List sprints for a board | read_only |
| create_sprint | Create a new sprint | internal_write |
| add_to_sprint | Move issues to a sprint | internal_write |
| get_board | Get board details (Scrum/Kanban) | read_only |

## MCP Client Configuration

```json
{
  "mcpServers": {
    "jira": {
      "command": "mcp-jira",
      "env": {
        "JIRA_URL": "https://yourorg.atlassian.net",
        "JIRA_EMAIL": "you@example.com",
        "JIRA_API_TOKEN": "your-api-token"
      }
    }
  }
}
```

## Build

```bash
cargo build --release
```

## License

Apache-2.0
