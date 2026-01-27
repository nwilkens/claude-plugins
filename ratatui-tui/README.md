# Ratatui TUI Plugin

A Claude Code plugin for building modern, interactive terminal user interfaces with Ratatui and Crossterm in Rust.

## Features

- **Complete widget coverage** - Paragraph, List, Table, Gauge, Sparkline, Tabs, Canvas, and more
- **Layout system** - Constraint-based layouts with vertical, horizontal, and nested arrangements
- **Styling guide** - Full coverage of Style, Color, Modifier, and Stylize trait
- **Event handling** - Keyboard, mouse, and resize events with Action enum pattern
- **Async patterns** - Background tasks with tokio, mpsc channels, and CancellationToken
- **Error handling** - Panic hooks, terminal restoration, anyhow/thiserror patterns
- **Testing patterns** - Unit tests, TestBackend integration tests
- **Example applications** - Four complete working Cargo projects

## Installation

```bash
# Add marketplace (if using local marketplace)
/plugin marketplace add ./path-to-marketplace

# Install plugin
/plugin install ratatui-tui@marketplace-name

# Or enable directly
/plugin enable ratatui-tui
```

## Usage

### Using the Skill

The skill is automatically invoked when you ask Claude to:
- Create a TUI or terminal application
- Build a command-line interface
- Create dashboard or monitoring tools
- Build data viewers or interactive terminal tools

Example prompts:
```
Create a todo list TUI with Ratatui
Build a system monitoring dashboard with CPU and memory gauges
Create a log viewer TUI that can filter and search log files
Make a file browser TUI with navigation and preview
```

## Included Content

### Main Skill (SKILL.md)
- Quick start with basic app structure
- Core architecture (event loop, state management)
- Layout system (constraints, nested layouts)
- Styling guide (Style, Color, Modifier)
- Event handling patterns
- Async patterns with tokio
- Error handling and terminal restoration
- Testing strategies

### Reference Files
- `references/widgets.md` - Gallery of Ratatui widgets
- `references/layouts.md` - Layout patterns and recipes
- `references/styling.md` - Complete styling guide
- `references/official-guides-index.md` - Index of official documentation

### Example Applications
- `assets/todo_app/` - Todo list with List, ListState, input handling
- `assets/dashboard_app/` - System monitor with Gauge, Sparkline, sysinfo
- `assets/data_viewer/` - JSON/CSV viewer with Table, modal dialogs
- `assets/worker_demo/` - Background tasks with tokio, mpsc channels

## Quick Example

```rust
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    loop {
        terminal.draw(|frame| {
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Hello Ratatui");
            let paragraph = Paragraph::new("Press 'q' to quit").block(block);
            frame.render_widget(paragraph, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
```

## Resources

- **Ratatui Documentation**: https://ratatui.rs/
- **Ratatui API Reference**: https://docs.rs/ratatui/
- **Ratatui GitHub**: https://github.com/ratatui/ratatui
- **Crossterm**: https://github.com/crossterm-rs/crossterm

## License

MIT License
