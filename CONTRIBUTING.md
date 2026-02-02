# Contributing to Claude Code Plugins

Thank you for contributing to this plugin collection.

## Plugin Structure

Every plugin follows this standard structure:

```
plugin-name/
├── .claude-plugin/
│   └── plugin.json          # Plugin metadata
├── commands/                 # Slash commands (optional)
│   └── command-name.md
├── skills/                   # Skills (optional)
│   └── skill-name/
│       └── SKILL.md
└── README.md                 # Plugin documentation
```

## Adding a New Plugin

### 1. Create the Plugin Directory

Create a new directory with your plugin name (lowercase with hyphens):

```bash
mkdir my-plugin-name
```

### 2. Create plugin.json

Create `.claude-plugin/plugin.json` with metadata:

```json
{
  "name": "my-plugin-name",
  "description": "Brief description of what the plugin does",
  "version": "1.0.0",
  "author": {
    "name": "your-username"
  },
  "keywords": ["relevant", "keywords"]
}
```

### 3. Add Commands (Optional)

Commands are invoked via slash commands (e.g., `/my-command`).

Create `commands/my-command.md`:

```markdown
# Command Name

Description of what the command does.

## Usage

Instructions for using the command.
```

### 4. Add Skills (Optional)

Skills are auto-invoked based on context.

Create `skills/my-skill/SKILL.md`:

```markdown
# Skill Name

Description of when and how to use this skill.

## Patterns

Examples and patterns for the skill.
```

### 5. Create README.md

Document your plugin with:
- Overview of the plugin's purpose
- Features list
- Usage instructions
- Examples

### 6. Register in Marketplace

Add your plugin to `.claude-plugin/marketplace.json`:

```json
{
  "name": "my-plugin-name",
  "source": "./my-plugin-name",
  "description": "Brief description"
}
```

## Commit Guidelines

### Commit Message Format

```
type: Brief description

Longer explanation if needed.
```

Types:
- `feat`: New plugin or feature
- `fix`: Bug fixes
- `docs`: Documentation changes
- `refactor`: Code restructuring
- `chore`: Maintenance tasks

### Examples

```
feat: Add kubernetes plugin for cluster management

Provides commands for common kubectl operations and
deployment patterns.
```

```
docs: Update postgresql-dba README with examples

- Add query optimization examples
- Include index strategy recommendations
```

## Pull Request Process

1. Create a branch for your changes
2. Follow the plugin structure guidelines
3. Test your plugin locally
4. Ensure documentation is complete
5. Submit a pull request with a clear description

## Questions?

If you're unsure about anything:
1. Check existing plugins for patterns to follow
2. Open an issue for discussion
