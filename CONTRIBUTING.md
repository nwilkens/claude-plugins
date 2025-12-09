# Contributing to Architecture Documentation

Thank you for contributing to this architecture documentation template.

## Quick Reference

### File Organization

Every diagram needs three files:
1. `.drawio` - The editable source diagram
2. `.png` - The exported image (auto-generated or manual)
3. `.md` - The documentation explaining the diagram

### Directories

| Directory | Purpose | Naming Pattern |
|-----------|---------|----------------|
| `architecture/` | High-level views | `{scope}-architecture.drawio` |
| `architecture/layers/` | Tier details | `{tier-name}.drawio` |
| `architecture/diagrams/` | Data flows | `{operation}-flow.drawio` |
| `architecture/infrastructure/` | Deployments | `{environment}.drawio` |

## Adding a New Diagram

### 1. Create the .drawio File

Use one of these tools:
- [app.diagrams.net](https://app.diagrams.net) (online)
- [VS Code Draw.io Extension](https://marketplace.visualstudio.com/items?itemName=hediet.vscode-drawio)
- [Desktop Application](https://www.diagrams.net/)

### 2. Follow the Color Scheme

See `architecture/README.md` for the complete color scheme:

| Tier | Background | Border |
|------|------------|--------|
| API/Gateway | `#dbeafe` | `#3b82f6` |
| Business Logic | `#e9d5ff` | `#8b5cf6` |
| Data/Storage | `#d1fae5` | `#10b981` |
| Services/Support | `#f1f5f9` | `#64748b` |

### 3. Create Documentation

Use templates from `templates/` directory:
- Copy the appropriate template
- Replace all placeholders
- Add specific details for your diagram

### 4. Export to PNG

Option A: Push and let GitHub Actions export automatically

Option B: Manual export:
```bash
# macOS
/Applications/draw.io.app/Contents/MacOS/draw.io -x -f png -o output.png input.drawio

# Linux
drawio -x -f png -o output.png input.drawio

# Windows
& "C:\Program Files\draw.io\draw.io.exe" -x -f png -o output.png input.drawio
```

### 5. Update the Index

Add your new diagram to `architecture/README.md` in the appropriate section.

## Updating Existing Diagrams

1. Edit the `.drawio` file
2. Export new `.png` (or push to trigger GitHub Actions)
3. Update the `.md` documentation if needed
4. Commit all changed files together

## Commit Guidelines

### Commit Message Format

```
type: Brief description

Longer explanation if needed.
```

Types:
- `docs`: Documentation changes
- `diagram`: New or updated diagram
- `fix`: Fixes to documentation or diagrams
- `template`: Changes to templates
- `ci`: Changes to GitHub Actions

### Examples

```
diagram: Add authentication flow diagram

Creates auth-flow.drawio showing the complete authentication
process from client request to session creation.
```

```
docs: Update API tier documentation

- Add rate limiting component
- Update component table
- Add monitoring metrics section
```

## Style Guidelines

### Diagrams

- Use consistent spacing (align to grid)
- Group related components visually
- Use swimlanes for multi-tier flows
- Include a title in the diagram
- Number steps in flow diagrams

### Documentation

- Start with an overview section
- Include component tables with Technology and Purpose
- Cross-reference related diagrams
- Use ASCII diagrams for text-based representation when helpful

### Naming

- Lowercase with hyphens: `api-tier`, `create-flow`
- Be descriptive but concise
- Match names across `.drawio`, `.png`, and `.md` files

## Pull Request Process

1. Create a branch for your changes
2. Make your changes following the guidelines above
3. Ensure all files are properly named and organized
4. Test that links and cross-references work
5. Submit a pull request with a clear description

## Questions?

If you're unsure about anything:
1. Check existing diagrams for patterns to follow
2. Review the templates in `templates/`
3. Open an issue for discussion
