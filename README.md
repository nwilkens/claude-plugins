# Architecture Documentation Template

A reusable template for AI-assisted software architecture documentation using .drawio diagrams with automated PNG export.

## Quick Start

### For AI Assistants (Claude, GPT, etc.)

When asked to document software architecture, use this template:

```
Using the frontend design plugin; and using ../architecture-docs-template as a
reference for the image design; we expect .drawio images.. we want to document
the [YOUR_PROJECT] architecture; high level logical and physical views.
You will want to look at [SOURCE_CODE_PATH] for repositories and documentation.
```

See [AI Workflow Guide](./docs/ai-workflow.md) for detailed instructions on:
- Creating new diagrams following established patterns
- Documenting components using templates
- Maintaining consistency with the color scheme
- Understanding the three-tier architecture model

## Overview

This template provides a structured approach for documenting software architecture:

- **Diagrams** are created in .drawio format (XML-based, editable)
- **Images** are automatically exported as .png via GitHub Actions
- **Documentation** follows consistent markdown templates

### Key Features

- **Automated PNG Export**: GitHub Actions converts .drawio to .png on every push
- **Consistent Color Scheme**: Pre-defined colors for architectural tiers
- **Documentation Templates**: Ready-to-use templates for layers, flows, and infrastructure
- **AI-Ready**: Instructions optimized for AI assistant workflows

## Architecture Overview

### Logical Architecture
![Logical Architecture](./architecture/logical-architecture.png)

### Physical Architecture
![Physical Architecture](./architecture/physical-architecture.png)

## Documentation Structure

```
architecture-docs-template/
├── .github/workflows/          # Automated drawio → PNG conversion
├── architecture/
│   ├── README.md               # Color scheme and diagram index
│   ├── logical-architecture.*  # High-level three-tier view
│   ├── physical-architecture.* # Deployment view
│   ├── layers/                 # Tier-specific component diagrams
│   │   ├── api-tier.*
│   │   ├── business-tier.*
│   │   └── data-tier.*
│   ├── diagrams/               # Data flow diagrams
│   │   ├── create-flow.*
│   │   └── read-flow.*
│   └── infrastructure/         # Deployment topology diagrams
│       ├── single-environment.*
│       └── multi-environment.*
├── docs/
│   ├── ai-workflow.md          # AI assistant instructions
│   ├── manual-conversion.md    # Manual drawio export guide
│   └── tasks/                  # Task tracking
└── templates/                  # Documentation templates
    ├── diagram-markdown.md
    ├── layer-documentation.md
    ├── flow-documentation.md
    └── infrastructure-documentation.md
```

## Quick Links

### High-Level Views
| Diagram | Description | Source |
|---------|-------------|--------|
| [Logical Architecture](./architecture/logical-architecture.png) | Three-tier architecture overview | [.drawio](./architecture/logical-architecture.drawio) |
| [Physical Architecture](./architecture/physical-architecture.png) | Deployment view | [.drawio](./architecture/physical-architecture.drawio) |

### Layer Diagrams
| Layer | Description | Documentation |
|-------|-------------|---------------|
| [API Tier](./architecture/layers/api-tier.png) | Gateway and API components | [Details](./architecture/layers/api-tier.md) |
| [Business Tier](./architecture/layers/business-tier.png) | Business logic components | [Details](./architecture/layers/business-tier.md) |
| [Data Tier](./architecture/layers/data-tier.png) | Data storage components | [Details](./architecture/layers/data-tier.md) |

### Flow Diagrams
| Flow | Description | Documentation |
|------|-------------|---------------|
| [Create Flow](./architecture/diagrams/create-flow.png) | Write/Create operation path | [Details](./architecture/diagrams/create-flow.md) |
| [Read Flow](./architecture/diagrams/read-flow.png) | Read operation path | [Details](./architecture/diagrams/read-flow.md) |

### Infrastructure Diagrams
| Deployment | Description | Documentation |
|------------|-------------|---------------|
| [Single Environment](./architecture/infrastructure/single-environment.png) | Minimal deployment | [Details](./architecture/infrastructure/single-environment.md) |
| [Multi-Environment](./architecture/infrastructure/multi-environment.png) | High availability deployment | [Details](./architecture/infrastructure/multi-environment.md) |

## How to Use This Template

### 1. Clone or Copy

```bash
git clone https://github.com/your-org/architecture-docs-template.git my-project-docs
cd my-project-docs
rm -rf .git
git init
```

### 2. Replace Placeholders

Search for `[PROJECT_NAME]` and other placeholders throughout the template:
- `[PROJECT_NAME]` - Your project name
- `[COMPONENT_NAME]` - Specific component names
- `[TECHNOLOGY]` - Technology stack details

### 3. Customize Diagrams

1. Open .drawio files with [diagrams.net](https://app.diagrams.net) or VS Code extension
2. Replace generic components with your architecture
3. Follow the color scheme in [architecture/README.md](./architecture/README.md)
4. Push changes - PNG files are auto-generated

### 4. Update Documentation

Use templates in `templates/` directory to document your components.

## Editing Diagrams

### Tools

1. **Online**: [app.diagrams.net](https://app.diagrams.net) (File -> Open From -> Device)
2. **VS Code**: [Draw.io Integration](https://marketplace.visualstudio.com/items?itemName=hediet.vscode-drawio) extension
3. **Desktop**: [diagrams.net](https://www.diagrams.net/) application

### Manual Export to PNG

See [Manual Conversion Guide](./docs/manual-conversion.md) for platform-specific instructions.

**Quick reference (macOS):**
```bash
# Single file
/Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o output.png input.drawio

# Batch export all diagrams
for f in $(find . -name "*.drawio"); do
  /Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o "${f%.drawio}.png" "$f"
done
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on:
- Adding new diagrams
- Updating documentation
- Following the color scheme
- Commit conventions

## License

This template is available under the MIT License. Feel free to use, modify, and distribute.
