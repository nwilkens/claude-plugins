# Architecture Documentation Template - AI Instructions

This repository is a template for creating software architecture documentation with .drawio diagrams.

## Project Overview

- **Purpose**: Reusable template for AI-assisted architecture documentation
- **Diagram Format**: .drawio (XML-based, editable with diagrams.net)
- **Image Export**: Automatic via GitHub Actions, or manual CLI
- **Documentation**: Markdown files accompanying each diagram

## Key Files

- `README.md` - Main entry point and usage guide
- `architecture/README.md` - Color scheme and diagram conventions
- `docs/ai-workflow.md` - Detailed instructions for AI assistants
- `docs/manual-conversion.md` - Manual export instructions
- `templates/` - Documentation templates to follow

## Color Scheme Quick Reference

| Tier | Background | Border |
|------|------------|--------|
| API/Gateway | `#dbeafe` | `#3b82f6` |
| Business Logic | `#e9d5ff` | `#8b5cf6` |
| Data/Storage | `#d1fae5` | `#10b981` |
| Services/Support | `#f1f5f9` | `#64748b` |

## When Creating Diagrams

1. **Always create three files**: `.drawio`, `.png`, and `.md`
2. **Follow naming convention**: lowercase-with-hyphens
3. **Use the color scheme** from `architecture/README.md`
4. **Use templates** from `templates/` directory
5. **Update indexes** in README files when adding new diagrams

## Directory Structure

| Directory | Content Type |
|-----------|--------------|
| `architecture/` | High-level architecture diagrams |
| `architecture/layers/` | Tier-specific component diagrams |
| `architecture/diagrams/` | Data flow diagrams |
| `architecture/infrastructure/` | Deployment topology diagrams |
| `templates/` | Documentation templates |
| `docs/` | Guides and task tracking |

## Documentation Standards

Every diagram documentation file should include:
1. Title with embedded image
2. Link to source .drawio file
3. Overview section
4. Components table (Component, Technology, Purpose)
5. Related diagrams section

## Commit Guidelines

- Create atomic commits after each change
- Include documentation with each commit
- Test builds/exports after commits
- Use descriptive commit messages
