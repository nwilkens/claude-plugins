# Ratatui Styling Guide

Complete guide to styling Ratatui terminal applications with the Style system.

## Style Struct

The `Style` struct is the foundation of all styling in Ratatui. It defines foreground color, background color, underline color, and text modifiers.

```rust
use ratatui::style::{Style, Color, Modifier};

// Create a style with method chaining
let style = Style::default()
    .fg(Color::White)
    .bg(Color::Blue)
    .add_modifier(Modifier::BOLD);

// Reset to terminal defaults
let reset_style = Style::reset();

// Patch one style with another (overrides set values)
let combined = base_style.patch(overlay_style);
```

## Colors

Ratatui supports multiple color formats for different terminal capabilities.

### Named Colors

Basic ANSI colors supported by virtually all terminals:

```rust
use ratatui::style::Color;

// Standard 8 colors
Color::Black
Color::Red
Color::Green
Color::Yellow
Color::Blue
Color::Magenta
Color::Cyan
Color::White

// Bright variants (light colors)
Color::Gray        // Equivalent to bright black
Color::DarkGray    // Dark gray
Color::LightRed
Color::LightGreen
Color::LightYellow
Color::LightBlue
Color::LightMagenta
Color::LightCyan
```

### Indexed Colors (256-color palette)

ANSI 256-color palette for terminals with extended color support:

```rust
use ratatui::style::Color;

// Index 0-15: Standard colors
// Index 16-231: 6x6x6 color cube
// Index 232-255: Grayscale ramp

let orange = Color::Indexed(208);
let purple = Color::Indexed(135);
let gray = Color::Indexed(244);

// Example: Create a grayscale
for i in 232..=255 {
    let shade = Color::Indexed(i);
}
```

### RGB Colors (True Color)

24-bit true color for terminals with RGB support:

```rust
use ratatui::style::Color;

// RGB values (0-255 for each channel)
let custom_blue = Color::Rgb(30, 58, 138);
let coral = Color::Rgb(255, 127, 80);
let slate = Color::Rgb(100, 116, 139);

// Hex-like colors
let dark_bg = Color::Rgb(0x0f, 0x17, 0x2a);
```

### Reset Color

Return to terminal default colors:

```rust
use ratatui::style::Color;

// Use terminal's default foreground/background
let default_style = Style::default()
    .fg(Color::Reset)
    .bg(Color::Reset);
```

## Modifiers

Text modifiers change how text is rendered. They are bitflags that can be combined.

### Available Modifiers

```rust
use ratatui::style::Modifier;

Modifier::BOLD          // Bold/bright text
Modifier::DIM           // Dimmed/faint text
Modifier::ITALIC        // Italic text
Modifier::UNDERLINED    // Underlined text
Modifier::SLOW_BLINK    // Slow blinking text
Modifier::RAPID_BLINK   // Rapid blinking text
Modifier::REVERSED      // Swap foreground and background
Modifier::HIDDEN        // Hidden/invisible text
Modifier::CROSSED_OUT   // Strikethrough text
```

### Combining Modifiers

Use bitwise OR to combine multiple modifiers:

```rust
use ratatui::style::{Style, Modifier};

// Combine with bitwise OR
let bold_italic = Style::default()
    .add_modifier(Modifier::BOLD | Modifier::ITALIC);

// Add modifiers incrementally
let style = Style::default()
    .add_modifier(Modifier::BOLD)
    .add_modifier(Modifier::UNDERLINED);

// Remove specific modifiers
let style = Style::default()
    .add_modifier(Modifier::BOLD | Modifier::ITALIC)
    .remove_modifier(Modifier::ITALIC);
```

## Stylize Trait (Fluent API)

The `Stylize` trait provides a convenient fluent API for inline styling.

```rust
use ratatui::style::Stylize;
use ratatui::text::{Span, Line, Text};

// Style strings directly
let text = "Hello".bold().italic().red().on_blue();

// Style Spans
let span = Span::from("World").green().bold();

// Style Lines
let line = Line::from("Status: OK").cyan().on_black();

// Chain multiple styles
let styled = "Error"
    .red()
    .bold()
    .underlined()
    .on_black();
```

### Stylize Color Methods

```rust
use ratatui::style::Stylize;

// Foreground colors
"text".black()
"text".red()
"text".green()
"text".yellow()
"text".blue()
"text".magenta()
"text".cyan()
"text".white()
"text".gray()
"text".dark_gray()
"text".light_red()
"text".light_green()
"text".light_yellow()
"text".light_blue()
"text".light_magenta()
"text".light_cyan()

// Background colors (prefix with on_)
"text".on_black()
"text".on_red()
"text".on_blue()
// ... etc
```

### Stylize Modifier Methods

```rust
use ratatui::style::Stylize;

"text".bold()
"text".dim()
"text".italic()
"text".underlined()
"text".slow_blink()
"text".rapid_blink()
"text".reversed()
"text".hidden()
"text".crossed_out()

// Remove modifiers
"text".not_bold()
"text".not_italic()
// ... etc
```

## Theme Patterns

Create a theme struct for consistent styling across your application:

```rust
use ratatui::style::{Style, Color, Modifier};

pub struct Theme {
    pub primary: Style,
    pub secondary: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub text: Style,
    pub text_muted: Style,
    pub text_highlight: Style,
    pub border: Style,
    pub border_focused: Style,
    pub background: Color,
    pub surface: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Style::default().fg(Color::Cyan),
            secondary: Style::default().fg(Color::Yellow),
            success: Style::default().fg(Color::Green),
            warning: Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            error: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            text: Style::default().fg(Color::White),
            text_muted: Style::default().fg(Color::DarkGray),
            text_highlight: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Gray),
            border_focused: Style::default().fg(Color::Cyan),
            background: Color::Reset,
            surface: Color::Rgb(30, 30, 40),
        }
    }
}

// Dark theme variant
impl Theme {
    pub fn dark() -> Self {
        Self {
            primary: Style::default().fg(Color::Rgb(96, 165, 250)),
            secondary: Style::default().fg(Color::Rgb(251, 191, 36)),
            success: Style::default().fg(Color::Rgb(34, 197, 94)),
            warning: Style::default().fg(Color::Rgb(251, 191, 36)),
            error: Style::default().fg(Color::Rgb(239, 68, 68)),
            text: Style::default().fg(Color::Rgb(226, 232, 240)),
            text_muted: Style::default().fg(Color::Rgb(148, 163, 184)),
            text_highlight: Style::default()
                .fg(Color::Rgb(226, 232, 240))
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Rgb(71, 85, 105)),
            border_focused: Style::default().fg(Color::Rgb(96, 165, 250)),
            background: Color::Rgb(15, 23, 42),
            surface: Color::Rgb(30, 41, 59),
        }
    }
}
```

## Applying Styles to Widgets

### Block

```rust
use ratatui::widgets::{Block, Borders};
use ratatui::style::{Style, Color};

let block = Block::default()
    .title("Panel")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Cyan))
    .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    .style(Style::default().bg(Color::Rgb(30, 41, 59)));
```

### Paragraph

```rust
use ratatui::widgets::Paragraph;
use ratatui::style::{Style, Color, Modifier};

let paragraph = Paragraph::new("Hello, World!")
    .style(Style::default().fg(Color::White).bg(Color::Black))
    .block(Block::default().borders(Borders::ALL));

// With styled text spans
use ratatui::text::{Line, Span};

let line = Line::from(vec![
    Span::styled("Error: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    Span::styled("File not found", Style::default().fg(Color::White)),
]);

let paragraph = Paragraph::new(line);
```

### List

```rust
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::style::{Style, Color, Modifier};

let items: Vec<ListItem> = vec![
    ListItem::new("Item 1").style(Style::default().fg(Color::White)),
    ListItem::new("Item 2").style(Style::default().fg(Color::Gray)),
    ListItem::new("Item 3 (selected)").style(Style::default().fg(Color::Green)),
];

let list = List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .bg(Color::Rgb(59, 130, 246))
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    )
    .highlight_symbol(">> ");
```

### Table

```rust
use ratatui::widgets::{Table, Row, Cell};
use ratatui::style::{Style, Color, Modifier};
use ratatui::layout::Constraint;

let header = Row::new(vec![
    Cell::from("Name").style(Style::default().add_modifier(Modifier::BOLD)),
    Cell::from("Value").style(Style::default().add_modifier(Modifier::BOLD)),
])
.style(Style::default().fg(Color::Cyan))
.height(1);

let rows = vec![
    Row::new(vec!["CPU", "45%"]),
    Row::new(vec!["Memory", "2.1 GB"]),
    Row::new(vec!["Disk", "120 GB"]),
];

let table = Table::new(rows, [Constraint::Length(10), Constraint::Length(10)])
    .header(header)
    .block(Block::default().title("Stats").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().bg(Color::DarkGray))
    .highlight_symbol(">> ");
```

### Gauge

```rust
use ratatui::widgets::Gauge;
use ratatui::style::{Style, Color, Modifier};

let gauge = Gauge::default()
    .block(Block::default().title("Progress").borders(Borders::ALL))
    .gauge_style(
        Style::default()
            .fg(Color::Green)
            .bg(Color::DarkGray)
    )
    .percent(65)
    .label("65%");
```

### Tabs

```rust
use ratatui::widgets::Tabs;
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::Line;

let titles = vec!["Tab1", "Tab2", "Tab3"]
    .into_iter()
    .map(Line::from)
    .collect();

let tabs = Tabs::new(titles)
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    )
    .select(0)
    .divider("|");
```

## Styled Text Composition

Build complex styled text with Span, Line, and Text:

```rust
use ratatui::text::{Span, Line, Text};
use ratatui::style::{Style, Color, Modifier, Stylize};

// Single styled span
let span = Span::styled(
    "Hello",
    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
);

// Line with multiple spans
let line = Line::from(vec![
    Span::raw("Status: "),
    Span::styled("OK", Style::default().fg(Color::Green)),
    Span::raw(" | "),
    Span::styled("3 warnings", Style::default().fg(Color::Yellow)),
]);

// Multi-line text
let text = Text::from(vec![
    Line::from("First line").style(Style::default().fg(Color::White)),
    Line::from("Second line").style(Style::default().fg(Color::Gray)),
    Line::from(vec![
        Span::styled("Error: ", Style::default().fg(Color::Red).bold()),
        Span::raw("Something went wrong"),
    ]),
]);

// Using Stylize trait for cleaner syntax
let text = Text::from(vec![
    Line::from("Header".bold().cyan()),
    Line::from("Normal text".white()),
    Line::from("Muted text".dark_gray()),
]);
```

## Underline Styling

Style the underline separately from text:

```rust
use ratatui::style::{Style, Color, Modifier};

let style = Style::default()
    .fg(Color::White)
    .add_modifier(Modifier::UNDERLINED)
    .underline_color(Color::Red);  // Red underline, white text
```

## Best Practices

### 1. Use a Theme Struct for Consistency

Centralize all style definitions in a theme struct:

```rust
// Good: Centralized theme
let title = Span::styled("Title", theme.text_highlight);
let error = Span::styled("Error", theme.error);

// Avoid: Scattered inline styles
let title = Span::styled("Title", Style::default().fg(Color::White).bold());
```

### 2. Prefer Named Colors for Portability

Named colors work on all terminals:

```rust
// Good: Works everywhere
Style::default().fg(Color::Cyan)

// Use with caution: Requires RGB support
Style::default().fg(Color::Rgb(96, 165, 250))
```

### 3. Use Stylize Trait for Inline Styling

The fluent API is more readable for simple styles:

```rust
// Good: Clean and readable
let text = "Status".green().bold();

// Verbose alternative
let text = Span::styled("Status", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
```

### 4. Check Terminal Capabilities

Consider fallbacks for limited terminals:

```rust
pub fn get_color(rgb: (u8, u8, u8), fallback: Color, supports_rgb: bool) -> Color {
    if supports_rgb {
        Color::Rgb(rgb.0, rgb.1, rgb.2)
    } else {
        fallback
    }
}

// Usage
let primary = get_color((96, 165, 250), Color::Cyan, terminal_supports_rgb);
```

### 5. Create Reusable Style Functions

```rust
impl Theme {
    pub fn status_style(&self, is_ok: bool) -> Style {
        if is_ok {
            self.success
        } else {
            self.error
        }
    }

    pub fn button_style(&self, focused: bool) -> Style {
        if focused {
            self.primary.add_modifier(Modifier::BOLD)
        } else {
            self.text_muted
        }
    }
}
```

### 6. Use Style::patch() for Layered Styles

```rust
let base = Style::default().fg(Color::White).bg(Color::Black);
let highlight = Style::default().add_modifier(Modifier::BOLD);

// Combine styles: highlight overrides only what it sets
let combined = base.patch(highlight);
// Result: fg=White, bg=Black, modifiers=BOLD
```

## Complete Theme Example

```rust
use ratatui::style::{Style, Color, Modifier};

/// Application theme with semantic color definitions
pub struct AppTheme {
    // Semantic colors
    pub primary: Style,
    pub secondary: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub info: Style,

    // Text styles
    pub text: Style,
    pub text_muted: Style,
    pub text_disabled: Style,
    pub text_highlight: Style,

    // Component styles
    pub border: Style,
    pub border_focused: Style,
    pub selection: Style,
    pub cursor: Style,

    // Background colors
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_surface: Color,
}

impl AppTheme {
    pub fn dark() -> Self {
        Self {
            // Semantic
            primary: Style::default().fg(Color::Rgb(96, 165, 250)),
            secondary: Style::default().fg(Color::Rgb(167, 139, 250)),
            success: Style::default().fg(Color::Rgb(34, 197, 94)),
            warning: Style::default().fg(Color::Rgb(251, 191, 36)),
            error: Style::default().fg(Color::Rgb(239, 68, 68)),
            info: Style::default().fg(Color::Rgb(56, 189, 248)),

            // Text
            text: Style::default().fg(Color::Rgb(226, 232, 240)),
            text_muted: Style::default().fg(Color::Rgb(148, 163, 184)),
            text_disabled: Style::default().fg(Color::Rgb(71, 85, 105)),
            text_highlight: Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD),

            // Components
            border: Style::default().fg(Color::Rgb(51, 65, 85)),
            border_focused: Style::default().fg(Color::Rgb(96, 165, 250)),
            selection: Style::default()
                .bg(Color::Rgb(59, 130, 246))
                .fg(Color::Rgb(255, 255, 255)),
            cursor: Style::default()
                .bg(Color::Rgb(96, 165, 250))
                .fg(Color::Rgb(15, 23, 42)),

            // Backgrounds
            bg_primary: Color::Rgb(15, 23, 42),
            bg_secondary: Color::Rgb(30, 41, 59),
            bg_surface: Color::Rgb(51, 65, 85),
        }
    }

    pub fn light() -> Self {
        Self {
            // Semantic
            primary: Style::default().fg(Color::Rgb(37, 99, 235)),
            secondary: Style::default().fg(Color::Rgb(124, 58, 237)),
            success: Style::default().fg(Color::Rgb(22, 163, 74)),
            warning: Style::default().fg(Color::Rgb(202, 138, 4)),
            error: Style::default().fg(Color::Rgb(220, 38, 38)),
            info: Style::default().fg(Color::Rgb(2, 132, 199)),

            // Text
            text: Style::default().fg(Color::Rgb(30, 41, 59)),
            text_muted: Style::default().fg(Color::Rgb(100, 116, 139)),
            text_disabled: Style::default().fg(Color::Rgb(148, 163, 184)),
            text_highlight: Style::default()
                .fg(Color::Rgb(15, 23, 42))
                .add_modifier(Modifier::BOLD),

            // Components
            border: Style::default().fg(Color::Rgb(203, 213, 225)),
            border_focused: Style::default().fg(Color::Rgb(37, 99, 235)),
            selection: Style::default()
                .bg(Color::Rgb(37, 99, 235))
                .fg(Color::Rgb(255, 255, 255)),
            cursor: Style::default()
                .bg(Color::Rgb(37, 99, 235))
                .fg(Color::Rgb(255, 255, 255)),

            // Backgrounds
            bg_primary: Color::Rgb(255, 255, 255),
            bg_secondary: Color::Rgb(248, 250, 252),
            bg_surface: Color::Rgb(241, 245, 249),
        }
    }
}
```
