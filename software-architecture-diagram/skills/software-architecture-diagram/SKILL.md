# Software Architecture Diagram Skill

Create professional software architecture documentation with .drawio diagrams following best practices for three-tier architecture.

## When to Use This Skill

Use this skill when the user asks to:
- Create architecture documentation or diagrams
- Document a software system's architecture
- Create .drawio diagrams for their project
- Document logical or physical architecture views
- Create data flow diagrams
- Document deployment or infrastructure

## Architecture Documentation Approach

### Three-Tier Architecture Pattern

Most software architectures follow this pattern:
1. **API Tier** - Entry points, gateways, authentication, rate limiting
2. **Business Tier** - Application logic, services, workers, orchestration
3. **Data Tier** - Databases, caches, storage, search indexes

### Directory Structure

Create this structure for architecture documentation:

```
architecture/
├── README.md                      # Color scheme & diagram index
├── logical-architecture.drawio    # High-level three-tier view
├── logical-architecture.png       # Exported image
├── physical-architecture.drawio   # Deployment view
├── physical-architecture.png      # Exported image
├── layers/                        # Tier-specific diagrams
│   ├── api-tier.drawio/png/md
│   ├── business-tier.drawio/png/md
│   └── data-tier.drawio/png/md
├── diagrams/                      # Data flow diagrams
│   ├── create-flow.drawio/png/md
│   └── read-flow.drawio/png/md
└── infrastructure/                # Deployment topologies
    ├── single-environment.drawio/png/md
    └── multi-environment.drawio/png/md
```

### File Triplets

Every diagram MUST have three files:
- `.drawio` - The editable source diagram (XML format)
- `.png` - The exported image for viewing
- `.md` - Documentation explaining the diagram

### Naming Conventions

Use lowercase with hyphens for all files:
- Layers: `{tier-name}.drawio` (e.g., `api-tier.drawio`)
- Flows: `{operation}-flow.drawio` (e.g., `create-flow.drawio`)
- Infrastructure: `{environment-type}.drawio` (e.g., `single-environment.drawio`)

## Color Scheme

Apply these colors consistently across all diagrams:

### Tier Colors (for grouping containers)

| Tier | Background | Border | Usage |
|------|------------|--------|-------|
| API/Gateway | `#dbeafe` | `#3b82f6` | Gateway, API, entry points |
| Business Logic | `#e9d5ff` | `#8b5cf6` | Services, workers, logic |
| Data/Storage | `#d1fae5` | `#10b981` | Databases, storage |
| Services/Support | `#f1f5f9` | `#64748b` | Monitoring, logging |
| Queue/Messaging | `#fce7f3` | `#ec4899` | Message queues, events |
| Cache | `#fef3c7` | `#f59e0b` | Redis, Memcached |
| External | `#fed7aa` | `#f97316` | Third-party, internet |

### Technology-Specific Colors

| Technology | Color | Hex |
|------------|-------|-----|
| PostgreSQL | Blue | `#336791` |
| Redis | Red | `#dc382d` |
| Elasticsearch | Yellow | `#fed10a` |
| Nginx | Green | `#059669` |
| RabbitMQ/Kafka | Pink | `#ec4899` |

### Connection Styles

| Type | Style | Color |
|------|-------|-------|
| Sync data flow | Solid arrow, 2px | `#374151` |
| Async data flow | Dashed arrow, 2px | `#8b5cf6` |
| Replication | Thick arrow, 3px | `#059669` |
| External | Dotted, 1px | `#9ca3af` |

## Draw.io XML Templates

### Component (Rounded Rectangle)

```xml
<mxCell id="component-1" value="Component Name&#xa;(Technology)" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dbeafe;strokeColor=#3b82f6;" vertex="1" parent="1">
  <mxGeometry x="100" y="100" width="140" height="70" as="geometry" />
</mxCell>
```

### Database (Cylinder)

```xml
<mxCell id="db-1" value="Database&#xa;(PostgreSQL)" style="shape=cylinder3;whiteSpace=wrap;html=1;boundedLbl=1;backgroundOutline=1;size=15;fillColor=#336791;strokeColor=#1e4a6e;fontColor=#ffffff;" vertex="1" parent="1">
  <mxGeometry x="100" y="100" width="120" height="90" as="geometry" />
</mxCell>
```

### Swimlane (Tier Container)

```xml
<mxCell id="tier-1" value="API TIER" style="swimlane;whiteSpace=wrap;html=1;fillColor=#dbeafe;strokeColor=#3b82f6;fontStyle=1;startSize=30;" vertex="1" parent="1">
  <mxGeometry x="50" y="50" width="900" height="150" as="geometry" />
</mxCell>
```

### Arrow Connection

```xml
<mxCell id="arrow-1" style="edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;strokeColor=#374151;strokeWidth=2;" edge="1" parent="1" source="component-1" target="db-1">
  <mxGeometry relative="1" as="geometry" />
</mxCell>
```

## Documentation Templates

### Layer Documentation Structure

```markdown
# [TIER_NAME] Tier

![TIER_NAME Architecture](./filename.png)

[View source: filename.drawio](./filename.drawio)

## Overview
[2-3 sentences about this tier's responsibility]

## Components

### [Component Name]

| Attribute | Value |
|-----------|-------|
| Technology | [Tech stack] |
| Purpose | [What it does] |
| Scales | [Horizontal/Vertical] |

**Features:**
- Feature 1
- Feature 2

## Request Flow
[ASCII or description of flow through this tier]

## Scaling
[How this tier scales]

## Monitoring
| Metric | Description | Alert Threshold |
|--------|-------------|-----------------|
| [Metric] | [Description] | [Threshold] |

## Related Diagrams
- [Link to related diagrams]
```

### Flow Documentation Structure

```markdown
# [OPERATION] Flow

![Flow](./filename.png)

[View source: filename.drawio](./filename.drawio)

## Overview
[Description of what this flow accomplishes]

## Flow Steps

### 1. [Step Name]
| Attribute | Value |
|-----------|-------|
| Component | [Which component] |
| Action | [What happens] |
| Input | [What is received] |
| Output | [What is produced] |

[Continue for each step...]

## Error Handling
| Scenario | Behavior | Response |
|----------|----------|----------|
| [Error] | [What happens] | [Response code] |

## Related Diagrams
- [Links]
```

## Export Instructions

After creating .drawio files, export to PNG:

**macOS:**
```bash
/Applications/draw.io.app/Contents/MacOS/draw.io -x -f png --scale 2 --border 10 -o output.png input.drawio
```

**Batch export:**
```bash
for f in $(find . -name "*.drawio"); do
  /Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o "${f%.drawio}.png" "$f"
done
```

## GitHub Actions for Auto-Export

Include this workflow for automatic PNG export:

```yaml
name: Export Draw.io Diagrams

on:
  push:
    paths:
      - '**/*.drawio'

jobs:
  export-drawio:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rlespinasse/drawio-export-action@v2
        with:
          format: png
          transparent: false
          border: 10
          output: .
      - run: |
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git config user.name "github-actions[bot]"
          git add -A
          git diff --staged --quiet || git commit -m "Auto-export: Update PNG from drawio"
          git push
```

## Best Practices

1. **Start with logical architecture** - Create the high-level three-tier view first
2. **Be consistent** - Use the same colors and styles throughout
3. **Document everything** - Every diagram needs accompanying markdown
4. **Cross-reference** - Link related diagrams in documentation
5. **Version control** - Commit both .drawio and .png files together
6. **Use swimlanes** - For flow diagrams spanning multiple tiers
7. **Number steps** - In flow diagrams, number each step clearly
