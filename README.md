# GitHub MCP Server Extension for Zed

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

This extension integrates [GitHub MCP Server](https://github.com/github/github-mcp-server) as a context server for [Zed's](https://zed.dev) [Agent Panel](https://zed.dev/docs/ai/overview).

## Features

- **Repository Management** — Browse code, search files, analyze commits, and explore project structure.
- **Issues & PRs** — Create, update, and manage issues and pull requests.
- **CI/CD & Actions** — Monitor GitHub Actions workflows, analyze build failures, and manage releases.
- **Code Security** — Review code scanning alerts, Dependabot alerts, and secret scanning findings.
- **Discussions & Notifications** — Access discussions, manage notifications, and track team activity.
- **Toolset Control** — Enable only the tools you need for faster, more focused AI interactions.
- **Read-Only Mode** — Restrict to read-only operations for safe exploration.

## Installation

Navigate to **Zed** > **Extensions** and search for `GitHub MCP Server`.

Or use the command palette ([macOS](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json#L581), [Linux](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json#L459)) to search `extensions`.

## Setup

You'll need to [create a Personal Access Token](https://github.com/settings/tokens/new?description=zed-mcp-server-github&scopes=repo) with `repo` permissions.

Add the following to your Zed settings:

```json
"context_servers": {
  "mcp-server-github": {
    "settings": {
      "github_personal_access_token": "<YOUR_PAT_HERE>"
    }
  }
}
```

## Settings Reference

| Setting | Type | Required | Description |
|---------|------|----------|-------------|
| `github_personal_access_token` | `string` | **Yes** | Your GitHub PAT (`ghp_*` or `github_pat_*`) |
| `github_host` | `string` | No | GitHub Enterprise Server URL (e.g., `https://ghe.example.com/`) |
| `toolsets` | `string` | No | Comma-separated toolsets to enable (e.g., `"repos,issues,actions"`) |
| `tools` | `string` | No | Comma-separated individual tools (e.g., `"get_file_contents,issue_read"`) |
| `read_only` | `boolean` | No | Prevent all write operations |
| `dynamic_toolsets` | `boolean` | No | Let the LLM discover and enable toolsets on-the-fly |
| `insiders` | `boolean` | No | Enable experimental features |
| `lockdown_mode` | `boolean` | No | Filter content from non-collaborators |
| `pre_release` | `boolean` | No | Use pre-release versions of the MCP server binary |

### Full Configuration Example

```json
"context_servers": {
  "mcp-server-github": {
    "settings": {
      "github_personal_access_token": "<YOUR_PAT_HERE>",
      "github_host": "https://ghe.example.com/",
      "toolsets": "repos,issues,pull_requests,actions",
      "read_only": false,
      "dynamic_toolsets": true,
      "insiders": false,
      "lockdown_mode": false,
      "pre_release": false
    }
  }
}
```

### Available Toolsets

| Toolset | Default | Description |
|---------|---------|-------------|
| `context` | ✅ | Current user and GitHub context |
| `repos` | ✅ | Repository browsing, search, branches, commits, releases |
| `issues` | ✅ | Issue management, search, labels, sub-issues |
| `pull_requests` | ✅ | PR management, reviews, diffs, merge |
| `users` | ✅ | User search and profiles |
| `actions` | | GitHub Actions workflows, jobs, logs |
| `code_security` | | Code scanning alerts |
| `copilot` | | Copilot coding agent integration |
| `dependabot` | | Dependabot alerts |
| `discussions` | | GitHub Discussions |
| `gists` | | Gist management |
| `git` | | Low-level Git operations |
| `labels` | | Label management |
| `notifications` | | Notification management |
| `orgs` | | Organization tools |
| `projects` | | GitHub Projects |
| `secret_protection` | | Secret scanning alerts |
| `security_advisories` | | Security advisory tools |
| `stargazers` | | Star/unstar repositories |

Use `"all"` to enable everything, or `"default"` for the default set.

## GitHub Enterprise

Set `github_host` to your GitHub Enterprise Server or Enterprise Cloud URL:

```json
"context_servers": {
  "mcp-server-github": {
    "settings": {
      "github_personal_access_token": "<YOUR_PAT_HERE>",
      "github_host": "https://ghe.example.com/"
    }
  }
}
```

For more details, see the [upstream enterprise documentation](https://github.com/github/github-mcp-server#github-enterprise-server-and-enterprise-cloud-with-data-residency-ghecom).

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "missing `github_personal_access_token` setting" | Add your PAT to the settings — see [Setup](#setup) |
| Token appears invalid | Ensure it starts with `ghp_` (classic) or `github_pat_` (fine-grained) |
| Tools not appearing in Agent Panel | Check that the extension is installed and your token has `repo` scope |
| GitHub Enterprise not connecting | Ensure `github_host` uses `https://` prefix |
| Too many tools cluttering context | Use `toolsets` or `tools` to limit what's enabled |
| Want to prevent accidental writes | Set `read_only` to `true` |

## License

[MIT](LICENSE)