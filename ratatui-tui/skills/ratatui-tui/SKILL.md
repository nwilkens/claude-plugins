---
name: ratatui-tui
description: Build modern, interactive terminal user interfaces with Ratatui and Crossterm in Rust. Use when creating command-line applications, dashboard tools, monitoring interfaces, data viewers, or any terminal-based UI. Covers architecture, widgets, layouts, styling, event handling, async patterns with tokio, and testing patterns.
---

# Ratatui TUI Development

Build production-quality terminal user interfaces using Ratatui, a Rust library for creating rich terminal UIs, paired with Crossterm for cross-platform terminal manipulation.

## Quick Start

Add dependencies to `Cargo.toml`:
```toml
[dependencies]
ratatui = "0.28"
crossterm = "0.28"
anyhow = "1.0"
tokio = { version = "1", features = ["full"] }
```

Basic app structure with event loop:
```rust
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Run app
    let result = run(&mut terminal);

    // Restore terminal (CRITICAL: always restore on exit)
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
    loop {
        // Draw UI
        terminal.draw(|frame| {
            let area = frame.area();
            let block = Block::default()
                .title("My App")
                .borders(Borders::ALL);
            let paragraph = Paragraph::new("Hello, Ratatui!")
                .block(block);
            frame.render_widget(paragraph, area);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
```

Run your app:
```bash
cargo run --release
```

## Core Architecture

### App Lifecycle

1. **Setup**: Enable raw mode, enter alternate screen
2. **Initialize**: Create Terminal with backend, initialize App state
3. **Event Loop**: Poll events, update state, render UI
4. **Shutdown**: Restore terminal (disable raw mode, leave alternate screen)

### App State Pattern

Use a dedicated struct for application state:
```rust
#[derive(Default)]
struct App {
    counter: i64,
    running: bool,
    selected_tab: usize,
    items: Vec<String>,
    list_state: ListState,
}

impl App {
    fn new() -> Self {
        Self {
            running: true,
            list_state: ListState::default().with_selected(Some(0)),
            ..Default::default()
        }
    }

    fn increment(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
```

### Main Event Loop Pattern

```rust
fn run(terminal: &mut Terminal<impl Backend>, app: &mut App) -> anyhow::Result<()> {
    while app.running {
        terminal.draw(|frame| ui(frame, app))?;
        handle_events(app)?;
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    // Render widgets based on app state
    let counter_text = format!("Counter: {}", app.counter);
    let paragraph = Paragraph::new(counter_text)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, frame.area());
}

fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Up => app.increment(),
                    KeyCode::Down => app.decrement(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
```

## Event Handling

### Crossterm Events

Handle keyboard, mouse, and resize events:
```rust
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEvent, MouseEventKind};

fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => app.quit(),
                    (KeyModifiers::NONE, KeyCode::Char('q')) => app.quit(),
                    (KeyModifiers::NONE, KeyCode::Up) => app.previous(),
                    (KeyModifiers::NONE, KeyCode::Down) => app.next(),
                    (KeyModifiers::NONE, KeyCode::Enter) => app.select(),
                    (KeyModifiers::NONE, KeyCode::Tab) => app.next_tab(),
                    _ => {}
                }
            }
            Event::Mouse(mouse) => handle_mouse(app, mouse),
            Event::Resize(width, height) => app.resize(width, height),
            _ => {}
        }
    }
    Ok(())
}

fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(_) => {
            app.click(mouse.column, mouse.row);
        }
        MouseEventKind::ScrollUp => app.scroll_up(),
        MouseEventKind::ScrollDown => app.scroll_down(),
        _ => {}
    }
}
```

### Action Enum Pattern

Use an enum for type-safe event handling:
```rust
#[derive(Debug, Clone, Copy)]
enum Action {
    Quit,
    Increment,
    Decrement,
    NextTab,
    PreviousTab,
    Select,
    ScrollUp,
    ScrollDown,
    None,
}

impl From<KeyCode> for Action {
    fn from(key: KeyCode) -> Self {
        match key {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Up | KeyCode::Char('k') => Action::Increment,
            KeyCode::Down | KeyCode::Char('j') => Action::Decrement,
            KeyCode::Tab => Action::NextTab,
            KeyCode::BackTab => Action::PreviousTab,
            KeyCode::Enter => Action::Select,
            _ => Action::None,
        }
    }
}

fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                let action = Action::from(key.code);
                app.handle_action(action);
            }
        }
    }
    Ok(())
}

impl App {
    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.running = false,
            Action::Increment => self.increment(),
            Action::Decrement => self.decrement(),
            Action::NextTab => self.next_tab(),
            Action::PreviousTab => self.previous_tab(),
            Action::Select => self.select_item(),
            Action::ScrollUp => self.scroll_up(),
            Action::ScrollDown => self.scroll_down(),
            Action::None => {}
        }
    }
}
```

## Layout System

### Layout Directions

Create vertical and horizontal layouts:
```rust
use ratatui::layout::{Constraint, Direction, Layout};

fn ui(frame: &mut Frame, app: &App) {
    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // Header: fixed 3 rows
            Constraint::Min(0),         // Content: remaining space
            Constraint::Length(1),      // Footer: fixed 1 row
        ])
        .split(frame.area());

    // Render to each chunk
    frame.render_widget(header_widget(), chunks[0]);
    frame.render_widget(content_widget(app), chunks[1]);
    frame.render_widget(footer_widget(), chunks[2]);
}

fn content_with_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    // Horizontal layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Sidebar: 30%
            Constraint::Percentage(70), // Main: 70%
        ])
        .split(area);

    frame.render_widget(sidebar_widget(app), chunks[0]);
    frame.render_widget(main_widget(app), chunks[1]);
}
```

### Constraint Types

Control widget sizing with constraints:
```rust
use ratatui::layout::Constraint;

// Fixed size in cells
Constraint::Length(10)      // Exactly 10 cells

// Percentage of available space
Constraint::Percentage(50)  // 50% of available space

// Minimum size
Constraint::Min(5)          // At least 5 cells

// Maximum size
Constraint::Max(20)         // At most 20 cells

// Ratio (numerator, denominator)
Constraint::Ratio(1, 3)     // 1/3 of available space

// Fill remaining space (with optional minimum)
Constraint::Fill(1)         // Fill remaining, weight 1

// Complex layout example
let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),      // Fixed header
        Constraint::Percentage(60), // Main content
        Constraint::Min(10),        // At least 10 rows for list
        Constraint::Length(1),      // Fixed status bar
    ]);
```

### Nested Layouts

Create complex layouts with nesting:
```rust
fn ui(frame: &mut Frame, app: &App) {
    // Outer vertical layout
    let outer = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    // Inner horizontal layout for content area
    let inner = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ])
    .split(outer[1]);

    // Right panel split vertically
    let right = Layout::vertical([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(inner[1]);

    frame.render_widget(header(), outer[0]);
    frame.render_widget(sidebar(app), inner[0]);
    frame.render_widget(top_panel(app), right[0]);
    frame.render_widget(bottom_panel(app), right[1]);
    frame.render_widget(footer(), outer[2]);
}
```

### Margin and Spacing

Add margins to layouts:
```rust
use ratatui::layout::Margin;

fn ui(frame: &mut Frame, app: &App) {
    // Add margin around content
    let inner_area = frame.area().inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    // Or use uniform margin
    let inner_area = frame.area().inner(Margin::new(2, 1));

    frame.render_widget(content(app), inner_area);
}
```

## Styling

### Style Struct

Apply styles to widgets:
```rust
use ratatui::style::{Color, Modifier, Style};

// Create styles
let style = Style::default()
    .fg(Color::White)
    .bg(Color::Blue)
    .add_modifier(Modifier::BOLD);

// Apply to text
let text = Span::styled("Hello", style);

// Named colors
let style = Style::default()
    .fg(Color::Red)
    .bg(Color::Black);

// RGB colors
let style = Style::default()
    .fg(Color::Rgb(255, 128, 0));  // Orange

// Indexed colors (256-color palette)
let style = Style::default()
    .fg(Color::Indexed(208));  // Also orange
```

### Stylize Trait (Fluent API)

Use the fluent API for cleaner styling:
```rust
use ratatui::style::Stylize;

// Fluent styling
let text = "Hello".bold().red().on_blue();
let span = Span::raw("World").italic().green();

// Chain multiple styles
let styled = "Important"
    .bold()
    .underlined()
    .fg(Color::Yellow)
    .bg(Color::DarkGray);

// Apply to widgets
let paragraph = Paragraph::new("Content")
    .style(Style::default().white().on_black())
    .block(Block::default().borders(Borders::ALL).blue());
```

### Modifiers

Add text modifiers:
```rust
use ratatui::style::Modifier;

let style = Style::default()
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::ITALIC)
    .add_modifier(Modifier::UNDERLINED);

// Or combine with bitwise OR
let style = Style::default()
    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

// Available modifiers:
// BOLD, DIM, ITALIC, UNDERLINED, SLOW_BLINK, RAPID_BLINK,
// REVERSED, HIDDEN, CROSSED_OUT
```

### Styled Text with Spans

Create rich text with multiple styles:
```rust
use ratatui::text::{Line, Span, Text};

// Single styled span
let span = Span::styled("Error", Style::default().red().bold());

// Line with multiple spans
let line = Line::from(vec![
    Span::raw("Status: "),
    Span::styled("OK", Style::default().green().bold()),
]);

// Multi-line text
let text = Text::from(vec![
    Line::from("First line"),
    Line::from(vec![
        Span::raw("Key: "),
        Span::styled("value", Style::default().yellow()),
    ]),
    Line::styled("Third line", Style::default().italic()),
]);

let paragraph = Paragraph::new(text);
```

## Widgets

### Block (Container)

Wrap widgets with borders and titles:
```rust
use ratatui::widgets::{Block, Borders, BorderType};

let block = Block::default()
    .title("Title")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(Style::default().cyan())
    .style(Style::default().bg(Color::DarkGray));

// Border types: Plain, Rounded, Double, Thick, QuadrantInside, QuadrantOutside
```

### Paragraph

Display text content:
```rust
use ratatui::widgets::{Paragraph, Wrap};

let paragraph = Paragraph::new("Long text content that might wrap...")
    .block(Block::default().title("Info").borders(Borders::ALL))
    .style(Style::default().white())
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

// Scrollable paragraph
let paragraph = Paragraph::new(long_text)
    .scroll((app.scroll_offset, 0));  // (vertical, horizontal)

frame.render_widget(paragraph, area);
```

### List

Display selectable lists:
```rust
use ratatui::widgets::{List, ListItem, ListState};

// Create list items
let items: Vec<ListItem> = app.items
    .iter()
    .map(|item| ListItem::new(item.as_str()))
    .collect();

let list = List::new(items)
    .block(Block::default().title("Items").borders(Borders::ALL))
    .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
    .highlight_symbol("> ");

// Render with state for selection tracking
frame.render_stateful_widget(list, area, &mut app.list_state);
```

### Table

Display tabular data:
```rust
use ratatui::widgets::{Table, Row, Cell, TableState};

let header = Row::new(vec![
    Cell::from("Name").style(Style::default().bold()),
    Cell::from("Age").style(Style::default().bold()),
    Cell::from("City").style(Style::default().bold()),
]);

let rows: Vec<Row> = app.data
    .iter()
    .map(|(name, age, city)| {
        Row::new(vec![
            Cell::from(name.as_str()),
            Cell::from(age.to_string()),
            Cell::from(city.as_str()),
        ])
    })
    .collect();

let table = Table::new(rows, [
    Constraint::Percentage(40),
    Constraint::Percentage(20),
    Constraint::Percentage(40),
])
    .header(header)
    .block(Block::default().title("Users").borders(Borders::ALL))
    .highlight_style(Style::default().bg(Color::DarkGray))
    .highlight_symbol(">> ");

frame.render_stateful_widget(table, area, &mut app.table_state);
```

### Tabs

Create tabbed interfaces:
```rust
use ratatui::widgets::Tabs;

let titles = vec!["Tab1", "Tab2", "Tab3"];

let tabs = Tabs::new(titles)
    .block(Block::default().title("Tabs").borders(Borders::ALL))
    .select(app.selected_tab)
    .style(Style::default().white())
    .highlight_style(Style::default().yellow().bold())
    .divider("|");

frame.render_widget(tabs, area);
```

### Gauge (Progress Bar)

Show progress indicators:
```rust
use ratatui::widgets::{Gauge, LineGauge};

// Filled gauge
let gauge = Gauge::default()
    .block(Block::default().title("Progress").borders(Borders::ALL))
    .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
    .percent(app.progress)
    .label(format!("{}%", app.progress));

// Line gauge (thin progress bar)
let line_gauge = LineGauge::default()
    .block(Block::default().title("Download"))
    .gauge_style(Style::default().fg(Color::Cyan))
    .ratio(app.download_progress);

frame.render_widget(gauge, area);
```

### Sparkline

Display mini charts:
```rust
use ratatui::widgets::Sparkline;

let sparkline = Sparkline::default()
    .block(Block::default().title("CPU").borders(Borders::ALL))
    .data(&app.cpu_history)
    .max(100)
    .style(Style::default().fg(Color::Yellow));

frame.render_widget(sparkline, area);
```

### Canvas (Drawing)

Draw shapes and charts:
```rust
use ratatui::widgets::canvas::{Canvas, Line, Rectangle, Circle};

let canvas = Canvas::default()
    .block(Block::default().title("Canvas").borders(Borders::ALL))
    .x_bounds([0.0, 100.0])
    .y_bounds([0.0, 100.0])
    .paint(|ctx| {
        ctx.draw(&Line {
            x1: 0.0,
            y1: 0.0,
            x2: 100.0,
            y2: 100.0,
            color: Color::Red,
        });
        ctx.draw(&Rectangle {
            x: 10.0,
            y: 10.0,
            width: 20.0,
            height: 20.0,
            color: Color::Green,
        });
    });

frame.render_widget(canvas, area);
```

### Third-Party Widget Crates

Popular extensions:
```toml
# Cargo.toml
tui-textarea = "0.5"     # Multi-line text editor
tui-input = "0.9"        # Single-line input
tui-tree-widget = "0.20" # Tree view
tui-scrollview = "0.3"   # Scrollable container
tui-big-text = "0.5"     # Large ASCII text
```

## StatefulWidget Pattern

For widgets that track selection or scroll state:
```rust
use ratatui::widgets::{StatefulWidget, ListState, TableState};

struct App {
    items: Vec<String>,
    list_state: ListState,
    table_state: TableState,
}

impl App {
    fn new() -> Self {
        Self {
            items: vec!["Item 1".into(), "Item 2".into(), "Item 3".into()],
            list_state: ListState::default().with_selected(Some(0)),
            table_state: TableState::default(),
        }
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let list = List::new(app.items.iter().map(|s| ListItem::new(s.as_str())))
        .highlight_style(Style::default().reversed());

    // Use render_stateful_widget for stateful widgets
    frame.render_stateful_widget(list, frame.area(), &mut app.list_state);
}
```

## Background Tasks

### Tokio Async Pattern

Use tokio for async operations:
```rust
use tokio::sync::mpsc;
use std::time::Duration;

#[derive(Debug)]
enum AppEvent {
    Input(KeyCode),
    Tick,
    DataLoaded(Vec<String>),
    Error(String),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel::<AppEvent>();

    // Spawn tick task
    let tick_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(250)).await;
            if tick_tx.send(AppEvent::Tick).is_err() {
                break;
            }
        }
    });

    // Spawn input handler
    let input_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        if input_tx.send(AppEvent::Input(key.code)).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });

    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = App::new();

    // Main event loop
    while app.running {
        terminal.draw(|f| ui(f, &app))?;

        if let Some(event) = rx.recv().await {
            match event {
                AppEvent::Input(key) => app.handle_key(key),
                AppEvent::Tick => app.tick(),
                AppEvent::DataLoaded(data) => app.set_data(data),
                AppEvent::Error(msg) => app.show_error(&msg),
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
```

### Worker with mpsc Channels

Communicate between main thread and workers:
```rust
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

struct App {
    running: bool,
    data: Vec<String>,
    loading: bool,
    worker_tx: mpsc::Sender<WorkerCommand>,
    cancel_token: CancellationToken,
}

#[derive(Debug)]
enum WorkerCommand {
    FetchData(String),
    Cancel,
}

#[derive(Debug)]
enum WorkerResult {
    Data(Vec<String>),
    Error(String),
    Progress(u8),
}

impl App {
    fn spawn_worker(
        &self,
        mut cmd_rx: mpsc::Receiver<WorkerCommand>,
        result_tx: mpsc::Sender<WorkerResult>,
        cancel_token: CancellationToken,
    ) {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => {
                        break;
                    }
                    Some(cmd) = cmd_rx.recv() => {
                        match cmd {
                            WorkerCommand::FetchData(url) => {
                                // Simulate fetch with progress
                                for i in 0..10 {
                                    if cancel_token.is_cancelled() {
                                        break;
                                    }
                                    tokio::time::sleep(Duration::from_millis(100)).await;
                                    let _ = result_tx.send(WorkerResult::Progress(i * 10)).await;
                                }

                                match fetch_data(&url).await {
                                    Ok(data) => {
                                        let _ = result_tx.send(WorkerResult::Data(data)).await;
                                    }
                                    Err(e) => {
                                        let _ = result_tx.send(WorkerResult::Error(e.to_string())).await;
                                    }
                                }
                            }
                            WorkerCommand::Cancel => break,
                        }
                    }
                }
            }
        });
    }

    fn request_data(&self, url: &str) {
        let _ = self.worker_tx.try_send(WorkerCommand::FetchData(url.to_string()));
    }

    fn cancel_work(&self) {
        self.cancel_token.cancel();
    }
}

async fn fetch_data(url: &str) -> anyhow::Result<Vec<String>> {
    // Actual fetch implementation
    Ok(vec!["data1".into(), "data2".into()])
}
```

### CancellationToken for Graceful Shutdown

```rust
use tokio_util::sync::CancellationToken;

async fn run_app(
    terminal: &mut Terminal<impl Backend>,
    cancel_token: CancellationToken,
) -> anyhow::Result<()> {
    let mut app = App::new();

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                break;
            }
            _ = async {
                terminal.draw(|f| ui(f, &app)).ok();
            } => {}
        }

        // Non-blocking event check
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            cancel_token.cancel();
                            break;
                        }
                        _ => app.handle_key(key.code),
                    }
                }
            }
        }
    }

    Ok(())
}
```

## Error Handling

### Using anyhow for Application Errors

```rust
use anyhow::{Context, Result, bail};

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path))?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;

    if config.items.is_empty() {
        bail!("Config must have at least one item");
    }

    Ok(config)
}

fn main() -> Result<()> {
    let config = load_config("config.toml")?;
    run_app(config)
}
```

### Using thiserror for Library Errors

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Terminal initialization failed: {0}")]
    TerminalInit(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Data fetch failed: {source}")]
    Fetch {
        #[from]
        source: reqwest::Error,
    },

    #[error("Invalid selection: index {index} out of bounds (max: {max})")]
    InvalidSelection { index: usize, max: usize },
}

impl App {
    fn select(&mut self, index: usize) -> Result<(), AppError> {
        if index >= self.items.len() {
            return Err(AppError::InvalidSelection {
                index,
                max: self.items.len().saturating_sub(1),
            });
        }
        self.selected = index;
        Ok(())
    }
}
```

### Terminal Restoration on Panic (CRITICAL)

Always restore terminal state, even on panic:
```rust
use std::panic;

fn main() -> anyhow::Result<()> {
    // Install panic hook BEFORE enabling raw mode
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal on panic
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Setup terminal
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Run app with Result to catch errors
    let result = run(&mut terminal);

    // Always restore terminal (runs even if run() returns Err)
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}
```

### Using a Terminal Guard

RAII pattern for terminal cleanup:
```rust
struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalGuard {
    fn new() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        io::stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }

    fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
    }
}

fn main() -> anyhow::Result<()> {
    // Set panic hook
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        original_hook(info);
    }));

    // Terminal automatically restored when guard is dropped
    let mut guard = TerminalGuard::new()?;
    run(guard.terminal())
}
```

## Testing

### Unit Testing App Logic

Test state changes independently of UI:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut app = App::new();
        assert_eq!(app.counter, 0);

        app.increment();
        assert_eq!(app.counter, 1);

        app.increment();
        assert_eq!(app.counter, 2);
    }

    #[test]
    fn test_decrement_saturation() {
        let mut app = App::new();
        app.decrement();
        // Should not go below 0 with saturating_sub
        assert_eq!(app.counter, 0);
    }

    #[test]
    fn test_list_navigation() {
        let mut app = App::new();
        app.items = vec!["a".into(), "b".into(), "c".into()];
        app.list_state = ListState::default().with_selected(Some(0));

        app.next();
        assert_eq!(app.list_state.selected(), Some(1));

        app.next();
        assert_eq!(app.list_state.selected(), Some(2));

        app.next(); // Should wrap to 0
        assert_eq!(app.list_state.selected(), Some(0));
    }

    #[test]
    fn test_action_handling() {
        let mut app = App::new();
        app.running = true;

        app.handle_action(Action::Quit);
        assert!(!app.running);
    }
}
```

### Integration Testing with TestBackend

Test rendering without a real terminal:
```rust
use ratatui::backend::TestBackend;

#[test]
fn test_ui_renders() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let app = App::new();

    terminal.draw(|f| ui(f, &app)).unwrap();

    // Get rendered buffer
    let buffer = terminal.backend().buffer();

    // Check that expected text is rendered
    let content = buffer_to_string(buffer);
    assert!(content.contains("Counter: 0"));
}

fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
    let mut s = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            s.push(buffer.get(x, y).symbol().chars().next().unwrap_or(' '));
        }
        s.push('\n');
    }
    s
}

#[test]
fn test_ui_layout() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.items = vec!["Item 1".into(), "Item 2".into()];

    terminal.draw(|f| ui(f, &mut app)).unwrap();

    let buffer = terminal.backend().buffer();

    // Verify specific cells
    assert_eq!(buffer.get(0, 0).symbol(), "┌"); // Top-left border
}
```

### Async Test with Tokio

```rust
#[cfg(test)]
mod async_tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_worker_communication() {
        let (tx, mut rx) = mpsc::channel::<WorkerResult>(10);
        let (cmd_tx, cmd_rx) = mpsc::channel::<WorkerCommand>(10);
        let cancel = CancellationToken::new();

        // Spawn worker
        spawn_worker(cmd_rx, tx, cancel.clone());

        // Send command
        cmd_tx.send(WorkerCommand::FetchData("test".into())).await.unwrap();

        // Wait for result with timeout
        let result = tokio::time::timeout(
            Duration::from_secs(5),
            rx.recv()
        ).await;

        assert!(result.is_ok());
        cancel.cancel();
    }
}
```

## Best Practices

### Panic Hooks for Terminal Cleanup

CRITICAL: Always install panic hooks before enabling raw mode:
```rust
fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
        let _ = io::stdout().execute(crossterm::cursor::Show);

        // Call original hook to print panic info
        original_hook(panic_info);
    }));
}
```

### Logging with tracing

Use tracing for debugging (logs to file, not terminal):
```rust
// Cargo.toml
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }
// tracing-appender = "0.2"

use tracing::{info, debug, error, warn, instrument};
use tracing_subscriber::{self, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

fn setup_logging() -> anyhow::Result<()> {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "app.log",
    );

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(file_appender)
        .with_ansi(false)
        .init();

    Ok(())
}

#[instrument(skip(app))]
fn handle_events(app: &mut App) -> anyhow::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            debug!(?key, "Key pressed");
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        info!("User requested quit");
                        app.quit();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
```

### Terminal Cleanup Checklist

Always ensure these are called on exit:
```rust
fn restore_terminal() -> anyhow::Result<()> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    io::stdout().execute(crossterm::cursor::Show)?;
    Ok(())
}
```

### Performance Tips

1. **Minimize redraws**: Only redraw when state changes
2. **Use constraints wisely**: Prefer `Length` and `Min` over `Percentage` when possible
3. **Batch state updates**: Update multiple fields before triggering redraw
4. **Use `Frame::area()`**: Cache the area instead of calling multiple times

```rust
fn run(terminal: &mut Terminal<impl Backend>, app: &mut App) -> anyhow::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        // Only draw if needed
        if app.needs_redraw {
            terminal.draw(|f| ui(f, app))?;
            app.needs_redraw = false;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                    app.needs_redraw = true;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            app.needs_redraw = true;
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }
    Ok(())
}
```

### Project Structure

Recommended layout for larger apps:
```
src/
├── main.rs          # Entry point, terminal setup
├── app.rs           # App state and logic
├── ui.rs            # UI rendering functions
├── ui/
│   ├── mod.rs
│   ├── header.rs
│   ├── sidebar.rs
│   └── content.rs
├── event.rs         # Event handling, Action enum
├── widgets/         # Custom widgets
│   ├── mod.rs
│   └── status_bar.rs
└── worker.rs        # Background task handling
```

## Common Pitfalls

1. **Forgetting terminal restoration**: Always use panic hooks and ensure cleanup runs on all exit paths
2. **Blocking the event loop**: Never use `std::thread::sleep()` in the main loop - use async or non-blocking event polling
3. **Not using `KeyEventKind::Press`**: Without this check, key events fire on both press and release
4. **Forgetting `render_stateful_widget`**: Lists and tables with selection need stateful rendering
5. **Wrong constraint ordering**: Constraints are applied in order; earlier constraints get priority
6. **Not handling resize events**: Terminal size can change; re-layout on `Event::Resize`
7. **Mixing sync and async**: Be consistent with your async runtime choice
8. **Logging to stdout**: Use file-based logging since stdout is the terminal
9. **Not testing with TestBackend**: Integration tests should use `TestBackend` to verify rendering
10. **Panic without cleanup**: Raw mode persists after crash; always install panic hooks first

## References

- **Ratatui Book**: https://ratatui.rs/
- **API Documentation**: https://docs.rs/ratatui
- **Crossterm Docs**: https://docs.rs/crossterm
- **Example Apps**: https://github.com/ratatui-org/ratatui/tree/main/examples
- **Async Template**: https://github.com/ratatui-org/async-template
- **Widget Showcase**: Run `cargo run --example demo` from ratatui repo
