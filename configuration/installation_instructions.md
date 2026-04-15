## Getting Started

1. [Create a GitHub Personal Access Token](https://github.com/settings/tokens/new?description=zed-mcp-server-github&scopes=repo) with `repo` permissions.
2. Add your token to the `github_personal_access_token` setting below.

## GitHub Enterprise

Set `github_host` to your GitHub Enterprise Server or Enterprise Cloud URL.
For more information, see [GitHub MCP with Enterprise Server](https://github.com/github/github-mcp-server#github-enterprise-server-and-enterprise-cloud-with-data-residency-ghecom).

## Toolset Configuration

By default, the server enables: `context`, `repos`, `issues`, `pull_requests`, and `users`.

- Use `toolsets` to enable specific groups (e.g., `"repos,issues,actions"`).
- Use `tools` to cherry-pick individual tools (e.g., `"get_file_contents,issue_read"`).
- Use `"all"` to enable every available toolset.

See the full [toolset documentation](https://github.com/github/github-mcp-server#tool-configuration) for details.

## Additional Options

- **read_only**: Prevents all write operations.
- **dynamic_toolsets**: Lets the LLM discover and enable toolsets on-the-fly.
- **insiders**: Enables experimental features and tools.
- **lockdown_mode**: Filters content from users without push access (useful for enterprise).
- **pre_release**: Uses pre-release versions of the GitHub MCP Server binary.