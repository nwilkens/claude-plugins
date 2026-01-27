# Ratatui Official Guide Index

High-level index of every guide from the official Ratatui documentation. Use `web_fetch` to retrieve full content on-demand when needed.

## Getting Started

### Installation
**URL:** https://ratatui.rs/installation/
**Topics:** Adding ratatui to Cargo.toml, crossterm backend setup, first app
**When to fetch:** User asks about installation, setup, or getting started

### Hello World Tutorial
**URL:** https://ratatui.rs/tutorials/hello-world/
**Topics:** Basic app setup, terminal initialization, event loop, rendering
**When to fetch:** User wants step-by-step tutorial or building their first app

### Counter Tutorial
**URL:** https://ratatui.rs/tutorials/counter-app/
**Topics:** State management, handling events, incrementing counters
**When to fetch:** User wants to learn about state and events

### JSON Editor Tutorial
**URL:** https://ratatui.rs/tutorials/json-editor/
**Topics:** Complex app structure, multiple components, file I/O
**When to fetch:** User building complex multi-component apps

## Core Concepts

### Widgets
**URL:** https://ratatui.rs/concepts/widgets/
**Topics:** Built-in widgets, Widget trait, StatefulWidget, rendering widgets
**When to fetch:** Questions about widgets, creating custom widgets, widget patterns

### Layout
**URL:** https://ratatui.rs/concepts/layout/
**Topics:** Layout struct, Constraints, Direction, splitting areas, nested layouts
**When to fetch:** Layout questions, arranging widgets, constraint systems

### Rendering
**URL:** https://ratatui.rs/concepts/rendering/
**Topics:** Frame, render cycle, immediate mode rendering, double buffering
**When to fetch:** Understanding render cycle, frame management

### Backends
**URL:** https://ratatui.rs/concepts/backends/
**Topics:** Crossterm, Termion, Termwiz backends, backend selection
**When to fetch:** Questions about terminal backends, cross-platform support

### Event Handling
**URL:** https://ratatui.rs/concepts/event-handling/
**Topics:** crossterm events, KeyEvent, MouseEvent, event polling
**When to fetch:** Handling keyboard/mouse input, event patterns

## Styling

### Colors
**URL:** https://ratatui.rs/concepts/colors/
**Topics:** Color enum, RGB, indexed colors, terminal color support
**When to fetch:** Color usage, RGB support, terminal compatibility

### Styles
**URL:** https://ratatui.rs/concepts/styles/
**Topics:** Style struct, Modifier, Stylize trait, applying styles
**When to fetch:** Styling widgets, text formatting, modifiers

### Text
**URL:** https://ratatui.rs/concepts/text/
**Topics:** Span, Line, Text types, styled text, text alignment
**When to fetch:** Text rendering, multiline text, styled spans

## API Reference

### Widgets Reference
**URL:** https://docs.rs/ratatui/latest/ratatui/widgets/index.html
**Topics:** Complete widget API documentation
**When to fetch:** Looking for specific widget documentation, method signatures

### Layout Reference
**URL:** https://docs.rs/ratatui/latest/ratatui/layout/index.html
**Topics:** Layout, Constraint, Direction, Rect, Margin
**When to fetch:** Layout API details, constraint options

### Style Reference
**URL:** https://docs.rs/ratatui/latest/ratatui/style/index.html
**Topics:** Style, Color, Modifier, Stylize trait
**When to fetch:** Style API details, color options

### Terminal Reference
**URL:** https://docs.rs/ratatui/latest/ratatui/terminal/index.html
**Topics:** Terminal struct, Frame, CompletedFrame
**When to fetch:** Terminal management, frame handling

## Widget Documentation

All built-in widgets have dedicated documentation:

### Display Widgets
- **Paragraph**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Paragraph.html
- **Block**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html
- **List**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html
- **Table**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html
- **Tabs**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Tabs.html

### Data Visualization
- **Gauge**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html
- **LineGauge**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.LineGauge.html
- **Sparkline**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html
- **BarChart**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.BarChart.html
- **Chart**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Chart.html

### State Management
- **ListState**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.ListState.html
- **TableState**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.TableState.html
- **ScrollbarState**: https://docs.rs/ratatui/latest/ratatui/widgets/struct.ScrollbarState.html

## Third-Party Crates

### Input Handling
- **tui-input**: https://crates.io/crates/tui-input
  Text input widget with cursor support
- **tui-textarea**: https://crates.io/crates/tui-textarea
  Multi-line text editor widget

### Tree Views
- **tui-tree-widget**: https://crates.io/crates/tui-tree-widget
  Tree view widget for hierarchical data

### Images
- **ratatui-image**: https://crates.io/crates/ratatui-image
  Image rendering in terminal

### File Dialogs
- **tui-file-dialog**: https://crates.io/crates/tui-file-dialog
  File browser and selection widget

## Templates and Examples

### Official Examples
**URL:** https://github.com/ratatui/ratatui/tree/main/examples
**Topics:** Example applications, widget demos, pattern examples
**When to fetch:** Looking for code examples, implementation patterns

### Templates
**URL:** https://ratatui.rs/templates/
**Topics:** Project templates, async patterns, component-based architecture
**When to fetch:** Starting new projects, architecture decisions

### Awesome Ratatui
**URL:** https://github.com/ratatui/awesome-ratatui
**Topics:** Community projects, libraries, tools, resources
**When to fetch:** Looking for third-party libraries, community resources

## Async and Background Tasks

### Tokio Integration
**URL:** https://ratatui.rs/tutorials/counter-app/#async
**Topics:** Using tokio with ratatui, async event handling
**When to fetch:** Questions about async patterns, tokio integration

### Recommended Patterns
- Use `tokio::spawn` for background tasks
- Use `mpsc` channels for worker communication
- Use `CancellationToken` for graceful task cancellation

## Best Practices

### Error Handling
**URL:** https://ratatui.rs/recipes/error-handling/
**Topics:** anyhow, thiserror, panic hooks, terminal restoration
**When to fetch:** Error handling patterns, panic recovery

### Logging
**URL:** https://ratatui.rs/recipes/logging/
**Topics:** tracing crate, tui-logger, debugging TUI apps
**When to fetch:** Adding logging to TUI apps, debugging

### Testing
**URL:** https://ratatui.rs/recipes/testing/
**Topics:** Unit testing widgets, integration testing, snapshot testing
**When to fetch:** Testing strategies, test patterns

## Usage Guidelines

### When to Fetch Guides

1. **Don't fetch unless needed**: The skill already covers fundamentals. Only fetch when:
   - User asks about a specific topic not covered in skill
   - Need detailed API information
   - Complex examples required
   - Latest updates needed (docs may be newer than skill)

2. **Fetch specific sections**: Use targeted URLs for relevant topics

3. **Combine with skill knowledge**: Use fetched content to supplement skill knowledge

### Example Fetch Patterns

```rust
// User asks about sparklines
web_fetch("https://docs.rs/ratatui/latest/ratatui/widgets/struct.Sparkline.html")

// User needs layout details
web_fetch("https://ratatui.rs/concepts/layout/")

// User wants testing info
web_fetch("https://ratatui.rs/recipes/testing/")

// Need widget examples
web_fetch("https://github.com/ratatui/ratatui/tree/main/examples")
```

## Quick Reference by Topic

### Need information about...

**App Structure** -> Hello World Tutorial
**Layout & Positioning** -> Layout concepts
**Styling & Colors** -> Styles, Colors concepts
**User Input** -> Event Handling concepts
**Custom Widgets** -> Widgets concepts
**Data Display** -> Table, List widgets
**Charts & Graphs** -> Chart, Sparkline, Gauge widgets
**Async Tasks** -> Tokio Integration
**Error Handling** -> Error Handling recipes
**Testing** -> Testing recipes
**Logging** -> Logging recipes

## Notes

- All docs.rs URLs follow pattern: `https://docs.rs/ratatui/latest/ratatui/{module}/`
- Official docs are actively maintained at ratatui.rs
- GitHub examples provide practical implementation patterns
- Use web_fetch with specific URLs when detailed or latest information needed
- Combine official docs with skill knowledge for best results
