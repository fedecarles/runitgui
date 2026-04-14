//! Runit command wrapper for service management
//!
//! This module provides the [`Runit`] struct which wraps the `sv` command
//! to interact with the runit service supervisor.

use crate::models::service::Service;
use anyhow::Result;
use std::path::Path;

const DEFAULT_SERVICE_DIR: &str = "/run/runit/service";
const DEFAULT_RUNIT_DIR: &str = "/etc/runit";
const RUNSVDIR_DEFAULT: &str = "/etc/runit/runsvdir/default";
const RUNSVDIR_SV_DIR: &str = "/etc/runit/sv";

/// Wrapper for runit `sv` commands
///
/// Provides methods to query and control services managed by runit.
/// Requires appropriate permissions to execute `sv` commands.
pub struct Runit {
    service_dir: String,
    runit_dir: String,
    runsvdir_dir: String,
    runsvdir_sv_dir: String,
}

impl Runit {
    /// Creates a new Runit instance with default directories
    pub fn new() -> Self {
        Self {
            service_dir: DEFAULT_SERVICE_DIR.to_string(),
            runit_dir: DEFAULT_RUNIT_DIR.to_string(),
            runsvdir_dir: RUNSVDIR_DEFAULT.to_string(),
            runsvdir_sv_dir: RUNSVDIR_SV_DIR.to_string(),
        }
    }

    /// Lists all available and enabled services
    ///
    /// Combines enabled services from service directories with
    /// available services from runit configuration directories.
    ///
    /// # Returns
    ///
    /// A vector of [`Service`] structs representing all discovered services
    pub fn list_services(&self) -> Result<Vec<Service>> {
        let mut services = Vec::new();

        if let Ok(enabled) = self.list_enabled_services() {
            services.extend(enabled);
        }

        if let Ok(available) = self.list_available_services() {
            for service in available {
                if !services.iter().any(|s| s.name == service.name) {
                    services.push(service);
                }
            }
        }

        services.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(services)
    }

    /// Lists enabled services from service directories
    fn list_enabled_services(&self) -> Result<Vec<Service>> {
        let mut services = Vec::new();

        for dir in [&self.service_dir, &self.runsvdir_dir] {
            let service_path = Path::new(dir);

            if !service_path.exists() {
                continue;
            }

            for entry in std::fs::read_dir(service_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() || path.is_symlink() {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    if name == "log" || name == "current" {
                        continue;
                    }

                    if services.iter().any(|s: &Service| s.name == name) {
                        continue;
                    }

                    let mut service =
                        Service::new(name.clone(), path.to_string_lossy().to_string());
                    service.enabled = true;
                    self.update_service_status(&mut service)?;
                    services.push(service);
                }
            }
        }

        Ok(services)
    }

    /// Lists available (disabled) services from runit directories
    fn list_available_services(&self) -> Result<Vec<Service>> {
        let mut services = Vec::new();

        for dir in [&self.runit_dir, &self.runsvdir_sv_dir] {
            let runit_path = Path::new(dir);

            if !runit_path.exists() {
                continue;
            }

            for entry in std::fs::read_dir(runit_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    if name == "template" || name == "services" || name == "runsvdir" {
                        continue;
                    }

                    let run_script = path.join("run");
                    if run_script.exists() {
                        let mut service = Service::new(name, path.to_string_lossy().to_string());
                        service.enabled = false;
                        self.update_service_status(&mut service)?;
                        services.push(service);
                    }
                }
            }
        }

        Ok(services)
    }

    /// Updates service status by running `sv status`
    fn update_service_status(&self, service: &mut Service) -> Result<()> {
        let output = std::process::Command::new("sv")
            .arg("status")
            .arg(&service.name)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let combined = format!("{}{}", stdout, stderr);

                if combined.contains("run:") {
                    service.status = "run".to_string();
                    if let Some(pid) = self.extract_pid(&combined) {
                        service.pid = Some(pid);
                    }
                } else if combined.contains("down:") {
                    service.status = "down".to_string();
                    service.pid = None;
                } else if combined.contains("finish:") {
                    service.status = "finish".to_string();
                    service.pid = None;
                } else if combined.contains("warning:") && output.status.success() {
                    service.status = "run".to_string();
                    service.pid = None;
                } else if combined.contains("warning:") {
                    service.status = "down".to_string();
                    service.pid = None;
                } else {
                    service.status = "unknown".to_string();
                    service.pid = None;
                }
            }
            Err(_) => {
                service.status = "unknown".to_string();
                service.pid = None;
            }
        }

        Ok(())
    }

    /// Extracts the PID from sv status output
    fn extract_pid(&self, status_output: &str) -> Option<String> {
        if let Some(start) = status_output.find("(pid ") {
            let rest = &status_output[start + 5..];
            if let Some(end) = rest.find(')') {
                return Some(rest[..end].to_string());
            }
        }
        None
    }

    /// Starts a service using `sv start`
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be started
    pub fn start(&self, service: &mut Service) -> Result<()> {
        let output = std::process::Command::new("sv")
            .arg("start")
            .arg(&service.name)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to start service: {}", stderr);
        }

        self.update_service_status(service)?;
        Ok(())
    }

    /// Stops a service using `sv stop`
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be stopped
    pub fn stop(&self, service: &mut Service) -> Result<()> {
        let output = std::process::Command::new("sv")
            .arg("stop")
            .arg(&service.name)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to stop service: {}", stderr);
        }

        self.update_service_status(service)?;
        Ok(())
    }

    /// Restarts a service using `sv restart`
    ///
    /// # Errors
    ///
    /// Returns an error if the service cannot be restarted
    pub fn restart(&self, service: &mut Service) -> Result<()> {
        let output = std::process::Command::new("sv")
            .arg("restart")
            .arg(&service.name)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to restart service: {}", stderr);
        }

        self.update_service_status(service)?;
        Ok(())
    }

    /// Enables a service by creating a symlink in the service directory
    ///
    /// # Errors
    ///
    /// Returns an error if the symlink cannot be created
    pub fn enable(&self, service: &mut Service) -> Result<()> {
        let link_path = Path::new(&self.service_dir).join(&service.name);
        let target_path = Path::new(&service.path);

        if link_path.exists() {
            return Ok(());
        }

        std::os::unix::fs::symlink(target_path, &link_path)?;

        service.enabled = true;
        Ok(())
    }

    /// Disables a service by removing the symlink from the service directory
    ///
    /// # Errors
    ///
    /// Returns an error if the symlink cannot be removed
    pub fn disable(&self, service: &mut Service) -> Result<()> {
        let link_path = Path::new(&self.service_dir).join(&service.name);

        if link_path.exists() {
            std::fs::remove_file(&link_path)?;
        }

        service.enabled = false;
        Ok(())
    }
}
