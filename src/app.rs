//! Application state and logic for runitgui
//!
//! This module contains the main application state and handles user input
//! for managing runit services through the TUI.

use crate::commands::runit::Runit;
use crate::models::service::Service;
use anyhow::Result;
use crossterm::event::KeyEvent;

/// Main application state holding services and UI state
pub struct App {
    services: Vec<Service>,
    selected_index: usize,
    message: String,
    runit: Runit,
}

impl App {
    /// Creates a new application instance
    ///
    /// # Errors
    ///
    /// Returns an error if service discovery fails
    pub fn new() -> Result<Self> {
        let runit = Runit::new();
        let services = runit.list_services()?;

        Ok(Self {
            services,
            selected_index: 0,
            message: "Ready. Use arrow keys to navigate, keys below to act.".to_string(),
            runit,
        })
    }

    /// Handles keyboard input from the user
    ///
    /// # Arguments
    ///
    /// * `key` - The key event from crossterm
    ///
    /// # Errors
    ///
    /// Returns an error if service operations fail
    pub fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            crossterm::event::KeyCode::Char(c) => match c {
                's' => {
                    if let Err(e) = self.start_selected() {
                        self.message = format!("Error: {}", e);
                    }
                }
                'x' => {
                    if let Err(e) = self.stop_selected() {
                        self.message = format!("Error: {}", e);
                    }
                }
                'r' => {
                    if let Err(e) = self.restart_selected() {
                        self.message = format!("Error: {}", e);
                    }
                }
                'e' => {
                    if let Err(e) = self.enable_selected() {
                        self.message = format!("Error: {}", e);
                    }
                }
                'd' => {
                    if let Err(e) = self.disable_selected() {
                        self.message = format!("Error: {}", e);
                    }
                }
                'q' => {
                    std::process::exit(0);
                }
                _ => {}
            },
            crossterm::event::KeyCode::Up => {
                if !self.services.is_empty() {
                    self.selected_index = self.selected_index.saturating_sub(1);
                }
            }
            crossterm::event::KeyCode::Down => {
                if !self.services.is_empty() && self.selected_index < self.services.len() - 1 {
                    self.selected_index += 1;
                }
            }
            _ => {}
        }

        if let Err(e) = self.refresh_services() {
            self.message = format!("Error refreshing: {}", e);
        }
        Ok(())
    }

    /// Starts the currently selected service
    fn start_selected(&mut self) -> Result<()> {
        if let Some(service) = self.services.get_mut(self.selected_index) {
            self.runit.start(service)?;
            self.message = format!("Started service: {}", service.name);
        }
        Ok(())
    }

    /// Stops the currently selected service
    fn stop_selected(&mut self) -> Result<()> {
        if let Some(service) = self.services.get_mut(self.selected_index) {
            self.runit.stop(service)?;
            self.message = format!("Stopped service: {}", service.name);
        }
        Ok(())
    }

    /// Restarts the currently selected service
    fn restart_selected(&mut self) -> Result<()> {
        if let Some(service) = self.services.get_mut(self.selected_index) {
            self.runit.restart(service)?;
            self.message = format!("Restarted service: {}", service.name);
        }
        Ok(())
    }

    /// Enables and starts the currently selected service
    ///
    /// Creates a symlink in the service directory and starts the service
    fn enable_selected(&mut self) -> Result<()> {
        if let Some(service) = self.services.get_mut(self.selected_index) {
            self.runit.enable(service)?;
            service.enabled = true;
            self.runit.start(service)?;
            self.message = format!("Enabled and started service: {}", service.name);
        }
        Ok(())
    }

    /// Disables and stops the currently selected service
    ///
    /// Stops the service and removes the symlink from the service directory
    fn disable_selected(&mut self) -> Result<()> {
        if let Some(service) = self.services.get_mut(self.selected_index) {
            self.runit.stop(service)?;
            self.runit.disable(service)?;
            service.enabled = false;
            self.message = format!("Disabled and stopped service: {}", service.name);
        }
        Ok(())
    }

    /// Refreshes the list of services from the system
    fn refresh_services(&mut self) -> Result<()> {
        self.services = self.runit.list_services()?;
        if self.selected_index >= self.services.len() && !self.services.is_empty() {
            self.selected_index = self.services.len() - 1;
        }
        Ok(())
    }

    /// Returns a reference to the list of services
    pub fn services(&self) -> &[Service] {
        &self.services
    }

    /// Returns the currently selected service index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Returns the current status message
    pub fn message(&self) -> &str {
        &self.message
    }
}
