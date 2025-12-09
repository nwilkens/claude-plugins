# Software Architecture Diagram Plugin

A Claude Code plugin for creating professional software architecture documentation with .drawio diagrams.

## Features

- **Three-tier architecture templates** - API, Business, and Data tier patterns
- **Consistent color scheme** - Professional, readable diagrams
- **File triplets** - Every diagram has `.drawio`, `.png`, and `.md` files
- **Documentation templates** - Layer, flow, and infrastructure documentation
- **GitHub Actions** - Automatic PNG export on push
- **Best practices** - Industry-standard patterns and conventions

## Installation

```bash
# Add marketplace (if using local marketplace)
/plugin marketplace add ./path-to-marketplace

# Install plugin
/plugin install software-architecture-diagram@marketplace-name

# Or enable directly
/plugin enable software-architecture-diagram
```

## Usage

### Using the Skill

The skill is automatically invoked when you ask Claude to:
- Create architecture documentation
- Document a software system
- Create .drawio diagrams
- Document infrastructure or deployments

Example prompts:
```
Create architecture documentation for my e-commerce application
Document the authentication flow for my API
Create a deployment diagram for my Kubernetes setup
```

### Using the Slash Command

```
/document-architecture
```

This command guides you through creating comprehensive architecture documentation.

## Included Templates

### Directory Structure

```
architecture/
├── README.md                      # Color scheme & index
├── logical-architecture.*         # High-level view
├── physical-architecture.*        # Deployment view
├── layers/                        # Tier diagrams
│   ├── api-tier.*
│   ├── business-tier.*
│   └── data-tier.*
├── diagrams/                      # Flow diagrams
│   ├── create-flow.*
│   └── read-flow.*
└── infrastructure/                # Deployment diagrams
    ├── single-environment.*
    └── multi-environment.*
```

### Color Scheme

| Tier | Background | Border |
|------|------------|--------|
| API/Gateway | `#dbeafe` | `#3b82f6` |
| Business | `#e9d5ff` | `#8b5cf6` |
| Data | `#d1fae5` | `#10b981` |
| Support | `#f1f5f9` | `#64748b` |
| Queue | `#fce7f3` | `#ec4899` |
| Cache | `#fef3c7` | `#f59e0b` |

## Manual Export

Export .drawio to PNG locally:

**macOS:**
```bash
/Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o output.png input.drawio
```

**Windows:**
```powershell
& "C:\Program Files\draw.io\draw.io.exe" -x -f png -o output.png input.drawio
```

**Linux:**
```bash
drawio -x -f png -o output.png input.drawio
```

## Reference Examples

The `skills/software-architecture-diagram/architecture/` directory contains complete working examples:
- Logical and physical architecture diagrams
- Layer diagrams for all three tiers
- Create and read flow diagrams
- Single and multi-environment deployments

## License

MIT License
