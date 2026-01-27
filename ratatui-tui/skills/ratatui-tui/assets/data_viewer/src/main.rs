//! Data Viewer TUI Application
//!
//! A Ratatui-based data viewer demonstrating:
//! - Table widget with TableState for data display
//! - JSON and CSV file loading and parsing
//! - Modal popup dialogs (centered rect)
//! - Tabs widget for switching views
//! - Two-panel split layout (file list + data view)

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table,
        TableState, Tabs, Wrap,
    },
    Frame, Terminal,
};
use serde_json::Value;
use std::{
    fs,
    io::{self, Stdout},
    path::{Path, PathBuf},
};

/// Active panel in the two-panel layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActivePanel {
    FileList,
    DataView,
}

/// Active tab in the data view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActiveTab {
    Table,
    Raw,
}

impl ActiveTab {
    fn index(&self) -> usize {
        match self {
            ActiveTab::Table => 0,
            ActiveTab::Raw => 1,
        }
    }

    fn next(&self) -> Self {
        match self {
            ActiveTab::Table => ActiveTab::Raw,
            ActiveTab::Raw => ActiveTab::Table,
        }
    }
}

/// Modal dialog state
#[derive(Debug, Clone)]
struct ModalDialog {
    title: String,
    message: String,
}

/// Loaded data representation
#[derive(Debug, Clone)]
struct LoadedData {
    file_name: String,
    file_type: String,
    raw_content: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

/// Main application state
struct App {
    /// List of files in the current directory
    file_list: Vec<PathBuf>,
    /// State for the file list widget
    file_list_state: ListState,
    /// Currently selected file path
    selected_file: Option<PathBuf>,
    /// Loaded data from the selected file
    table_data: Option<LoadedData>,
    /// State for the table widget
    table_state: TableState,
    /// Currently active tab
    active_tab: ActiveTab,
    /// Currently active panel
    active_panel: ActivePanel,
    /// Current directory being viewed
    current_dir: PathBuf,
    /// Modal dialog (if any)
    modal: Option<ModalDialog>,
    /// Whether the app should quit
    should_quit: bool,
}

impl App {
    /// Create a new App instance
    fn new() -> Result<Self> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let mut app = App {
            file_list: Vec::new(),
            file_list_state: ListState::default(),
            selected_file: None,
            table_data: None,
            table_state: TableState::default(),
            active_tab: ActiveTab::Table,
            active_panel: ActivePanel::FileList,
            current_dir,
            modal: None,
            should_quit: false,
        };
        app.refresh_file_list()?;
        if !app.file_list.is_empty() {
            app.file_list_state.select(Some(0));
        }
        Ok(app)
    }

    /// Refresh the file list from the current directory
    fn refresh_file_list(&mut self) -> Result<()> {
        self.file_list.clear();

        // Add parent directory entry if not at root
        if let Some(parent) = self.current_dir.parent() {
            self.file_list.push(parent.to_path_buf());
        }

        let entries = fs::read_dir(&self.current_dir)
            .context("Failed to read directory")?;

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "json" || ext_lower == "csv" {
                    files.push(path);
                }
            }
        }

        // Sort directories and files
        dirs.sort();
        files.sort();

        // Add directories first, then files
        self.file_list.extend(dirs);
        self.file_list.extend(files);

        Ok(())
    }

    /// Navigate to the selected file or directory
    fn select_current(&mut self) {
        if let Some(selected) = self.file_list_state.selected() {
            if let Some(path) = self.file_list.get(selected).cloned() {
                if path.is_dir() {
                    self.current_dir = path;
                    if let Err(e) = self.refresh_file_list() {
                        self.show_error("Navigation Error", &e.to_string());
                    } else {
                        self.file_list_state.select(Some(0));
                    }
                } else {
                    self.load_file(&path);
                }
            }
        }
    }

    /// Load a file and parse its contents
    fn load_file(&mut self, path: &Path) {
        let ext = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let result = match ext.as_str() {
            "json" => self.load_json(path),
            "csv" => self.load_csv(path),
            _ => {
                self.show_error("Unsupported File", "Only JSON and CSV files are supported");
                return;
            }
        };

        match result {
            Ok(data) => {
                self.selected_file = Some(path.to_path_buf());
                self.table_data = Some(data);
                self.table_state.select(Some(0));
                self.active_panel = ActivePanel::DataView;
            }
            Err(e) => {
                self.show_error("Load Error", &e.to_string());
            }
        }
    }

    /// Load and parse a JSON file
    fn load_json(&self, path: &Path) -> Result<LoadedData> {
        let content = fs::read_to_string(path).context("Failed to read file")?;
        let value: Value = serde_json::from_str(&content).context("Failed to parse JSON")?;

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // Try to extract tabular data from JSON
        let (headers, rows) = self.json_to_table(&value);

        Ok(LoadedData {
            file_name,
            file_type: "JSON".to_string(),
            raw_content: serde_json::to_string_pretty(&value)
                .unwrap_or_else(|_| content.clone()),
            headers,
            rows,
        })
    }

    /// Convert JSON value to table format
    fn json_to_table(&self, value: &Value) -> (Vec<String>, Vec<Vec<String>>) {
        match value {
            Value::Array(arr) => {
                // Check if it's an array of objects
                if arr.is_empty() {
                    return (vec!["(empty)".to_string()], vec![]);
                }

                if let Some(Value::Object(first)) = arr.first() {
                    let headers: Vec<String> = first.keys().cloned().collect();
                    let rows: Vec<Vec<String>> = arr
                        .iter()
                        .filter_map(|item| {
                            if let Value::Object(obj) = item {
                                Some(
                                    headers
                                        .iter()
                                        .map(|h| self.value_to_string(obj.get(h)))
                                        .collect(),
                                )
                            } else {
                                None
                            }
                        })
                        .collect();
                    return (headers, rows);
                }

                // Array of primitives
                let headers = vec!["Index".to_string(), "Value".to_string()];
                let rows: Vec<Vec<String>> = arr
                    .iter()
                    .enumerate()
                    .map(|(i, v)| vec![i.to_string(), self.value_to_string(Some(v))])
                    .collect();
                (headers, rows)
            }
            Value::Object(obj) => {
                let headers = vec!["Key".to_string(), "Value".to_string()];
                let rows: Vec<Vec<String>> = obj
                    .iter()
                    .map(|(k, v)| vec![k.clone(), self.value_to_string(Some(v))])
                    .collect();
                (headers, rows)
            }
            _ => {
                let headers = vec!["Value".to_string()];
                let rows = vec![vec![self.value_to_string(Some(value))]];
                (headers, rows)
            }
        }
    }

    /// Convert a JSON value to a display string
    fn value_to_string(&self, value: Option<&Value>) -> String {
        match value {
            None => String::new(),
            Some(Value::Null) => "null".to_string(),
            Some(Value::Bool(b)) => b.to_string(),
            Some(Value::Number(n)) => n.to_string(),
            Some(Value::String(s)) => s.clone(),
            Some(Value::Array(arr)) => format!("[{} items]", arr.len()),
            Some(Value::Object(obj)) => format!("{{{} keys}}", obj.len()),
        }
    }

    /// Load and parse a CSV file
    fn load_csv(&self, path: &Path) -> Result<LoadedData> {
        let content = fs::read_to_string(path).context("Failed to read file")?;
        let mut reader = csv::Reader::from_reader(content.as_bytes());

        let headers: Vec<String> = reader
            .headers()
            .context("Failed to read CSV headers")?
            .iter()
            .map(|s| s.to_string())
            .collect();

        let rows: Vec<Vec<String>> = reader
            .records()
            .filter_map(|r| r.ok())
            .map(|r| r.iter().map(|s| s.to_string()).collect())
            .collect();

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(LoadedData {
            file_name,
            file_type: "CSV".to_string(),
            raw_content: content,
            headers,
            rows,
        })
    }

    /// Show an error modal
    fn show_error(&mut self, title: &str, message: &str) {
        self.modal = Some(ModalDialog {
            title: title.to_string(),
            message: message.to_string(),
        });
    }

    /// Dismiss the modal dialog
    fn dismiss_modal(&mut self) {
        self.modal = None;
    }

    /// Move selection up in the current panel
    fn move_up(&mut self) {
        match self.active_panel {
            ActivePanel::FileList => {
                if let Some(selected) = self.file_list_state.selected() {
                    if selected > 0 {
                        self.file_list_state.select(Some(selected - 1));
                    }
                }
            }
            ActivePanel::DataView => {
                if let Some(selected) = self.table_state.selected() {
                    if selected > 0 {
                        self.table_state.select(Some(selected - 1));
                    }
                }
            }
        }
    }

    /// Move selection down in the current panel
    fn move_down(&mut self) {
        match self.active_panel {
            ActivePanel::FileList => {
                if let Some(selected) = self.file_list_state.selected() {
                    if selected < self.file_list.len().saturating_sub(1) {
                        self.file_list_state.select(Some(selected + 1));
                    }
                }
            }
            ActivePanel::DataView => {
                if let Some(data) = &self.table_data {
                    if let Some(selected) = self.table_state.selected() {
                        if selected < data.rows.len().saturating_sub(1) {
                            self.table_state.select(Some(selected + 1));
                        }
                    }
                }
            }
        }
    }

    /// Switch between panels
    fn switch_panel(&mut self) {
        self.active_panel = match self.active_panel {
            ActivePanel::FileList => ActivePanel::DataView,
            ActivePanel::DataView => ActivePanel::FileList,
        };
    }

    /// Switch to the next tab
    fn next_tab(&mut self) {
        self.active_tab = self.active_tab.next();
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyCode) {
        // Handle modal first
        if self.modal.is_some() {
            match key {
                KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ') => {
                    self.dismiss_modal();
                }
                _ => {}
            }
            return;
        }

        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Tab => {
                self.switch_panel();
            }
            KeyCode::Char('1') | KeyCode::Char('2') => {
                self.next_tab();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.move_down();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.move_up();
            }
            KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
                if self.active_panel == ActivePanel::FileList {
                    self.select_current();
                }
            }
            KeyCode::Char('h') | KeyCode::Left => {
                if self.active_panel == ActivePanel::FileList {
                    // Navigate to parent directory
                    if let Some(parent) = self.current_dir.parent() {
                        self.current_dir = parent.to_path_buf();
                        let _ = self.refresh_file_list();
                        self.file_list_state.select(Some(0));
                    }
                }
            }
            KeyCode::Char('r') => {
                let _ = self.refresh_file_list();
            }
            _ => {}
        }
    }
}

/// Create a centered rect for modal dialogs
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Render the UI
fn ui(frame: &mut Frame, app: &mut App) {
    // Create the main layout: two panels side by side
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    // Render file list panel (left)
    render_file_list(frame, app, main_chunks[0]);

    // Render data view panel (right)
    render_data_view(frame, app, main_chunks[1]);

    // Render modal dialog if present
    if let Some(modal) = &app.modal {
        render_modal(frame, modal);
    }
}

/// Render the file list panel
fn render_file_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let is_active = app.active_panel == ActivePanel::FileList;
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title = format!(" Files: {} ", app.current_dir.display());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let items: Vec<ListItem> = app
        .file_list
        .iter()
        .map(|path| {
            let name = if path == &app.current_dir.parent().unwrap_or(path) {
                "..".to_string()
            } else {
                path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string())
            };

            let (icon, style) = if path.is_dir() {
                ("[D]", Style::default().fg(Color::Blue).bold())
            } else {
                let ext = path
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                match ext.as_str() {
                    "json" => ("[J]", Style::default().fg(Color::Yellow)),
                    "csv" => ("[C]", Style::default().fg(Color::Green)),
                    _ => ("[F]", Style::default()),
                }
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", icon), style),
                Span::raw(name),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.file_list_state);
}

/// Render the data view panel with tabs
fn render_data_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let is_active = app.active_panel == ActivePanel::DataView;
    let border_style = if is_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    // Split area for tabs and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Render tabs
    let titles = vec!["Table", "Raw"];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .title(" Data View ")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .select(app.active_tab.index())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, chunks[0]);

    // Render content based on active tab
    let content_block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style);

    match app.active_tab {
        ActiveTab::Table => {
            render_table_view(frame, app, chunks[1], content_block);
        }
        ActiveTab::Raw => {
            render_raw_view(frame, app, chunks[1], content_block);
        }
    }
}

/// Render the table view
fn render_table_view(frame: &mut Frame, app: &mut App, area: Rect, block: Block) {
    if let Some(data) = &app.table_data {
        if data.headers.is_empty() {
            let paragraph = Paragraph::new("No tabular data available")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(paragraph, area);
            return;
        }

        // Calculate column widths
        let col_count = data.headers.len();
        let available_width = area.width.saturating_sub(2) as usize; // Account for borders
        let col_width = (available_width / col_count).max(10);

        let header_cells: Vec<Cell> = data
            .headers
            .iter()
            .map(|h| {
                Cell::from(h.clone()).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            })
            .collect();
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows: Vec<Row> = data
            .rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let cells: Vec<Cell> = row.iter().map(|c| Cell::from(c.clone())).collect();
                let style = if i % 2 == 0 {
                    Style::default()
                } else {
                    Style::default().bg(Color::Rgb(30, 30, 30))
                };
                Row::new(cells).style(style)
            })
            .collect();

        let widths: Vec<Constraint> = (0..col_count)
            .map(|_| Constraint::Min(col_width as u16))
            .collect();

        let table = Table::new(rows, widths)
            .header(header)
            .block(block.title(format!(" {} ({} rows) ", data.file_name, data.rows.len())))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut app.table_state);
    } else {
        let paragraph = Paragraph::new("Select a JSON or CSV file to view its contents\n\nUse j/k or arrow keys to navigate\nPress Enter to open a file\nPress Tab to switch panels\nPress 1/2 to switch tabs\nPress q to quit")
            .block(block.title(" No Data "))
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }
}

/// Render the raw view
fn render_raw_view(frame: &mut Frame, app: &App, area: Rect, block: Block) {
    let content = if let Some(data) = &app.table_data {
        data.raw_content.clone()
    } else {
        "No file loaded".to_string()
    };

    let paragraph = Paragraph::new(content)
        .block(block.title(" Raw Content "))
        .style(Style::default())
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Render a modal dialog
fn render_modal(frame: &mut Frame, modal: &ModalDialog) {
    let area = centered_rect(50, 30, frame.area());

    // Clear the area behind the modal
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(format!(" {} ", modal.title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Black));

    let text = format!("{}\n\nPress Enter or Esc to close", modal.message);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Setup the terminal
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Failed to create terminal")?;
    Ok(terminal)
}

/// Restore the terminal to its original state
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;
    Ok(())
}

/// Run the application main loop
fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key.code);
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create app state
    let mut app = App::new()?;

    // Run the app
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    restore_terminal(&mut terminal)?;

    // Handle any errors from run_app
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_tab_index() {
        assert_eq!(ActiveTab::Table.index(), 0);
        assert_eq!(ActiveTab::Raw.index(), 1);
    }

    #[test]
    fn test_active_tab_next() {
        assert_eq!(ActiveTab::Table.next(), ActiveTab::Raw);
        assert_eq!(ActiveTab::Raw.next(), ActiveTab::Table);
    }

    #[test]
    fn test_json_to_table_array_of_objects() {
        let app = App {
            file_list: vec![],
            file_list_state: ListState::default(),
            selected_file: None,
            table_data: None,
            table_state: TableState::default(),
            active_tab: ActiveTab::Table,
            active_panel: ActivePanel::FileList,
            current_dir: PathBuf::from("/tmp"),
            modal: None,
            should_quit: false,
        };

        let json = serde_json::json!([
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]);

        let (headers, rows) = app.json_to_table(&json);
        assert_eq!(headers.len(), 2);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_json_to_table_object() {
        let app = App {
            file_list: vec![],
            file_list_state: ListState::default(),
            selected_file: None,
            table_data: None,
            table_state: TableState::default(),
            active_tab: ActiveTab::Table,
            active_panel: ActivePanel::FileList,
            current_dir: PathBuf::from("/tmp"),
            modal: None,
            should_quit: false,
        };

        let json = serde_json::json!({"key1": "value1", "key2": "value2"});

        let (headers, rows) = app.json_to_table(&json);
        assert_eq!(headers, vec!["Key", "Value"]);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_value_to_string() {
        let app = App {
            file_list: vec![],
            file_list_state: ListState::default(),
            selected_file: None,
            table_data: None,
            table_state: TableState::default(),
            active_tab: ActiveTab::Table,
            active_panel: ActivePanel::FileList,
            current_dir: PathBuf::from("/tmp"),
            modal: None,
            should_quit: false,
        };

        assert_eq!(app.value_to_string(None), "");
        assert_eq!(app.value_to_string(Some(&Value::Null)), "null");
        assert_eq!(app.value_to_string(Some(&Value::Bool(true))), "true");
        assert_eq!(app.value_to_string(Some(&serde_json::json!(42))), "42");
        assert_eq!(app.value_to_string(Some(&Value::String("hello".to_string()))), "hello");
    }

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 50);
        let centered = centered_rect(50, 50, area);

        // The centered rect should be roughly in the middle
        assert!(centered.x > 0);
        assert!(centered.y > 0);
        assert!(centered.width < area.width);
        assert!(centered.height < area.height);
    }
}
