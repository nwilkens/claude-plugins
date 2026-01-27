# Ratatui Widget Gallery

Comprehensive examples of all built-in Ratatui widgets and popular third-party crates.

## Built-in Widgets

### Block

Container with borders, titles, and padding:
```rust
use ratatui::{
    widgets::{Block, Borders, Padding},
    style::{Style, Color},
    Frame,
};

// Simple block with all borders
let block = Block::default()
    .borders(Borders::ALL)
    .title("My Block");

// Styled block with selective borders
let styled_block = Block::default()
    .borders(Borders::LEFT | Borders::RIGHT)
    .border_style(Style::default().fg(Color::Cyan))
    .title("Styled")
    .title_style(Style::default().fg(Color::Yellow));

// Block with padding
let padded_block = Block::default()
    .borders(Borders::ALL)
    .padding(Padding::new(2, 2, 1, 1)) // left, right, top, bottom
    .title("Padded Content");

// Block with multiple titles
let multi_title = Block::default()
    .borders(Borders::ALL)
    .title("Left Title")
    .title_bottom("Bottom Title");

frame.render_widget(block, area);
```

### Paragraph

Text display with wrapping, alignment, and styling:
```rust
use ratatui::{
    widgets::{Block, Borders, Paragraph, Wrap},
    style::{Style, Color, Modifier},
    text::{Line, Span, Text},
    layout::Alignment,
    Frame,
};

// Simple paragraph
let paragraph = Paragraph::new("Hello, Ratatui!")
    .block(Block::default().borders(Borders::ALL).title("Text"));

// Styled text with spans
let text = vec![
    Line::from(vec![
        Span::raw("Normal text, "),
        Span::styled("bold text", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(", and "),
        Span::styled("colored", Style::default().fg(Color::Green)),
    ]),
    Line::from("Second line"),
];

let styled_para = Paragraph::new(text)
    .block(Block::default().borders(Borders::ALL).title("Styled"))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

// Scrollable paragraph
let long_text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
let scrollable = Paragraph::new(long_text)
    .block(Block::default().borders(Borders::ALL).title("Scrollable"))
    .scroll((scroll_offset, 0)); // (vertical, horizontal) offset

frame.render_widget(paragraph, area);
```

### List

Vertical list with selection state:
```rust
use ratatui::{
    widgets::{Block, Borders, List, ListItem, ListState, ListDirection},
    style::{Style, Color, Modifier},
    Frame,
};

// Create list items
let items: Vec<ListItem> = vec![
    ListItem::new("Item 1"),
    ListItem::new("Item 2").style(Style::default().fg(Color::Yellow)),
    ListItem::new("Item 3"),
    ListItem::new("Item 4"),
];

// Create the list widget
let list = List::new(items)
    .block(Block::default().borders(Borders::ALL).title("List"))
    .highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    )
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom);

// Create and manage state
let mut state = ListState::default();
state.select(Some(0)); // Select first item

// Render with state
frame.render_stateful_widget(list, area, &mut state);

// Navigation helpers
fn next_item(state: &mut ListState, items_len: usize) {
    let i = match state.selected() {
        Some(i) => {
            if i >= items_len - 1 { 0 } else { i + 1 }
        }
        None => 0,
    };
    state.select(Some(i));
}

fn previous_item(state: &mut ListState, items_len: usize) {
    let i = match state.selected() {
        Some(i) => {
            if i == 0 { items_len - 1 } else { i - 1 }
        }
        None => 0,
    };
    state.select(Some(i));
}
```

### Table

Tabular data with column widths and row selection:
```rust
use ratatui::{
    widgets::{Block, Borders, Table, Row, Cell, TableState},
    style::{Style, Color, Modifier},
    layout::Constraint,
    Frame,
};

// Create table rows
let rows = vec![
    Row::new(vec![
        Cell::from("Alice"),
        Cell::from("30"),
        Cell::from("Engineer"),
    ]),
    Row::new(vec![
        Cell::from("Bob"),
        Cell::from("25"),
        Cell::from("Designer"),
    ]).style(Style::default().fg(Color::Yellow)),
    Row::new(vec![
        Cell::from("Charlie"),
        Cell::from("35"),
        Cell::from("Manager"),
    ]).height(2), // Multi-line row
];

// Create header
let header = Row::new(vec!["Name", "Age", "Role"])
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    .bottom_margin(1);

// Create table
let table = Table::new(
    rows,
    [
        Constraint::Length(15),      // Name column
        Constraint::Length(5),       // Age column
        Constraint::Percentage(50),  // Role column
    ]
)
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Users"))
    .highlight_style(Style::default().bg(Color::DarkGray))
    .highlight_symbol(">> ")
    .column_spacing(2);

// Table state for selection
let mut state = TableState::default();
state.select(Some(0));

frame.render_stateful_widget(table, area, &mut state);
```

### Gauge

Progress bar and percentage display:
```rust
use ratatui::{
    widgets::{Block, Borders, Gauge, LineGauge},
    style::{Style, Color, Modifier},
    Frame,
};

// Simple gauge (0.0 to 1.0)
let gauge = Gauge::default()
    .block(Block::default().borders(Borders::ALL).title("Progress"))
    .gauge_style(Style::default().fg(Color::Green))
    .ratio(0.75); // 75%

// Gauge with percentage display
let gauge_percent = Gauge::default()
    .block(Block::default().borders(Borders::ALL).title("Download"))
    .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
    .percent(42)
    .label("42% complete");

// Gauge with custom label
let custom_gauge = Gauge::default()
    .block(Block::default().borders(Borders::ALL).title("Upload"))
    .gauge_style(Style::default().fg(Color::Yellow))
    .ratio(0.6)
    .label(format!("{}/100 MB", 60))
    .use_unicode(true);

// Line gauge (thin progress bar)
let line_gauge = LineGauge::default()
    .block(Block::default().borders(Borders::ALL).title("Line Gauge"))
    .gauge_style(Style::default().fg(Color::Magenta))
    .line_set(symbols::line::THICK)
    .ratio(0.5);

frame.render_widget(gauge, area);
```

### Sparkline

Mini line charts for compact data visualization:
```rust
use ratatui::{
    widgets::{Block, Borders, Sparkline},
    style::{Style, Color},
    Frame,
};

let data = vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4];

let sparkline = Sparkline::default()
    .block(Block::default().borders(Borders::ALL).title("Sparkline"))
    .data(&data)
    .max(10)
    .style(Style::default().fg(Color::Green))
    .bar_set(symbols::bar::NINE_LEVELS);

// Different bar styles
let sparkline_half = Sparkline::default()
    .block(Block::default().borders(Borders::ALL).title("Half Blocks"))
    .data(&data)
    .bar_set(symbols::bar::HALF);

frame.render_widget(sparkline, area);
```

### BarChart

Vertical bar chart visualization:
```rust
use ratatui::{
    widgets::{Block, Borders, BarChart, Bar, BarGroup},
    style::{Style, Color},
    Frame,
};

// Simple bar chart with tuples
let data = [("Mon", 5), ("Tue", 8), ("Wed", 12), ("Thu", 6), ("Fri", 9)];

let barchart = BarChart::default()
    .block(Block::default().borders(Borders::ALL).title("Weekly Stats"))
    .data(&data)
    .bar_width(5)
    .bar_gap(2)
    .bar_style(Style::default().fg(Color::Cyan))
    .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

// Bar chart with groups (multiple data series)
let group1 = BarGroup::default()
    .label(Line::from("Q1"))
    .bars(&[
        Bar::default().value(10).label("A".into()).style(Style::default().fg(Color::Red)),
        Bar::default().value(15).label("B".into()).style(Style::default().fg(Color::Green)),
    ]);

let group2 = BarGroup::default()
    .label(Line::from("Q2"))
    .bars(&[
        Bar::default().value(12).label("A".into()).style(Style::default().fg(Color::Red)),
        Bar::default().value(18).label("B".into()).style(Style::default().fg(Color::Green)),
    ]);

let grouped_chart = BarChart::default()
    .block(Block::default().borders(Borders::ALL).title("Quarterly"))
    .data(vec![group1, group2])
    .bar_width(3)
    .group_gap(3);

frame.render_widget(barchart, area);
```

### Chart

Line and scatter charts with multiple datasets:
```rust
use ratatui::{
    widgets::{Block, Borders, Chart, Dataset, Axis, GraphType},
    style::{Style, Color, Modifier},
    symbols,
    text::Span,
    Frame,
};

// Prepare data points as (f64, f64) tuples
let data1: Vec<(f64, f64)> = vec![
    (0.0, 1.0), (1.0, 3.0), (2.0, 2.0), (3.0, 4.0), (4.0, 3.0), (5.0, 5.0)
];
let data2: Vec<(f64, f64)> = vec![
    (0.0, 2.0), (1.0, 1.0), (2.0, 3.0), (3.0, 2.0), (4.0, 4.0), (5.0, 3.0)
];

// Create datasets
let datasets = vec![
    Dataset::default()
        .name("Series A")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data1),
    Dataset::default()
        .name("Series B")
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Scatter)
        .style(Style::default().fg(Color::Yellow))
        .data(&data2),
];

// Create chart with axes
let chart = Chart::new(datasets)
    .block(Block::default().borders(Borders::ALL).title("Chart"))
    .x_axis(
        Axis::default()
            .title("X Axis")
            .style(Style::default().fg(Color::Gray))
            .bounds([0.0, 5.0])
            .labels(vec![
                Span::raw("0"),
                Span::raw("2.5"),
                Span::raw("5"),
            ])
    )
    .y_axis(
        Axis::default()
            .title("Y Axis")
            .style(Style::default().fg(Color::Gray))
            .bounds([0.0, 6.0])
            .labels(vec![
                Span::raw("0"),
                Span::raw("3"),
                Span::raw("6"),
            ])
    )
    .legend_position(Some(LegendPosition::TopRight))
    .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

frame.render_widget(chart, area);
```

### Tabs

Tab navigation widget:
```rust
use ratatui::{
    widgets::{Block, Borders, Tabs},
    style::{Style, Color, Modifier},
    text::Line,
    symbols,
    Frame,
};

let titles = vec!["Tab 1", "Tab 2", "Tab 3", "Tab 4"];

let tabs = Tabs::new(titles)
    .block(Block::default().borders(Borders::ALL).title("Tabs"))
    .select(current_tab_index)
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    )
    .divider(symbols::DOT)
    .padding(" ", " ");

frame.render_widget(tabs, area);

// Handle tab switching in your event loop
fn handle_key(&mut self, key: KeyCode) {
    match key {
        KeyCode::Right => {
            self.current_tab = (self.current_tab + 1) % self.tab_count;
        }
        KeyCode::Left => {
            self.current_tab = self.current_tab.saturating_sub(1);
        }
        _ => {}
    }
}
```

### Canvas

Custom drawing with shapes and primitives:
```rust
use ratatui::{
    widgets::{
        Block, Borders,
        canvas::{Canvas, Line, Rectangle, Circle, Map, MapResolution, Points},
    },
    style::{Style, Color},
    Frame,
};

// Canvas with shapes
let canvas = Canvas::default()
    .block(Block::default().borders(Borders::ALL).title("Canvas"))
    .x_bounds([-180.0, 180.0])
    .y_bounds([-90.0, 90.0])
    .paint(|ctx| {
        // Draw a rectangle
        ctx.draw(&Rectangle {
            x: -50.0,
            y: -25.0,
            width: 100.0,
            height: 50.0,
            color: Color::Green,
        });

        // Draw a line
        ctx.draw(&Line {
            x1: -180.0,
            y1: 0.0,
            x2: 180.0,
            y2: 0.0,
            color: Color::White,
        });

        // Draw a circle
        ctx.draw(&Circle {
            x: 0.0,
            y: 0.0,
            radius: 30.0,
            color: Color::Cyan,
        });

        // Draw points
        ctx.draw(&Points {
            coords: &[(10.0, 10.0), (20.0, 20.0), (30.0, 15.0)],
            color: Color::Yellow,
        });

        // Print text on canvas
        ctx.print(0.0, 0.0, Line::from("Center").style(Style::default().fg(Color::White)));
    })
    .marker(symbols::Marker::Braille); // or Dot, Block

// World map canvas
let map_canvas = Canvas::default()
    .block(Block::default().borders(Borders::ALL).title("World Map"))
    .x_bounds([-180.0, 180.0])
    .y_bounds([-90.0, 90.0])
    .paint(|ctx| {
        ctx.draw(&Map {
            color: Color::Green,
            resolution: MapResolution::High,
        });
    });

frame.render_widget(canvas, area);
```

### Clear

Clear a rectangular region:
```rust
use ratatui::{
    widgets::Clear,
    Frame,
};

// Clear a region (useful for popups/modals)
frame.render_widget(Clear, popup_area);

// Then render your popup content on top
frame.render_widget(popup_content, popup_area);
```

### Scrollbar

Scrollbar widget for indicating scroll position:
```rust
use ratatui::{
    widgets::{Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState},
    style::{Style, Color},
    Frame,
};

// Vertical scrollbar
let scrollbar = Scrollbar::default()
    .orientation(ScrollbarOrientation::VerticalRight)
    .begin_symbol(Some("^"))
    .end_symbol(Some("v"))
    .track_symbol(Some("|"))
    .thumb_symbol("█");

let mut scrollbar_state = ScrollbarState::default()
    .content_length(100)  // Total content length
    .position(25);        // Current scroll position

frame.render_stateful_widget(
    scrollbar,
    area,
    &mut scrollbar_state
);

// Horizontal scrollbar
let h_scrollbar = Scrollbar::default()
    .orientation(ScrollbarOrientation::HorizontalBottom)
    .thumb_style(Style::default().fg(Color::Cyan));

frame.render_stateful_widget(
    h_scrollbar,
    area,
    &mut scrollbar_state
);
```

## Stateful Widgets

Ratatui uses the `StatefulWidget` trait for widgets that maintain external state.

### Understanding StatefulWidget

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

// The StatefulWidget trait
pub trait StatefulWidget {
    type State;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}

// Render a stateful widget
frame.render_stateful_widget(widget, area, &mut state);

// vs regular widget
frame.render_widget(widget, area);
```

### ListState Pattern

```rust
use ratatui::widgets::ListState;

struct App {
    items: Vec<String>,
    list_state: ListState,
}

impl App {
    fn new(items: Vec<String>) -> Self {
        let mut list_state = ListState::default();
        if !items.is_empty() {
            list_state.select(Some(0));
        }
        Self { items, list_state }
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

    fn selected_item(&self) -> Option<&String> {
        self.list_state.selected().map(|i| &self.items[i])
    }
}
```

### TableState Pattern

```rust
use ratatui::widgets::TableState;

struct TableApp {
    rows: Vec<Vec<String>>,
    table_state: TableState,
}

impl TableApp {
    fn new(rows: Vec<Vec<String>>) -> Self {
        Self {
            rows,
            table_state: TableState::default().with_selected(0),
        }
    }

    fn next_row(&mut self) {
        let i = self.table_state.selected().unwrap_or(0);
        let next = if i >= self.rows.len() - 1 { 0 } else { i + 1 };
        self.table_state.select(Some(next));
    }

    fn previous_row(&mut self) {
        let i = self.table_state.selected().unwrap_or(0);
        let prev = if i == 0 { self.rows.len() - 1 } else { i - 1 };
        self.table_state.select(Some(prev));
    }

    // Offset for implementing virtual scrolling
    fn scroll_offset(&self) -> usize {
        self.table_state.offset()
    }
}
```

### ScrollbarState Pattern

```rust
use ratatui::widgets::ScrollbarState;

struct ScrollableContent {
    content: Vec<String>,
    scroll_position: usize,
    scrollbar_state: ScrollbarState,
}

impl ScrollableContent {
    fn new(content: Vec<String>) -> Self {
        let len = content.len();
        Self {
            content,
            scroll_position: 0,
            scrollbar_state: ScrollbarState::default().content_length(len),
        }
    }

    fn scroll_down(&mut self) {
        self.scroll_position = self.scroll_position.saturating_add(1);
        self.scrollbar_state = self.scrollbar_state.position(self.scroll_position);
    }

    fn scroll_up(&mut self) {
        self.scroll_position = self.scroll_position.saturating_sub(1);
        self.scrollbar_state = self.scrollbar_state.position(self.scroll_position);
    }
}
```

## Third-Party Widget Crates

### tui-input

Text input widget for single-line input:
```rust
// Cargo.toml: tui-input = "0.8"

use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

struct App {
    input: Input,
    input_mode: InputMode,
}

impl App {
    fn new() -> Self {
        Self {
            input: Input::default(),
            input_mode: InputMode::Normal,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Editing => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
            }
            _ => {}
        }
    }
}

// Rendering
fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let width = area.width.saturating_sub(2) as usize; // Account for borders
    let scroll = app.input.visual_scroll(width);

    let input = Paragraph::new(app.input.value())
        .style(Style::default().fg(Color::Yellow))
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, area);

    // Position cursor
    if matches!(app.input_mode, InputMode::Editing) {
        f.set_cursor_position((
            area.x + (app.input.visual_cursor().saturating_sub(scroll) as u16) + 1,
            area.y + 1,
        ));
    }
}
```

### tui-textarea

Multi-line text editor with advanced features:
```rust
// Cargo.toml: tui-textarea = "0.5"

use tui_textarea::{TextArea, CursorMove, Input, Key};

struct Editor<'a> {
    textarea: TextArea<'a>,
}

impl<'a> Editor<'a> {
    fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_block(Block::default().borders(Borders::ALL).title("Editor"));
        textarea.set_cursor_line_style(Style::default().bg(Color::DarkGray));
        textarea.set_line_number_style(Style::default().fg(Color::Gray));
        Self { textarea }
    }

    fn with_content(content: &str) -> Self {
        let lines: Vec<String> = content.lines().map(String::from).collect();
        let mut textarea = TextArea::new(lines);
        textarea.set_block(Block::default().borders(Borders::ALL).title("Editor"));
        Self { textarea }
    }

    fn handle_input(&mut self, input: Input) -> bool {
        match input {
            Input { key: Key::Esc, .. } => false, // Exit edit mode
            input => {
                self.textarea.input(input);
                true
            }
        }
    }

    fn content(&self) -> String {
        self.textarea.lines().join("\n")
    }
}

// Rendering
fn render_editor(f: &mut Frame, editor: &Editor, area: Rect) {
    f.render_widget(editor.textarea.widget(), area);
}
```

### tui-tree-widget

Tree view for hierarchical data:
```rust
// Cargo.toml: tui-tree-widget = "0.21"

use tui_tree_widget::{Tree, TreeItem, TreeState};

struct FileTree<'a> {
    items: Vec<TreeItem<'a, &'a str>>,
    state: TreeState<&'a str>,
}

impl<'a> FileTree<'a> {
    fn new() -> Self {
        let items = vec![
            TreeItem::new("src", "src/", vec![
                TreeItem::new_leaf("main", "main.rs"),
                TreeItem::new_leaf("lib", "lib.rs"),
                TreeItem::new("modules", "modules/", vec![
                    TreeItem::new_leaf("mod1", "mod1.rs"),
                    TreeItem::new_leaf("mod2", "mod2.rs"),
                ]),
            ]),
            TreeItem::new("tests", "tests/", vec![
                TreeItem::new_leaf("test1", "test_main.rs"),
            ]),
            TreeItem::new_leaf("cargo", "Cargo.toml"),
        ];

        Self {
            items,
            state: TreeState::default(),
        }
    }

    fn toggle(&mut self) {
        self.state.toggle_selected();
    }

    fn select_next(&mut self) {
        self.state.key_down();
    }

    fn select_previous(&mut self) {
        self.state.key_up();
    }
}

// Rendering
fn render_tree(f: &mut Frame, tree: &mut FileTree, area: Rect) {
    let widget = Tree::new(&tree.items)
        .expect("valid tree")
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");

    f.render_stateful_widget(widget, area, &mut tree.state);
}
```

### ratatui-image

Display images in the terminal:
```rust
// Cargo.toml: ratatui-image = "1.0"

use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};
use image::io::Reader as ImageReader;

struct ImageViewer {
    image: Box<dyn StatefulProtocol>,
}

impl ImageViewer {
    fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a picker to detect terminal capabilities
        let mut picker = Picker::from_termios()?;
        picker.guess_protocol();

        // Load the image
        let dyn_img = ImageReader::open(path)?.decode()?;

        // Create the protocol
        let image = picker.new_resize_protocol(dyn_img);

        Ok(Self { image })
    }
}

// Rendering
fn render_image(f: &mut Frame, viewer: &mut ImageViewer, area: Rect) {
    let image_widget = StatefulImage::new(None);
    f.render_stateful_widget(image_widget, area, &mut viewer.image);
}
```

## Custom Widget Implementation

### Implementing the Widget Trait

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Color, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// A custom status bar widget
struct StatusBar<'a> {
    mode: &'a str,
    filename: &'a str,
    position: (usize, usize), // (line, column)
    modified: bool,
    block: Option<Block<'a>>,
}

impl<'a> StatusBar<'a> {
    fn new(mode: &'a str, filename: &'a str) -> Self {
        Self {
            mode,
            filename,
            position: (1, 1),
            modified: false,
            block: None,
        }
    }

    fn position(mut self, line: usize, col: usize) -> Self {
        self.position = (line, col);
        self
    }

    fn modified(mut self, modified: bool) -> Self {
        self.modified = modified;
        self
    }

    fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate inner area if block is present
        let inner = match &self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if inner.height == 0 || inner.width == 0 {
            return;
        }

        // Build the status line
        let mode_style = Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD);

        let modified_indicator = if self.modified { " [+]" } else { "" };

        let left = Line::from(vec![
            Span::styled(format!(" {} ", self.mode.to_uppercase()), mode_style),
            Span::raw(" "),
            Span::raw(self.filename),
            Span::styled(modified_indicator, Style::default().fg(Color::Yellow)),
        ]);

        let right = format!(" {}:{} ", self.position.0, self.position.1);

        // Render left-aligned content
        buf.set_line(inner.x, inner.y, &left, inner.width);

        // Render right-aligned content
        let right_x = inner.x + inner.width.saturating_sub(right.len() as u16);
        buf.set_string(right_x, inner.y, &right, Style::default().fg(Color::Gray));
    }
}

// Usage
fn render_status(f: &mut Frame, area: Rect) {
    let status = StatusBar::new("normal", "main.rs")
        .position(42, 15)
        .modified(true);

    f.render_widget(status, area);
}
```

### Implementing StatefulWidget

```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Color, Modifier},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

/// State for a paginated list
#[derive(Default)]
struct PaginatedListState {
    selected: usize,
    page: usize,
    page_size: usize,
}

impl PaginatedListState {
    fn new(page_size: usize) -> Self {
        Self {
            selected: 0,
            page: 0,
            page_size,
        }
    }

    fn next(&mut self, total_items: usize) {
        if self.selected < total_items.saturating_sub(1) {
            self.selected += 1;
            // Update page if needed
            self.page = self.selected / self.page_size;
        }
    }

    fn previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.page = self.selected / self.page_size;
    }

    fn next_page(&mut self, total_items: usize) {
        let max_page = total_items.saturating_sub(1) / self.page_size;
        if self.page < max_page {
            self.page += 1;
            self.selected = self.page * self.page_size;
        }
    }

    fn previous_page(&mut self) {
        if self.page > 0 {
            self.page -= 1;
            self.selected = self.page * self.page_size;
        }
    }
}

/// A paginated list widget
struct PaginatedList<'a> {
    items: &'a [String],
    block: Option<Block<'a>>,
    highlight_style: Style,
}

impl<'a> PaginatedList<'a> {
    fn new(items: &'a [String]) -> Self {
        Self {
            items,
            block: None,
            highlight_style: Style::default().add_modifier(Modifier::REVERSED),
        }
    }

    fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }
}

impl StatefulWidget for PaginatedList<'_> {
    type State = PaginatedListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Render block if present
        let inner = match &self.block {
            Some(block) => {
                let inner = block.inner(area);
                block.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if inner.height == 0 || inner.width == 0 {
            return;
        }

        // Calculate visible items
        let start = state.page * state.page_size;
        let end = (start + state.page_size).min(self.items.len());

        for (i, item) in self.items[start..end].iter().enumerate() {
            let y = inner.y + i as u16;
            if y >= inner.y + inner.height {
                break;
            }

            let global_index = start + i;
            let style = if global_index == state.selected {
                self.highlight_style
            } else {
                Style::default()
            };

            let prefix = if global_index == state.selected { ">> " } else { "   " };
            let line = format!("{}{}", prefix, item);
            buf.set_string(inner.x, y, &line, style);
        }

        // Render page indicator
        let total_pages = (self.items.len() + state.page_size - 1) / state.page_size;
        let indicator = format!(" Page {}/{} ", state.page + 1, total_pages);
        let x = inner.x + inner.width.saturating_sub(indicator.len() as u16);
        buf.set_string(x, inner.y + inner.height - 1, &indicator, Style::default().fg(Color::Gray));
    }
}

// Usage
fn render_paginated(f: &mut Frame, items: &[String], state: &mut PaginatedListState, area: Rect) {
    let list = PaginatedList::new(items)
        .block(Block::default().borders(Borders::ALL).title("Items"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, area, state);
}
```

## Widget Composition Patterns

### Form Layout

```rust
use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    Frame,
};

fn render_form(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Name field
            Constraint::Length(3), // Email field
            Constraint::Length(3), // Password field
            Constraint::Length(3), // Buttons
            Constraint::Min(0),    // Spacer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Registration Form")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default());
    f.render_widget(title, chunks[0]);

    // Name input
    let name = Paragraph::new("John Doe")
        .block(Block::default().borders(Borders::ALL).title("Name"));
    f.render_widget(name, chunks[1]);

    // Email input
    let email = Paragraph::new("john@example.com")
        .block(Block::default().borders(Borders::ALL).title("Email"));
    f.render_widget(email, chunks[2]);

    // Password input
    let password = Paragraph::new("********")
        .block(Block::default().borders(Borders::ALL).title("Password"));
    f.render_widget(password, chunks[3]);

    // Buttons
    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[4]);

    let submit = Paragraph::new(" Submit ")
        .style(Style::default().fg(Color::Black).bg(Color::Green))
        .block(Block::default().borders(Borders::ALL));
    let cancel = Paragraph::new(" Cancel ")
        .style(Style::default().fg(Color::Black).bg(Color::Red))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(submit, buttons[0]);
    f.render_widget(cancel, buttons[1]);
}
```

### Dashboard Layout

```rust
use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Block, Borders, Paragraph, List, ListItem, Gauge, BarChart},
    style::{Style, Color, Modifier},
    Frame,
};

fn render_dashboard(f: &mut Frame, area: Rect) {
    // Main layout: header, content, footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    // Header
    let header = Paragraph::new(" Dashboard - System Monitor ")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, main_chunks[0]);

    // Content: sidebar + main area
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Sidebar
            Constraint::Percentage(80), // Main
        ])
        .split(main_chunks[1]);

    // Sidebar menu
    let menu_items: Vec<ListItem> = vec![
        ListItem::new("Overview"),
        ListItem::new("Processes"),
        ListItem::new("Network"),
        ListItem::new("Storage"),
    ];
    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_widget(menu, content_chunks[0]);

    // Main area: stats row + charts
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Stats
            Constraint::Min(0),    // Charts
        ])
        .split(content_chunks[1]);

    // Stats row
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(main_chunks[0]);

    let cpu_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(65);
    f.render_widget(cpu_gauge, stats_chunks[0]);

    let mem_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Memory"))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(42);
    f.render_widget(mem_gauge, stats_chunks[1]);

    let disk_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Disk"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(78);
    f.render_widget(disk_gauge, stats_chunks[2]);

    // Charts area
    let chart_data = [("Mon", 5), ("Tue", 8), ("Wed", 12), ("Thu", 6), ("Fri", 9)];
    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Weekly Activity"))
        .data(&chart_data)
        .bar_width(7)
        .bar_style(Style::default().fg(Color::Cyan));
    f.render_widget(chart, main_chunks[1]);

    // Footer
    let footer = Paragraph::new(" Press 'q' to quit | Arrow keys to navigate ")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(footer, main_chunks[2]);
}
```

### Modal/Popup Pattern

```rust
use ratatui::{
    layout::{Layout, Constraint, Rect, Flex},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    style::{Style, Color},
    Frame,
};

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn render_with_popup(f: &mut Frame, show_popup: bool) {
    // Render main content
    let main = Paragraph::new("Main application content here...")
        .block(Block::default().borders(Borders::ALL).title("Main"));
    f.render_widget(main, f.area());

    // Render popup if needed
    if show_popup {
        let popup_area = centered_rect(60, 40, f.area());

        // Clear the popup area first
        f.render_widget(Clear, popup_area);

        // Render popup content
        let popup = Paragraph::new("Are you sure you want to quit?\n\nPress 'y' to confirm or 'n' to cancel.")
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .title("Confirm")
            );
        f.render_widget(popup, popup_area);
    }
}
```
