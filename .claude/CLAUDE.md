# Claude Code Plugins - AI Instructions

This repository is a collection of Claude Code plugins for software development workflows.

## Available Plugins

| Plugin | Purpose |
|--------|---------|
| `postgresql-dba` | PostgreSQL performance analysis and DBA toolkit |
| `ratatui-tui` | Rust terminal UI development with Ratatui |
| `software-architecture-diagram` | Architecture documentation with .drawio diagrams |
| `triton` | Triton DataCenter infrastructure management |

## Plugin Structure

Each plugin follows this standard structure:
```
plugin-name/
├── .claude-plugin/
│   └── plugin.json
├── commands/
│   └── command-name.md
├── skills/
│   └── skill-name/
│       └── SKILL.md
└── README.md
```

## Commit Guidelines

- Create atomic commits after each change
- Include documentation with each commit
- Use descriptive commit messages
