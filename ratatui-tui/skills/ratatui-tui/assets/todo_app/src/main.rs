use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io;

/// Represents a single todo item
#[derive(Clone)]
struct TodoItem {
    text: String,
    completed: bool,
}

impl TodoItem {
    fn new(text: String) -> Self {
        Self {
            text,
            completed: false,
        }
    }

    fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

/// Type-safe actions for event handling
enum Action {
    Add,
    Delete,
    Toggle,
    Input(char),
    Backspace,
    MoveUp,
    MoveDown,
    Quit,
    None,
}

/// Application state
struct App {
    todos: Vec<TodoItem>,
    input: String,
    input_mode: bool,
    list_state: ListState,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            todos: vec![
                TodoItem::new("Learn Rust".to_string()),
                TodoItem::new("Build a TUI app".to_string()),
                TodoItem::new("Master Ratatui".to_string()),
            ],
            input: String::new(),
            input_mode: false,
            list_state: ListState::default(),
        };
        if !app.todos.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }

    fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    fn move_up(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn move_down(&mut self) {
        if self.todos.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.todos.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn add_todo(&mut self) {
        if !self.input.is_empty() {
            self.todos.push(TodoItem::new(self.input.clone()));
            self.input.clear();
            if self.todos.len() == 1 {
                self.list_state.select(Some(0));
            }
        }
        self.input_mode = false;
    }

    fn delete_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.todos.len() {
                self.todos.remove(i);
                if self.todos.is_empty() {
                    self.list_state.select(None);
                } else if i >= self.todos.len() {
                    self.list_state.select(Some(self.todos.len() - 1));
                }
            }
        }
    }

    fn toggle_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i < self.todos.len() {
                self.todos[i].toggle();
            }
        }
    }

    /// Map key events to actions
    fn handle_key(&self, key: KeyCode) -> Action {
        if self.input_mode {
            match key {
                KeyCode::Enter => Action::Add,
                KeyCode::Esc => Action::Quit,
                KeyCode::Backspace => Action::Backspace,
                KeyCode::Char(c) => Action::Input(c),
                _ => Action::None,
            }
        } else {
            match key {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Char('a') => Action::Add,
                KeyCode::Char('d') => Action::Delete,
                KeyCode::Char(' ') => Action::Toggle,
                KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
                KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
                _ => Action::None,
            }
        }
    }

    /// Process an action and update state
    fn process_action(&mut self, action: Action) -> bool {
        match action {
            Action::Quit => {
                if self.input_mode {
                    self.input_mode = false;
                    self.input.clear();
                    false
                } else {
                    true
                }
            }
            Action::Add => {
                if self.input_mode {
                    self.add_todo();
                } else {
                    self.input_mode = true;
                }
                false
            }
            Action::Delete => {
                self.delete_selected();
                false
            }
            Action::Toggle => {
                self.toggle_selected();
                false
            }
            Action::Input(c) => {
                self.input.push(c);
                false
            }
            Action::Backspace => {
                self.input.pop();
                false
            }
            Action::MoveUp => {
                self.move_up();
                false
            }
            Action::MoveDown => {
                self.move_down();
                false
            }
            Action::None => false,
        }
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main event loop
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                let action = app.handle_key(key.code);
                if app.process_action(action) {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Input
            Constraint::Min(5),    // Todo list
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    // Input widget
    let input_style = if app.input_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.input.as_str())
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(input_style)
                .title(if app.input_mode {
                    "New Todo (Enter to add, Esc to cancel)"
                } else {
                    "Press 'a' to add new todo"
                }),
        );
    f.render_widget(input, chunks[0]);

    // Show cursor in input mode
    if app.input_mode {
        f.set_cursor_position((chunks[0].x + app.input.len() as u16 + 1, chunks[0].y + 1));
    }

    // Todo list widget
    let items: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let checkbox = if todo.completed { "[x]" } else { "[ ]" };
            let style = if todo.completed {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(Color::White)
            };

            let selected_marker = if app.selected_index() == Some(i) {
                "> "
            } else {
                "  "
            };

            ListItem::new(Line::from(vec![
                Span::styled(selected_marker, Style::default().fg(Color::Cyan)),
                Span::styled(format!("{} ", checkbox), Style::default().fg(Color::Yellow)),
                Span::styled(&todo.text, style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(format!(" Todos ({}) ", app.todos.len())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // Help widget
    let help_text = if app.input_mode {
        "Enter: Add todo | Esc: Cancel"
    } else {
        "a: Add | d: Delete | Space: Toggle | j/k or Up/Down: Navigate | q: Quit"
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Help "),
        );
    f.render_widget(help, chunks[2]);
}
