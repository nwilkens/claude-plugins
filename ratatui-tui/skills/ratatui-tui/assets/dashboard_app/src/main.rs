//! System Monitoring Dashboard
//!
//! A TUI dashboard demonstrating Ratatui widgets for system monitoring:
//! - Gauge widgets for CPU and Memory percentages
//! - Sparkline for historical CPU data visualization
//! - Grid-like layout with nested constraints
//! - Periodic updates with tokio timer
//! - sysinfo crate for system metrics

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame, Terminal,
};
use sysinfo::System;
use tokio::time::interval;

/// Application state holding system metrics and history
struct App {
    /// Historical CPU usage values for sparkline (last 60 samples)
    cpu_history: Vec<u64>,
    /// Current CPU usage percentage
    cpu_percent: f64,
    /// Current memory usage percentage
    memory_percent: f64,
    /// Total memory in bytes
    memory_total: u64,
    /// Used memory in bytes
    memory_used: u64,
    /// Number of running processes
    process_count: usize,
    /// System info instance
    sys: System,
    /// Flag to signal app should quit
    should_quit: bool,
}

impl App {
    /// Create a new App instance with initialized system info
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_percent = sys.global_cpu_usage() as f64;
        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();
        let memory_percent = if memory_total > 0 {
            (memory_used as f64 / memory_total as f64) * 100.0
        } else {
            0.0
        };
        let process_count = sys.processes().len();

        Self {
            cpu_history: vec![cpu_percent as u64],
            cpu_percent,
            memory_percent,
            memory_total,
            memory_used,
            process_count,
            sys,
            should_quit: false,
        }
    }

    /// Update system metrics
    fn update(&mut self) {
        self.sys.refresh_all();

        // Update CPU
        self.cpu_percent = self.sys.global_cpu_usage() as f64;
        self.cpu_history.push(self.cpu_percent as u64);

        // Keep only last 60 samples for sparkline
        if self.cpu_history.len() > 60 {
            self.cpu_history.remove(0);
        }

        // Update memory
        self.memory_total = self.sys.total_memory();
        self.memory_used = self.sys.used_memory();
        self.memory_percent = if self.memory_total > 0 {
            (self.memory_used as f64 / self.memory_total as f64) * 100.0
        } else {
            0.0
        };

        // Update process count
        self.process_count = self.sys.processes().len();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Create update interval (1 second)
    let mut update_interval = interval(Duration::from_secs(1));

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| ui(f, &app))?;

        // Handle events with timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }

        // Check if it's time to update metrics
        tokio::select! {
            _ = update_interval.tick() => {
                app.update();
            }
            else => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

/// Render the dashboard UI
fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Main layout: Header, Content, Footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Render header
    render_header(f, main_chunks[0]);

    // Content area: Metrics cards on left, Charts on right
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Metrics cards
            Constraint::Percentage(60), // Charts
        ])
        .split(main_chunks[1]);

    // Metrics cards layout (vertical stack)
    let metrics_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(content_chunks[0]);

    // Render metric cards
    render_cpu_gauge(f, metrics_chunks[0], app);
    render_memory_gauge(f, metrics_chunks[1], app);
    render_process_count(f, metrics_chunks[2], app);

    // Charts area
    render_cpu_sparkline(f, content_chunks[1], app);

    // Render footer
    render_footer(f, main_chunks[2]);
}

/// Render the header with title
fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " System Monitor ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("- Ratatui Dashboard Demo"),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Dashboard "),
    );
    f.render_widget(header, area);
}

/// Render the footer with help text
fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" Press ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" or ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(" to quit ", Style::default().fg(Color::DarkGray)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(footer, area);
}

/// Render CPU usage gauge
fn render_cpu_gauge(f: &mut Frame, area: Rect, app: &App) {
    let cpu_color = if app.cpu_percent > 80.0 {
        Color::Red
    } else if app.cpu_percent > 50.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(cpu_color))
                .title(" CPU Usage "),
        )
        .gauge_style(Style::default().fg(cpu_color).bg(Color::Black))
        .percent(app.cpu_percent.min(100.0) as u16)
        .label(format!("{:.1}%", app.cpu_percent));
    f.render_widget(gauge, area);
}

/// Render memory usage gauge
fn render_memory_gauge(f: &mut Frame, area: Rect, app: &App) {
    let mem_color = if app.memory_percent > 80.0 {
        Color::Red
    } else if app.memory_percent > 50.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    // Convert bytes to human-readable format
    let used_gb = app.memory_used as f64 / (1024.0 * 1024.0 * 1024.0);
    let total_gb = app.memory_total as f64 / (1024.0 * 1024.0 * 1024.0);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(mem_color))
                .title(" Memory Usage "),
        )
        .gauge_style(Style::default().fg(mem_color).bg(Color::Black))
        .percent(app.memory_percent.min(100.0) as u16)
        .label(format!("{:.1} GB / {:.1} GB ({:.1}%)", used_gb, total_gb, app.memory_percent));
    f.render_widget(gauge, area);
}

/// Render process count widget
fn render_process_count(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from(vec![
            Span::styled(
                format!("{}", app.process_count),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Running Processes", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .title(" Processes "),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

/// Render CPU history sparkline
fn render_cpu_sparkline(f: &mut Frame, area: Rect, app: &App) {
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" CPU History (60s) "),
        )
        .data(&app.cpu_history)
        .max(100)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(sparkline, area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(!app.cpu_history.is_empty());
        assert!(app.cpu_percent >= 0.0);
        assert!(app.memory_percent >= 0.0 && app.memory_percent <= 100.0);
        assert!(app.process_count > 0);
    }

    #[test]
    fn test_app_update() {
        let mut app = App::new();
        let initial_history_len = app.cpu_history.len();
        app.update();
        assert_eq!(app.cpu_history.len(), initial_history_len + 1);
    }

    #[test]
    fn test_cpu_history_limit() {
        let mut app = App::new();
        // Fill history beyond limit
        for _ in 0..70 {
            app.update();
        }
        assert!(app.cpu_history.len() <= 60);
    }
}
