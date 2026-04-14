//! User interface rendering for runitgui
//!
//! This module handles all TUI rendering using the ratatui library.
//! It provides the visual layout including header, service table, and footer.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::App;

/// Renders the complete UI frame
///
/// Splits the terminal area into three sections:
/// - Header (3 lines) - Application title
/// - Service list (flexible) - Main content area
/// - Footer (3 lines) - Help text and status messages
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, layout[0]);
    render_service_list(frame, layout[1], app);
    render_footer(frame, layout[2], app);
}

/// Renders the header section with application title
fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let title = Paragraph::new("runitgui - Service Manager")
        .block(block)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );

    frame.render_widget(title, area);
}

/// Renders the service list table
///
/// Displays all available services with columns:
/// - Service name
/// - Status (color-coded: green=run, red=down, yellow=finish)
/// - Enabled indicator (● = enabled, ○ = disabled)
/// - Process ID
fn render_service_list(frame: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let header_row = Row::new(vec![
        Cell::from("Service"),
        Cell::from("Status"),
        Cell::from("Enabled"),
        Cell::from("PID"),
    ])
    .style(
        Style::default()
            .fg(Color::White)
            .add_modifier(ratatui::style::Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .services()
        .iter()
        .enumerate()
        .map(|(i, service)| {
            let style = if i == app.selected_index() {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };

            let status_color = match service.status.as_str() {
                "run" => Color::Green,
                "down" => Color::Red,
                "finish" => Color::Yellow,
                _ => Color::Gray,
            };

            let enabled_indicator = if service.enabled { "●" } else { "○" };

            Row::new(vec![
                Cell::from(service.name.clone()),
                Cell::from(
                    Span::raw(service.status.clone()).style(Style::default().fg(status_color)),
                ),
                Cell::from(enabled_indicator),
                Cell::from(service.pid.clone().unwrap_or_else(|| "-".to_string())),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
        ],
    )
    .header(header_row)
    .block(Block::default().borders(Borders::ALL).title("Services"))
    .column_spacing(1);

    frame.render_widget(table, area);
}

/// Renders the footer with keyboard shortcuts and status message
fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    let help_text = Line::from(vec![
        Span::raw("[s]start "),
        Span::raw("[x]stop "),
        Span::raw("[r]restart "),
        Span::raw("[e]enable "),
        Span::raw("[d]disable "),
        Span::raw("[q]quit"),
    ]);

    let message_text = Line::from(Span::raw(app.message()));

    let footer = Paragraph::new(vec![help_text, message_text]).block(block);

    frame.render_widget(footer, area);
}
