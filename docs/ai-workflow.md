# AI Workflow Guide

This document provides instructions for AI assistants (Claude, GPT, etc.) working with this documentation repository.

## Overview

This repository uses a structured approach to architecture documentation:
- **Diagrams** are created in .drawio format (XML-based, editable)
- **Images** are exported as .png for viewing in markdown
- **Documentation** follows consistent templates

## Quick Start for AI Assistants

When asked to document software architecture using this template:

### 1. Understand the Structure

```
architecture/
├── logical-architecture.drawio    # High-level system view
├── physical-architecture.drawio   # Deployment view
├── layers/                        # Component details per tier
├── diagrams/                      # Data flow diagrams
└── infrastructure/                # Deployment topologies
```

### 2. Follow the Three-Tier Pattern

Most software architectures follow this pattern:
1. **API Tier** - Entry points, gateways, authentication
2. **Business Tier** - Application logic, services, processing
3. **Data Tier** - Databases, caches, storage

### 3. Use Consistent Naming

- Lowercase with hyphens: `api-tier.drawio`, `create-flow.drawio`
- File triplets: Every diagram needs `.drawio`, `.png`, and `.md` files

### 4. Apply the Color Scheme

Reference `architecture/README.md` for the complete color scheme:
- API Tier: Light blue background (`#dbeafe`), blue border (`#3b82f6`)
- Business Tier: Light purple background (`#e9d5ff`), purple border (`#8b5cf6`)
- Data Tier: Light green background (`#d1fae5`), green border (`#10b981`)

## Creating New Diagrams

### Step 1: Identify the Category

| Category | Directory | When to Use |
|----------|-----------|-------------|
| `layers/` | Single tier component details | Showing components within one tier |
| `diagrams/` | Cross-tier data flows | Showing how data moves through the system |
| `infrastructure/` | Deployment topologies | Showing how components are deployed |

### Step 2: Choose Naming Pattern

| Category | Pattern | Examples |
|----------|---------|----------|
| Layers | `{tier-name}.drawio` | `api-tier.drawio`, `cache-tier.drawio` |
| Flows | `{operation}-flow.drawio` | `create-flow.drawio`, `auth-flow.drawio` |
| Infrastructure | `{environment-type}.drawio` | `kubernetes.drawio`, `aws-deployment.drawio` |

### Step 3: Create the Diagram

Use the draw.io XML format. Example structure:

```xml
<mxfile host="app.diagrams.net">
  <diagram name="Page-1">
    <mxGraphModel dx="1434" dy="780" grid="1" gridSize="10">
      <root>
        <mxCell id="0" />
        <mxCell id="1" parent="0" />
        <!-- Components go here -->
      </root>
    </mxGraphModel>
  </diagram>
</mxfile>
```

### Step 4: Create Documentation

Use templates from `templates/` directory:
- `diagram-markdown.md` - Basic diagram documentation
- `layer-documentation.md` - For tier/layer diagrams
- `flow-documentation.md` - For data flow diagrams
- `infrastructure-documentation.md` - For deployment diagrams

## Documentation Structure

Every diagram's `.md` file should follow this pattern:

```markdown
# [Diagram Title]

![Image](./filename.png)

[View source: filename.drawio](./filename.drawio)

## Overview

[2-3 sentences describing what this diagram represents]

## Components

| Component | Technology | Purpose |
|-----------|------------|---------|
| [Name] | [Tech] | [Description] |

## [Flow Steps / Architecture Notes / Configuration]

[Detailed information specific to this diagram type]

## Related Diagrams

- [Related 1](../path/to/related.png) - Brief description
- [Related 2](../path/to/related.png) - Brief description
```

## Updating Existing Diagrams

1. **Read first**: Always read the existing `.drawio` file to understand current structure
2. **Make targeted changes**: Modify only what's needed
3. **Update documentation**: If components changed, update the `.md` file
4. **Export PNG**: Generate new PNG (or push to trigger GitHub Actions)
5. **Commit together**: Always commit `.drawio`, `.png`, and `.md` changes together

## Task Tracking

Use `docs/tasks/` for tracking documentation work:

```markdown
# Task: [Task Name]

## Status: [In Progress / Complete]

## Tasks

- [x] Completed task
- [ ] Pending task
- [ ] Another pending task

## Notes

[Additional context or decisions made]
```

## Common Patterns

### Adding a New Component

1. Determine which tier it belongs to
2. Add to the appropriate layer diagram
3. Update the logical architecture if it's a significant component
4. Add to component tables in relevant `.md` files
5. Update flow diagrams if it participates in data flows

### Documenting a New Flow

1. Create diagram in `diagrams/` folder
2. Show all tiers involved using swimlanes
3. Number the steps in the diagram
4. Document each step in the `.md` file
5. Link to related layer diagrams

### Adding Infrastructure Variant

1. Create diagram in `infrastructure/` folder
2. Show server/container layout
3. Include component counts and specifications
4. Document scaling considerations
5. Note limitations and recommended use cases

## Draw.io XML Tips

### Creating a Rounded Rectangle (Component)

```xml
<mxCell id="component-1" value="Component Name" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dbeafe;strokeColor=#3b82f6;" vertex="1" parent="1">
  <mxGeometry x="100" y="100" width="120" height="60" as="geometry" />
</mxCell>
```

### Creating a Database Cylinder

```xml
<mxCell id="db-1" value="Database" style="shape=cylinder3;whiteSpace=wrap;html=1;boundedLbl=1;backgroundOutline=1;size=15;fillColor=#d1fae5;strokeColor=#10b981;" vertex="1" parent="1">
  <mxGeometry x="100" y="100" width="60" height="80" as="geometry" />
</mxCell>
```

### Creating an Arrow Connection

```xml
<mxCell id="arrow-1" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;strokeColor=#374151;strokeWidth=2;" edge="1" parent="1" source="component-1" target="db-1">
  <mxGeometry relative="1" as="geometry" />
</mxCell>
```

### Creating a Container/Group

```xml
<mxCell id="group-1" value="API Tier" style="swimlane;whiteSpace=wrap;html=1;fillColor=#dbeafe;strokeColor=#3b82f6;" vertex="1" parent="1">
  <mxGeometry x="50" y="50" width="400" height="200" as="geometry" />
</mxCell>
```

## Best Practices

### Do

- Follow the established color scheme
- Use consistent naming conventions
- Create all three files (`.drawio`, `.png`, `.md`)
- Cross-reference related diagrams
- Keep diagrams focused on one concept
- Use swimlanes for multi-tier flow diagrams

### Don't

- Create orphan diagrams without documentation
- Use inconsistent colors or styles
- Mix multiple unrelated concepts in one diagram
- Forget to update the diagram index in `architecture/README.md`
- Leave placeholder text in final diagrams

## Example Prompts

### Creating a New Architecture Documentation

```
Using the frontend design plugin; and using ../architecture-docs-template as a
reference for the image design; we expect .drawio images.. we want to document
the MyProject architecture; high level logical and physical views. You will
want to look at ~/workspace/myproject for repositories and documentation.
```

### Adding a New Component

```
Add a Redis cache layer to the architecture documentation. It should appear in:
1. The logical architecture diagram
2. A new cache-tier.drawio in the layers folder
3. The read-flow diagram (as it will be used for caching reads)
```

### Documenting a New Flow

```
Document the authentication flow for the application. Create:
1. auth-flow.drawio showing the complete authentication process
2. auth-flow.md with detailed step-by-step documentation
3. Update the README to include the new flow
```
