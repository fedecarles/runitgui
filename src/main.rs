//! Runitgui - A terminal-based service manager for runit
//!
//! This application provides a TUI (Terminal User Interface) for managing runit services.
//! It allows users to view, start, stop, restart, enable, and disable services.
//!
//! # Features
//!
//! - View all available and enabled runit services
//! - Display service status (running, down, finish)
//! - Show enabled/disabled state for each service
//! - Start, stop, and restart services
//! - Enable and disable services (creates/removes symlinks)
//! - Keyboard-driven interface
//!
//! # Usage
//!
//! Run with root privileges for full functionality:
//!
//! ```bash
//! sudo cargo run --release
//! ```
//!
//! ## Keyboard Shortcuts
//!
//! - `↑` / `↓` - Navigate services
//! - `s` - Start selected service
//! - `x` - Stop selected service
//! - `r` - Restart selected service
//! - `e` - Enable selected service
//! - `d` - Disable selected service
//! - `q` - Quit application
//!
//! # Service Directories
//!
//! The application scans the following directories for services:
//! - `/run/runit/service` - Primary service directory
//! - `/etc/runit/runsvdir/default` - Default runsvdir services
//! - `/etc/runit/sv` - Service definitions
//! - `/etc/runit` - Additional service configurations
//!
//! # Requirements
//!
//! - Rust 1.70 or later
//! - runit installed on the system
//! - Terminal with mouse support (optional)

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
};
use ratatui::DefaultTerminal;

use crate::app::App;

mod app;
mod commands;
mod models;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    execute!(terminal.backend_mut(), EnableMouseCapture)?;

    let result = run(&mut terminal);

    execute!(terminal.backend_mut(), DisableMouseCapture).ok();
    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new()?;

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key)?;
            }
        }
    }
}
