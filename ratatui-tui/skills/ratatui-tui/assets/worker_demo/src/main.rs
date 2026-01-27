//! Worker Demo - Ratatui TUI Worker/Async Example
//!
//! Demonstrates:
//! - tokio::spawn for background tasks
//! - mpsc channels for worker communication
//! - tokio_util::sync::CancellationToken for task cancellation
//! - Gauge widget for progress display
//! - Log-like output area
//!
//! Controls:
//! - s: Start worker
//! - c: Cancel worker
//! - q: Quit application

use std::io;
use std::time::{Duration, Instant};

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
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame, Terminal,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Messages sent from worker to UI
#[derive(Debug, Clone)]
enum WorkerMessage {
    /// Progress update (0.0 to 1.0)
    Progress(f64),
    /// Log message to display
    Log(String),
    /// Worker completed successfully
    Complete,
    /// Worker encountered an error
    Error(String),
}

/// Application state
struct App {
    /// Current progress (0.0 to 1.0)
    progress: f64,
    /// Log messages
    logs: Vec<String>,
    /// Whether a worker is currently running
    worker_running: bool,
    /// Cancellation token for the current worker
    cancel_token: Option<CancellationToken>,
    /// Channel sender for worker messages (kept to clone for new workers)
    tx: mpsc::Sender<WorkerMessage>,
    /// Channel receiver for worker messages
    rx: mpsc::Receiver<WorkerMessage>,
    /// Start time of current worker
    start_time: Option<Instant>,
    /// Number of files processed
    processed_count: u32,
    /// Total files to process
    total_files: u32,
    /// Current status message
    status: String,
}

impl App {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            progress: 0.0,
            logs: vec![
                "Worker Demo Started".to_string(),
                "Press 's' to start, 'c' to cancel, 'q' to quit".to_string(),
            ],
            worker_running: false,
            cancel_token: None,
            tx,
            rx,
            start_time: None,
            processed_count: 0,
            total_files: 100,
            status: "Idle".to_string(),
        }
    }

    /// Add a log message
    fn log(&mut self, msg: impl Into<String>) {
        self.logs.push(msg.into());
        // Keep only last 100 messages
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    /// Start the background worker
    fn start_worker(&mut self) {
        if self.worker_running {
            self.log("Worker already running!");
            return;
        }

        // Reset state
        self.progress = 0.0;
        self.processed_count = 0;
        self.worker_running = true;
        self.start_time = Some(Instant::now());
        self.status = "Starting...".to_string();

        // Create cancellation token
        let cancel_token = CancellationToken::new();
        self.cancel_token = Some(cancel_token.clone());

        // Clone sender for the worker
        let tx = self.tx.clone();
        let total_files = self.total_files;

        self.log("Starting file processing worker...");

        // Spawn the background task
        tokio::spawn(async move {
            process_files(tx, cancel_token, total_files).await;
        });
    }

    /// Cancel the running worker
    fn cancel_worker(&mut self) {
        if self.cancel_token.is_some() && self.worker_running {
            self.log("Cancelling worker...");
            if let Some(token) = &self.cancel_token {
                token.cancel();
            }
        } else if self.cancel_token.is_none() {
            self.log("No worker to cancel");
        }
    }

    /// Process incoming worker messages
    fn process_messages(&mut self) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                WorkerMessage::Progress(p) => {
                    self.progress = p;
                    self.processed_count = (p * self.total_files as f64) as u32;
                    self.status = format!(
                        "Processing file_{:04}.txt",
                        self.processed_count
                    );
                }
                WorkerMessage::Log(text) => {
                    self.log(text);
                }
                WorkerMessage::Complete => {
                    self.worker_running = false;
                    self.cancel_token = None;
                    self.status = "Complete".to_string();
                    self.log("Worker completed successfully!");
                }
                WorkerMessage::Error(err) => {
                    self.worker_running = false;
                    self.cancel_token = None;
                    self.status = format!("Error: {}", err);
                    self.log(format!("Worker error: {}", err));
                }
            }
        }
    }

    /// Get elapsed time as a formatted string
    fn elapsed_time(&self) -> String {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            format!("{:.1}s", elapsed)
        } else {
            "0.0s".to_string()
        }
    }

    /// Get processing speed
    fn speed(&self) -> String {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let speed = self.processed_count as f64 / elapsed;
                return format!("{:.1} files/sec", speed);
            }
        }
        "0.0 files/sec".to_string()
    }
}

/// Background worker task that processes files
async fn process_files(
    tx: mpsc::Sender<WorkerMessage>,
    cancel_token: CancellationToken,
    total_files: u32,
) {
    let _ = tx.send(WorkerMessage::Log("Worker: Running".to_string())).await;

    for i in 0..total_files {
        // Check for cancellation
        if cancel_token.is_cancelled() {
            let _ = tx.send(WorkerMessage::Log("Processing cancelled!".to_string())).await;
            let _ = tx.send(WorkerMessage::Error("Cancelled by user".to_string())).await;
            return;
        }

        // Simulate file processing work
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Calculate progress
        let progress = (i + 1) as f64 / total_files as f64;
        let _ = tx.send(WorkerMessage::Progress(progress)).await;

        // Log every 10th file
        if (i + 1) % 10 == 0 {
            let _ = tx
                .send(WorkerMessage::Log(format!(
                    "Processed {}/{} files",
                    i + 1,
                    total_files
                )))
                .await;
        }
    }

    let _ = tx.send(WorkerMessage::Log("All files processed successfully!".to_string())).await;
    let _ = tx.send(WorkerMessage::Complete).await;
}

/// Draw the UI
fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Controls
            Constraint::Length(5), // Progress
            Constraint::Length(6), // Stats
            Constraint::Min(10),   // Logs
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Worker Demo - Ratatui Async Example")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Controls
    let controls = Paragraph::new(Line::from(vec![
        Span::styled("[S]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" Start  "),
        Span::styled("[C]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" Cancel  "),
        Span::styled("[Q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw(" Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls, chunks[1]);

    // Progress gauge
    let progress_label = format!("{:.1}%", app.progress * 100.0);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(
            Style::default()
                .fg(if app.worker_running { Color::Green } else { Color::Blue })
                .bg(Color::DarkGray),
        )
        .percent((app.progress * 100.0) as u16)
        .label(progress_label);
    f.render_widget(gauge, chunks[2]);

    // Stats
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                &app.status,
                Style::default().fg(if app.worker_running {
                    Color::Yellow
                } else if app.status == "Complete" {
                    Color::Green
                } else {
                    Color::White
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("Files: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{} / {}", app.processed_count, app.total_files)),
        ]),
        Line::from(vec![
            Span::styled("Time: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.elapsed_time()),
        ]),
        Line::from(vec![
            Span::styled("Speed: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(app.speed()),
        ]),
    ];
    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    f.render_widget(stats, chunks[3]);

    // Log area
    let log_items: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(20)
        .rev()
        .map(|log| {
            let style = if log.contains("error") || log.contains("Error") || log.contains("cancelled") {
                Style::default().fg(Color::Red)
            } else if log.contains("success") || log.contains("Complete") || log.contains("completed") {
                Style::default().fg(Color::Green)
            } else if log.contains("Starting") || log.contains("Running") {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::Gray)
            };
            ListItem::new(Line::from(Span::styled(log.as_str(), style)))
        })
        .collect();
    let logs = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title("Log"));
    f.render_widget(logs, chunks[4]);

    // Footer
    let footer = Paragraph::new("Worker running: ".to_string() + if app.worker_running { "Yes" } else { "No" })
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[5]);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main loop
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        // Draw UI
        terminal.draw(|f| ui(f, &app))?;

        // Process worker messages
        app.process_messages();

        // Handle input with timeout
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('s') => app.start_worker(),
                        KeyCode::Char('c') => app.cancel_worker(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
