# Example Ratatui Applications

Complete, working example applications demonstrating various Ratatui patterns and features.

## Running the Examples

Each example is a standalone Cargo project. To run:

```bash
# Navigate to an example directory
cd todo_app

# Build and run
cargo run

# Or run with release optimizations
cargo run --release
```

For development with automatic recompilation, use `cargo-watch`:

```bash
# Install cargo-watch (once)
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

## Examples

### todo_app - Todo List Application

A fully functional todo list demonstrating:
- Input handling with crossterm events
- List widget with ListState for selection tracking
- Application state management with a central App struct
- Keyboard shortcuts and bindings
- Styling with Ratatui Style and Color
- Visual feedback for completed items

**Key Features:**
- Add/delete/toggle todos
- Mark items as complete with strikethrough
- Statistics tracking
- Keyboard shortcuts (a: add, d: delete, Space: toggle, j/k: navigate, q: quit)

**Patterns Demonstrated:**
- Action enum for type-safe event handling
- Stateful widgets (List with ListState)
- Layout constraints for UI structure
- Conditional styling based on state

**Run it:**
```bash
cd todo_app && cargo run
```

### dashboard_app - System Monitor Dashboard

A real-time system monitoring dashboard demonstrating:
- Nested layouts with Constraint-based sizing
- Gauge widgets for percentage displays
- Sparkline for historical data visualization
- Periodic updates with tokio interval
- sysinfo crate for system metrics

**Key Features:**
- Live CPU and memory monitoring
- Historical CPU usage sparkline (60 samples)
- Running process count
- Auto-refresh every second
- Color-coded gauges (green/yellow/red thresholds)

**Patterns Demonstrated:**
- Horizontal and vertical layout composition
- Multiple widget types (Gauge, Sparkline, Paragraph)
- Async runtime with tokio for timed updates
- External crate integration (sysinfo)

**Run it:**
```bash
cd dashboard_app && cargo run
```

### data_viewer - JSON/CSV Data Viewer

A file browser and data viewer demonstrating:
- Two-panel split layout (file list + data view)
- Table widget with TableState for tabular data
- Tabs widget for switching views (Table/Raw)
- Modal popup dialogs using centered rect calculation
- File I/O with JSON/CSV parsing

**Key Features:**
- Browse directories and select files
- Load and display JSON and CSV files
- View data as table or raw content
- File type icons ([D] directory, [J] JSON, [C] CSV)
- Modal error dialogs
- Vim-style navigation (h/j/k/l)

**Patterns Demonstrated:**
- Panel switching with Tab key
- Tab widget for view modes
- Clear widget for modal overlays
- serde_json and csv crate integration
- Centered rect helper for popup positioning

**Run it:**
```bash
cd data_viewer && cargo run
```

### worker_demo - Background Task Processing

A comprehensive async worker pattern demonstration:

**Key Features:**
- tokio::spawn for background task execution
- mpsc channels for worker-to-UI communication
- CancellationToken for graceful task cancellation
- Real-time progress updates via Gauge widget
- Log-style output area with colored messages
- Statistics tracking (speed, elapsed time)

**Patterns Demonstrated:**
- Message passing with WorkerMessage enum
- Async task spawning and management
- Channel-based communication (mpsc::Sender/Receiver)
- Cancellation token pattern for task control
- Non-blocking UI with event polling
- Progress reporting from background tasks

**Controls:**
- `s`: Start worker (processes 100 simulated files)
- `c`: Cancel running worker
- `q`: Quit application

**Run it:**
```bash
cd worker_demo && cargo run
```

## Common Patterns

All examples demonstrate these fundamental Ratatui patterns:

- **Terminal setup/restore**: enable_raw_mode, EnterAlternateScreen, crossterm backend
- **Event loop**: poll/read events with timeout, handle KeyEventKind::Press
- **Layout system**: Layout::default() with Direction and Constraint
- **Widget rendering**: Frame::render_widget and render_stateful_widget
- **Styling**: Style::default().fg(Color).add_modifier(Modifier)
- **Blocks**: Block::default().borders(Borders::ALL).title("...")

## Learning Path

1. **Start with todo_app** - Learn basic input, lists, and state management
2. **Move to dashboard_app** - Understand layouts, gauges, and timed updates
3. **Try worker_demo** - Master async tasks and channel communication
4. **Explore data_viewer** - Learn complex layouts, tabs, and file handling

## Project Structure

Each example follows a standard Cargo project structure:

```
example_name/
├── Cargo.toml    # Dependencies and metadata
└── src/
    └── main.rs   # Application code
```

## Common Dependencies

| Crate | Purpose |
|-------|---------|
| ratatui | TUI framework |
| crossterm | Terminal backend and events |
| anyhow | Error handling |
| tokio | Async runtime (dashboard, worker) |
| sysinfo | System metrics (dashboard) |
| serde/serde_json | JSON parsing (data_viewer) |
| csv | CSV parsing (data_viewer) |
| tokio-util | CancellationToken (worker) |

## Extending the Examples

Feel free to modify these examples:
- Add persistence to todo_app (save/load from file with serde)
- Add network monitoring to dashboard_app
- Add export functionality to data_viewer
- Add parallel workers to worker_demo
- Customize the styling and color schemes
- Add new widgets and features

## Resources

- Ratatui Documentation: https://ratatui.rs/
- Ratatui GitHub: https://github.com/ratatui/ratatui
- Crossterm Documentation: https://docs.rs/crossterm/
- Tokio Documentation: https://tokio.rs/
