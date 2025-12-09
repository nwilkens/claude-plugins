# Claude Code Plugins Repository

A collection of Claude Code plugins for software development workflows.

## Available Plugins

### software-architecture-diagram

Create professional software architecture documentation with .drawio diagrams.

**Features:**
- Three-tier architecture templates (API, Business, Data)
- Consistent color scheme for professional diagrams
- File triplets: `.drawio`, `.png`, and `.md` for every diagram
- Documentation templates for layers, flows, and infrastructure
- GitHub Actions for automatic PNG export
- Best practices and conventions

**Location:** `software-architecture-diagram/`

[View plugin documentation](./software-architecture-diagram/README.md)

## Installation

### Using as a Marketplace

Add this repository as a Claude Code marketplace:

```bash
/plugin marketplace add nwilkens/claude-plugins
```

Then install individual plugins:

```bash
/plugin install software-architecture-diagram@claude-plugins
```

### Manual Installation

Copy the desired plugin directory to your Claude Code plugins location.

## Plugin Structure

Each plugin follows the Claude Code plugin format:

```
plugin-name/
├── .claude-plugin/
│   └── plugin.json          # Plugin metadata
├── commands/                 # Slash commands
│   └── command-name.md
├── skills/                   # Skills (auto-invoked)
│   └── skill-name/
│       └── SKILL.md
└── README.md                 # Plugin documentation
```

## Creating New Plugins

1. Create a new directory with your plugin name
2. Add `.claude-plugin/plugin.json` with metadata
3. Add commands in `commands/` (optional)
4. Add skills in `skills/` (optional)
5. Document in `README.md`

## Template Files (Legacy)

The root directory also contains the original architecture documentation template files for reference:
- `architecture/` - Example diagrams and documentation
- `templates/` - Documentation templates
- `docs/` - Guides and workflows

These can be used directly or as reference for the plugin.

## License

MIT License
