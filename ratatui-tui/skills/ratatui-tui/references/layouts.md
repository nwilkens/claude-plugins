# Ratatui Layout Patterns

Common layout recipes for Ratatui terminal applications using constraint-based layouts.

## Layout Basics

Ratatui uses a constraint-based layout system where you define how space should be divided using the `Layout` struct and various `Constraint` types.

### Layout Struct

Create layouts using builder methods:

```rust
use ratatui::layout::{Layout, Direction, Constraint};

// Vertical layout (stack top to bottom)
let layout = Layout::vertical([
    Constraint::Length(3),
    Constraint::Min(0),
    Constraint::Length(3),
]);

// Horizontal layout (stack left to right)
let layout = Layout::horizontal([
    Constraint::Percentage(30),
    Constraint::Percentage(70),
]);

// Using default() with direction
let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(0),
    ]);
```

### Direction

Controls how widgets are arranged:

```rust
use ratatui::layout::Direction;

// Stack widgets vertically (top to bottom)
Direction::Vertical

// Stack widgets horizontally (left to right)
Direction::Horizontal
```

### Constraint Types

Constraints define how space is allocated to each area:

```rust
use ratatui::layout::Constraint;

// Percentage of available space (0-100)
Constraint::Percentage(50)  // 50% of parent

// Fixed size in cells
Constraint::Length(10)  // Exactly 10 cells

// Minimum size (at least this many cells)
Constraint::Min(5)  // At least 5 cells, can grow

// Maximum size (at most this many cells)
Constraint::Max(20)  // At most 20 cells, can shrink

// Ratio of space (numerator, denominator)
Constraint::Ratio(1, 3)  // 1/3 of available space

// Fill remaining space with weight
Constraint::Fill(1)  // Fill with weight 1
```

### Splitting Areas

Apply layouts to get concrete `Rect` areas:

```rust
use ratatui::layout::{Layout, Constraint, Rect};

fn render(frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .split(frame.area());

    // chunks[0] is the header area (3 rows)
    // chunks[1] is the content area (remaining space)

    frame.render_widget(header_widget, chunks[0]);
    frame.render_widget(content_widget, chunks[1]);
}
```

## Common Patterns

### Two-Panel Split (Horizontal)

Side-by-side panels:

```rust
use ratatui::{
    layout::{Layout, Direction, Constraint},
    widgets::{Block, Borders},
    Frame,
};

fn render_split_horizontal(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(frame.area());

    let left_block = Block::default()
        .title("Sidebar")
        .borders(Borders::ALL);

    let right_block = Block::default()
        .title("Main Content")
        .borders(Borders::ALL);

    frame.render_widget(left_block, chunks[0]);
    frame.render_widget(right_block, chunks[1]);
}
```

### Two-Panel Split (Vertical)

Stacked panels:

```rust
fn render_split_vertical(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    frame.render_widget(top_widget, chunks[0]);
    frame.render_widget(bottom_widget, chunks[1]);
}
```

### Header-Content-Footer

Classic application layout:

```rust
fn render_header_content_footer(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Header (fixed 3 rows)
            Constraint::Min(0),       // Content (fill remaining)
            Constraint::Length(3),    // Footer (fixed 3 rows)
        ])
        .split(frame.area());

    let header = Block::default()
        .title("My Application")
        .borders(Borders::ALL);

    let content = Block::default()
        .title("Content")
        .borders(Borders::ALL);

    let footer = Block::default()
        .title("Status: Ready")
        .borders(Borders::ALL);

    frame.render_widget(header, chunks[0]);
    frame.render_widget(content, chunks[1]);
    frame.render_widget(footer, chunks[2]);
}
```

### Three-Column Layout

Sidebar-Content-Info panel:

```rust
fn render_three_column(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),   // Left sidebar (fixed)
            Constraint::Min(0),       // Content (fill)
            Constraint::Length(25),   // Right panel (fixed)
        ])
        .split(frame.area());

    frame.render_widget(sidebar, chunks[0]);
    frame.render_widget(content, chunks[1]);
    frame.render_widget(info_panel, chunks[2]);
}
```

### Dashboard Grid

Nested layouts for grid-like arrangements:

```rust
fn render_dashboard(frame: &mut Frame) {
    // First, split vertically into rows
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Header
            Constraint::Length(10),   // Top row of cards
            Constraint::Min(0),       // Main content
            Constraint::Length(3),    // Footer
        ])
        .split(frame.area());

    // Split the top row into columns for metric cards
    let top_cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(rows[1]);

    // Render header
    let header = Block::default()
        .title("Dashboard")
        .borders(Borders::ALL);
    frame.render_widget(header, rows[0]);

    // Render metric cards
    for (i, area) in top_cards.iter().enumerate() {
        let card = Block::default()
            .title(format!("Metric {}", i + 1))
            .borders(Borders::ALL);
        frame.render_widget(card, *area);
    }

    // Split main content into two columns
    let main_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(rows[2]);

    frame.render_widget(main_chart, main_cols[0]);
    frame.render_widget(side_panel, main_cols[1]);
}
```

### Centered Dialog

Modal dialog centered on screen:

```rust
use ratatui::layout::Rect;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_with_dialog(frame: &mut Frame) {
    // Render main content first
    let main_block = Block::default()
        .title("Main Application")
        .borders(Borders::ALL);
    frame.render_widget(main_block, frame.area());

    // Render centered dialog on top
    let dialog_area = centered_rect(60, 40, frame.area());

    // Clear the dialog area first
    frame.render_widget(Clear, dialog_area);

    let dialog = Block::default()
        .title("Confirm Action")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));
    frame.render_widget(dialog, dialog_area);
}
```

### Nested Layouts

Combining horizontal and vertical layouts for complex UIs:

```rust
fn render_complex_layout(frame: &mut Frame) {
    // Main vertical split
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),    // Header
            Constraint::Min(0),       // Body
        ])
        .split(frame.area());

    // Body horizontal split
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25),   // Sidebar
            Constraint::Min(0),       // Content area
        ])
        .split(main_chunks[1]);

    // Content area vertical split
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),       // Main content
            Constraint::Length(10),   // Bottom panel
        ])
        .split(body_chunks[1]);

    // Now render to each area
    frame.render_widget(header, main_chunks[0]);
    frame.render_widget(sidebar, body_chunks[0]);
    frame.render_widget(main_content, content_chunks[0]);
    frame.render_widget(bottom_panel, content_chunks[1]);
}
```

### Fill with Weights

Using Fill constraint for proportional distribution:

```rust
fn render_weighted_panels(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),  // Weight 1
            Constraint::Fill(2),  // Weight 2 (twice as wide)
            Constraint::Fill(1),  // Weight 1
        ])
        .split(frame.area());

    // Results in 25% - 50% - 25% distribution
    frame.render_widget(left, chunks[0]);
    frame.render_widget(center, chunks[1]);
    frame.render_widget(right, chunks[2]);
}
```

### Minimum with Fill

Guarantee minimum sizes while filling remaining space:

```rust
fn render_min_fill(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),     // At least 20, can grow
            Constraint::Length(1),   // Divider
            Constraint::Min(30),     // At least 30, can grow
        ])
        .split(frame.area());

    frame.render_widget(left_panel, chunks[0]);
    frame.render_widget(divider, chunks[1]);
    frame.render_widget(right_panel, chunks[2]);
}
```

## Helper Functions

### Centered Rect (Percentage)

Center a popup as a percentage of the parent:

```rust
/// Create a centered rect using percentages of the parent rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
```

### Centered Rect (Fixed Size)

Center a popup with fixed dimensions:

```rust
/// Create a centered rect with fixed width and height
fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;

    Rect::new(
        x,
        y,
        width.min(r.width),
        height.min(r.height),
    )
}
```

### Margin Rect

Add margins around a rect:

```rust
/// Create a rect with margins removed from all sides
fn margin_rect(margin: u16, r: Rect) -> Rect {
    Rect::new(
        r.x + margin,
        r.y + margin,
        r.width.saturating_sub(margin * 2),
        r.height.saturating_sub(margin * 2),
    )
}

/// Create a rect with different horizontal and vertical margins
fn margin_rect_hv(h_margin: u16, v_margin: u16, r: Rect) -> Rect {
    Rect::new(
        r.x + h_margin,
        r.y + v_margin,
        r.width.saturating_sub(h_margin * 2),
        r.height.saturating_sub(v_margin * 2),
    )
}
```

### Inner Rect (Block Content Area)

Get the inner area of a block (accounting for borders):

```rust
use ratatui::widgets::Block;

fn render_with_inner(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Outer")
        .borders(Borders::ALL);

    // Get the inner area (excludes borders and title)
    let inner_area = block.inner(area);

    // Render the outer block
    frame.render_widget(block, area);

    // Now use inner_area for content
    frame.render_widget(content, inner_area);
}
```

### Split with Spacing

Add gaps between layout chunks:

```rust
fn render_with_spacing(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)  // Outer margin
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    // For gaps between items, use Length constraints
    let chunks_with_gap = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(2),     // Gap
            Constraint::Percentage(45),
        ])
        .split(frame.area());

    frame.render_widget(left, chunks_with_gap[0]);
    // chunks_with_gap[1] is the gap (render nothing or a divider)
    frame.render_widget(right, chunks_with_gap[2]);
}
```

## Layout Debugging

### Visualize Layout Boundaries

Render borders around all areas to debug layout:

```rust
fn debug_layout(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Render debug borders with area info
    for (i, chunk) in chunks.iter().enumerate() {
        let debug_block = Block::default()
            .title(format!(
                "Area {}: {}x{} at ({},{})",
                i, chunk.width, chunk.height, chunk.x, chunk.y
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        frame.render_widget(debug_block, *chunk);
    }
}
```

### Constraint Behavior Reference

Understanding how constraints interact:

```rust
// Length is inflexible - takes exactly the specified size
Constraint::Length(10)  // Always 10 cells (if available)

// Percentage is relative to parent
Constraint::Percentage(50)  // Half of available space

// Min guarantees minimum, can expand
Constraint::Min(10)  // At least 10, takes more if available

// Max caps the maximum, can shrink
Constraint::Max(50)  // At most 50, takes less if needed

// Ratio is proportional
Constraint::Ratio(1, 3)  // Exactly 1/3 of space

// Fill distributes remaining space by weight
Constraint::Fill(1)  // Share remaining space equally
Constraint::Fill(2)  // Get twice the share of Fill(1)
```

## Responsive Patterns

### Adapt to Terminal Size

Adjust layout based on available space:

```rust
fn render_responsive(frame: &mut Frame) {
    let area = frame.area();

    if area.width >= 120 {
        // Wide layout: three columns
        render_three_column_layout(frame);
    } else if area.width >= 80 {
        // Medium layout: two columns
        render_two_column_layout(frame);
    } else {
        // Narrow layout: single column
        render_single_column_layout(frame);
    }
}
```

### Collapse Sidebar on Small Screens

```rust
struct App {
    show_sidebar: bool,
}

fn render_collapsible(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Auto-collapse sidebar on narrow terminals
    let show_sidebar = app.show_sidebar && area.width >= 80;

    if show_sidebar {
        let chunks = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .split(area);

        frame.render_widget(sidebar, chunks[0]);
        frame.render_widget(content, chunks[1]);
    } else {
        frame.render_widget(content, area);
    }
}
```
